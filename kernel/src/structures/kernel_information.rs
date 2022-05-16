use bootloader::{
    boot_info::{FrameBuffer, MemoryRegions, Optional},
    BootInfo,
};

use crate::{debug, memory::FullFrameAllocator};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct KernelInformation {
    pub bootloader_version: [u16; 3],
    pub physical_memory_offset: u64,
    pub framebuffer: Optional<KernelFrameBuffer>,
    pub memory_regions: &'static MemoryRegions,
    pub allocator: FullFrameAllocator,
    pub kernel_start: u64, // The start address of the kernel space in all processes
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct KernelFrameBuffer {
    pub width: usize,
    pub height: usize,
    pub format: PixelFormat,
    pub bytes_per_pixel: usize,
    pub stride: usize,
    pub buffer: *mut u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum PixelFormat {
    /// One byte red, then one byte green, then one byte blue.
    ///
    /// Length might be larger than 3, check [`bytes_per_pixel`][FrameBufferInfo::bytes_per_pixel]
    /// for this.
    RGB,
    /// One byte blue, then one byte green, then one byte red.
    ///
    /// Length might be larger than 3, check [`bytes_per_pixel`][FrameBufferInfo::bytes_per_pixel]
    /// for this.
    BGR,
    /// A single byte, representing the grayscale value.
    ///
    /// Length might be larger than 1, check [`bytes_per_pixel`][FrameBufferInfo::bytes_per_pixel]
    /// for this.
    U8,
}

impl KernelFrameBuffer {
    pub(crate) fn new(buffer: &FrameBuffer) -> KernelFrameBuffer {
        let info = buffer.info();
        KernelFrameBuffer {
            width: info.horizontal_resolution,
            height: info.vertical_resolution,
            format: match info.pixel_format {
                bootloader::boot_info::PixelFormat::RGB => PixelFormat::RGB,
                bootloader::boot_info::PixelFormat::BGR => PixelFormat::BGR,
                bootloader::boot_info::PixelFormat::U8 => PixelFormat::U8,
                _ => panic!("Unsupported pixel format: {:?}", info.pixel_format),
            },
            bytes_per_pixel: info.bytes_per_pixel,
            stride: info.stride,
            buffer: buffer.buffer().as_ptr() as *mut u8,
        }
    }
}

impl KernelInformation {
    pub(crate) fn new(boot_info: &'static BootInfo) -> KernelInformation {
        let bootloader_version = [
            boot_info.version_major,
            boot_info.version_minor,
            boot_info.version_patch,
        ];
        let framebuffer = match boot_info.framebuffer.as_ref() {
            Some(framebuffer) => Optional::Some(KernelFrameBuffer::new(framebuffer)),
            None => Optional::None,
        };
        debug::log("Obtained kernel info");
        KernelInformation {
            bootloader_version,
            physical_memory_offset: *boot_info
                .physical_memory_offset
                .as_ref()
                .expect("No physical memory mapping"),
            framebuffer,
            memory_regions: &boot_info.memory_regions,
            allocator: unsafe { FullFrameAllocator::init(&boot_info.memory_regions) },
            kernel_start: 0x007F_C000_0000u64,
        }
    }
}
