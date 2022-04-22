use alloc::vec::Vec;
use kernel::structures::port_extensions::{PortExtRead, PortExtWrite};
use x86_64::instructions::{
    interrupts::without_interrupts,
    port::{Port, PortReadOnly, PortWriteOnly},
};

use crate::{constants::ATACommands, partition::Partition};

use super::{
    constants::{error_register_flags, status_register_flags, ATAIdentifyError},
    descriptor::ATADescriptor,
};

#[allow(dead_code)]
pub struct ATABus {
    data_register_rw: Port<u16>,
    error_register_r: PortReadOnly<u8>,
    features_register_w: PortWriteOnly<u8>,
    sector_count_register_rw: Port<u8>,
    lba_low_register_rw: Port<u8>,
    lba_mid_register_rw: Port<u8>,
    lba_high_register_rw: Port<u8>,
    drive_head_register_rw: Port<u8>,
    status_register_r: PortReadOnly<u8>,
    command_register_w: PortWriteOnly<u8>,
    alternate_status_register_r: PortReadOnly<u8>,
    device_control_register_w: PortWriteOnly<u8>,
    drive_address_register_r: PortReadOnly<u8>,
}

impl ATABus {
    pub(crate) const fn new(base_port: u16) -> Self {
        ATABus {
            data_register_rw: Port::new(base_port + 0x00),
            error_register_r: PortReadOnly::new(base_port + 0x01),
            features_register_w: PortWriteOnly::new(base_port + 0x01),
            sector_count_register_rw: Port::new(base_port + 0x02),
            lba_low_register_rw: Port::new(base_port + 0x03),
            lba_mid_register_rw: Port::new(base_port + 0x04),
            lba_high_register_rw: Port::new(base_port + 0x05),
            drive_head_register_rw: Port::new(base_port + 0x06),
            status_register_r: PortReadOnly::new(base_port + 0x07),
            command_register_w: PortWriteOnly::new(base_port + 0x07),
            alternate_status_register_r: PortReadOnly::new(base_port + 0x206),
            device_control_register_w: PortWriteOnly::new(base_port + 0x206),
            drive_address_register_r: PortReadOnly::new(base_port + 0x207),
        }
    }

    pub unsafe fn connected(&mut self) -> bool {
        self.status_register_r.read() != 0xFF
    }

