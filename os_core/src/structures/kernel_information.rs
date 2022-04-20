use bootloader::{
    boot_info::{FrameBuffer, PixelFormat},
    BootInfo,
};

#[derive(Clone, Copy)]
pub struct KernelInformation {
    pub bootloader_version: [u16; 3],
    pub framebuffer: Option<KernelFrameBuffer>,
}

#[derive(Clone, Copy)]
pub struct KernelFrameBuffer {
    pub width: usize,
    pub height: usize,
    pub format: PixelFormat,
    pub bytes_per_pixel: usize,
    pub stride: usize,
    pub buffer: *const [u8],
}

impl KernelFrameBuffer {
    pub(crate) fn new(mut buffer: FrameBuffer) -> KernelFrameBuffer {
        let info = buffer.info();
        KernelFrameBuffer {
            width: info.horizontal_resolution,
            height: info.vertical_resolution,
            format: info.pixel_format,
            bytes_per_pixel: info.bytes_per_pixel,
            stride: info.stride,
            buffer: buffer.buffer_mut(),
        }
    }
}

impl KernelInformation {
    pub(crate) fn new(boot_info: BootInfo) -> KernelInformation {
        let bootloader_version = [
            boot_info.version_major,
            boot_info.version_minor,
            boot_info.version_patch,
        ];
        let framebuffer_option = boot_info.framebuffer.into_option();
        match framebuffer_option {
            Some(framebuffer) => KernelInformation {
                bootloader_version,
                framebuffer: Some(KernelFrameBuffer::new(framebuffer)),
            },
            None => KernelInformation {
                bootloader_version,
                framebuffer: None,
            },
        }
    }
}
