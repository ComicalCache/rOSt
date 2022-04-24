use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};

use crate::serial_println;

pub fn print_memory_map(memory_map: &MemoryRegions) {
    serial_println!("[    ---MEMORY MAP---    ]");
    memory_map.iter().for_each(|region| {
        let mut size = region.end - region.start;
        let mut size_format = "B";
        if size >= 2 * 1024 {
            if size < 2 * 1024 * 1024 {
                size /= 1024;
                size_format = "KiB";
            } else if size < 2 * 1024 * 1024 * 1024 {
                size /= 1024 * 1024;
                size_format = "MiB";
            } else {
                size /= 1024 * 1024 * 1024;
                size_format = "GiB";
            }
        }
        serial_println!(
            "{:14}- {:>4}{:>3}  ({:X})",
            decode_memory_kind(region.kind),
            size,
            size_format,
            region.start
        );
    });
}

fn decode_memory_kind(kind: MemoryRegionKind) -> &'static str {
    match kind {
        MemoryRegionKind::Usable => "usable",
        MemoryRegionKind::Bootloader => "bootloader",
        MemoryRegionKind::UnknownBios(kind) => match kind {
            1 => "usable BIOS",
            2 => "Reserved BIOS",
            3 => "ACPI reclaimable",
            4 => "ACPI NVS",
            5 => "Bad memory",
            _ => "unknown BIOS",
        },
        MemoryRegionKind::UnknownUefi(kind) => match kind {
            0 => "EfiReservedMemoryType",
            1 => "EfiLoaderCode",
            2 => "EfiLoaderData",
            3 => "EfiBootServicesCode",
            4 => "EfiBootServicesData",
            5 => "EfiRuntimeServiceCode",
            6 => "EfiRuntimeServicesData",
            7 => "EfiConventionalMemory",
            8 => "EfiUnusableMemory",
            9 => "EfiACPIReclaimMemory",
            10 => "EfiACPIMemoryNVS",
            11 => "EfiMemoryMappedIO",
            12 => "EfiMemoryMappedIOPort Space",
            13 => "EfiPalCode",
            14 => "EfiPersistentMemory",
            _ => "unknown UEFI",
        },
        _ => "unknown",
    }
}