    pub unsafe fn wait_for(&mut self, flag: u8, should_be_on: bool) -> Result<(), u8> {
        let condition = if should_be_on { flag } else { 0 };
        loop {
            let status = self.status_register_r.read();
            if status & flag == condition {
                break;
            }
            if status & status_register_flags::ERR != 0 {
                let error = self.error_register_r.read();
                if error != 0 {
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    pub unsafe fn wait_400ns(&mut self) -> Result<(), u8> {
        for _ in 0..15 {
            let status = self.status_register_r.read();
            if status & status_register_flags::ERR != 0 {
                let error = self.error_register_r.read();
                if error != 0 {
                    return Err(error);
                }
            }
        }
        Ok(())
    }
}

impl ATABus {
    pub fn identify(&mut self, master: bool) -> Result<ATADescriptor, ATAIdentifyError> {
        unsafe fn handle_identify_error(bus: &mut ATABus, error: u8) -> ATAIdentifyError {
            if error & error_register_flags::ABRT == 0 {
                return ATAIdentifyError::DeviceIsATAPI;
            }
            let mid = bus.lba_mid_register_rw.read();
            let high = bus.lba_high_register_rw.read();

            match (mid, high) {
                (0x14, 0xEB) => ATAIdentifyError::DeviceIsATAPI,
                (0x3C, 0xC3) => ATAIdentifyError::DeviceIsSATA,
                (0, 0) => ATAIdentifyError::Unknown,
                (_, _) => ATAIdentifyError::DeviceIsNotATA,
            }
        }

        unsafe {
            if !self.connected() {
                return Err(ATAIdentifyError::BusNotConnected);
            }
            if let Err(_) = self.wait_for(status_register_flags::BSY, false) {
                return Err(ATAIdentifyError::Unknown);
            }
            without_interrupts(|| {
                self.drive_head_register_rw
                    .write(if master { 0xA0 } else { 0xB0 });
                self.device_control_register_w.write(0x00);
                if let Err(_) = self.wait_400ns() {
                    return Err(ATAIdentifyError::Unknown);
                }
                self.sector_count_register_rw.write(0x00);
                self.lba_low_register_rw.write(0x00);
                self.lba_mid_register_rw.write(0x00);
                self.lba_high_register_rw.write(0x00);
                self.command_register_w.write(ATACommands::Identify as u8);
                let status = self.status_register_r.read();
                if status == 0 {
                    return Err(ATAIdentifyError::NoDevice);
                }
                if let Err(error) = self.wait_for(status_register_flags::BSY, false) {
                    return Err(handle_identify_error(self, error));
                }
                if let Err(error) = self.wait_for(status_register_flags::DRQ, true) {
                    return Err(handle_identify_error(self, error));
                }
                let mut identify_buffer: [u16; 256] = [0; 256];
                self.data_register_rw.read_to_buffer(&mut identify_buffer);

                Ok(ATADescriptor::new(identify_buffer))
            })
        }
    }

    pub fn read_sector(&mut self, master: bool, lba: u64) -> Result<[u8; 512], u8> {
        if lba > u32::MAX.into() {
            todo!("LBA48 not supported");
        }
        unsafe {
            let slave = if master { 0xE0 } else { 0xF0 };
            self.drive_head_register_rw
                .write(slave | ((lba >> 24) & 0x0F) as u8);
            self.features_register_w.write(0x00);
            self.sector_count_register_rw.write(0x01);
            self.lba_low_register_rw.write(lba as u8);
            self.lba_mid_register_rw.write((lba >> 8) as u8);
            self.lba_high_register_rw.write((lba >> 16) as u8);
            self.command_register_w
                .write(ATACommands::ReadSectors as u8);
            self.wait_for(status_register_flags::BSY, false)?;
            self.wait_for(status_register_flags::DRQ, true)?;
            let mut buffer = [0u8; 512];
            self.data_register_rw.read_to_buffer(&mut buffer);
            self.wait_400ns()?;
            Ok(buffer)
        }
    }

    pub fn write_sector(&mut self, master: bool, lba: u64, buffer: &[u8; 512]) -> Result<(), u8> {
        if lba > u32::MAX.into() {
            todo!("LBA48 not supported");
        }
        unsafe {
            let slave = if master { 0xE0 } else { 0xF0 };
            self.drive_head_register_rw
                .write(slave | ((lba >> 24) & 0x0F) as u8);
            self.features_register_w.write(0x00);
            self.sector_count_register_rw.write(0x01);
            self.lba_low_register_rw.write(lba as u8);
            self.lba_mid_register_rw.write((lba >> 8) as u8);
            self.lba_high_register_rw.write((lba >> 16) as u8);
            self.command_register_w
                .write(ATACommands::WriteSectors as u8);
            self.wait_for(status_register_flags::BSY, false)?;
            self.wait_for(status_register_flags::DRQ, true)?;
            self.data_register_rw.write_from_buffer(buffer);
            self.wait_400ns()?;
            self.wait_for(status_register_flags::DRQ, false)?;
            self.wait_for(status_register_flags::BSY, false)?;
            self.command_register_w.write(ATACommands::CacheFlush as u8);
            self.wait_for(status_register_flags::BSY, false)?;
            Ok(())
        }
    }

    pub fn get_partitions(&mut self, master: bool) -> Result<Vec<Partition>, u8> {
        let mbr = self.read_sector(master, 0)?;
        Ok(Vec::from_iter(
            mbr[446..510].chunks(16).filter_map(Partition::from_bytes),
        ))
    }
}
