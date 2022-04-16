use super::vga_core::Interpolatable;

pub struct VGAPoint {
    pub x: usize,
    pub y: usize,
}
pub static ZERO: VGAPoint = VGAPoint { x: 0, y: 0 };

impl Interpolatable<u16, VGAPoint> for VGAPoint {
    fn interpolate(a: &VGAPoint, b: &VGAPoint, t: u16) -> VGAPoint {
        let t1 = (u16::MAX - t) as usize;
        let t2 = t as usize;
        VGAPoint {
            x: ((a.x * t1 + b.x * t2)/ 65535) as usize,
            y: ((a.y * t1 + b.y * t2)/ 65535) as usize,
        }
    }
}