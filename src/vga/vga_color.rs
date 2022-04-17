pub struct VGAColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}
pub static TRANSPARENT: VGAColor = VGAColor { red: 0, green: 0, blue: 0, alpha: 0 };
pub static WHITE: VGAColor = VGAColor { red: 255, green: 255, blue: 255, alpha: 255 };
pub static BLACK: VGAColor = VGAColor { red: 0, green: 0, blue: 0, alpha: 255 };
pub static RED: VGAColor = VGAColor { red: 255, green: 0, blue: 0, alpha: 255 };
pub static GREEN: VGAColor = VGAColor { red: 0, green: 255, blue: 0, alpha: 255 };
pub static BLUE: VGAColor = VGAColor { red: 0, green: 0, blue: 255, alpha: 255 };
pub static CLAY: VGAColor = VGAColor{ red: 128, green: 64, blue: 11, alpha: 255 };
pub static CHARLOTTE: VGAColor = VGAColor { red: 161, green: 232, blue: 223, alpha: 255 };

impl VGAColor {
  /// Multiplies the color by the given intensity value, returning a new color with parameters scaled back to 0-255.
  pub fn multiply(&self, intensity: u8) -> VGAColor {
    let intu16 = intensity as u16;
    VGAColor {
      red: ((self.red as u16 * intu16)/255) as u8,
      green: ((self.green as u16 * intu16)/255) as u8,
      blue: ((self.blue as u16 * intu16)/255) as u8,
      alpha: ((self.alpha as u16 * intu16)/255) as u8
    }
  }
  /// Multiplies only the alpha value by the opacity value, returning a new color with alpha scaled back to 0-255.
  pub fn multiply_alpha(&self, opacity: u8) -> VGAColor {
    let opu16 = opacity as u16;
    VGAColor {
      red: self.red,
      green: self.green,
      blue: self.blue,
      alpha: ((self.alpha as u16 * opu16)/255) as u8
    }
  }
  /// Returns the byte value for the grayscale version of the color, using the human light perception formula
  pub fn to_grayscale(&self) -> u8 {
    let red = self.red as u32;
    let green = self.green as u32;
    let blue = self.blue as u32;
    ((red * 299 + green * 587 + blue * 114) / 1000) as u8
  }
  /// Interpolates between two colors, where t=0 -> First color, t=255 -> Second color
  pub fn interpolate(a: &VGAColor, b: &VGAColor, t: u8) -> VGAColor {
    let t1 = (255 - t) as u16;
    let t2 = t as u16;
    VGAColor {
      red: ((a.red as u16 * t1 + b.red as u16 * t2)/255) as u8,
      green: ((a.green as u16 * t1 + b.green as u16 * t2)/255) as u8,
      blue: ((a.blue as u16 * t1 + b.blue as u16 * t2)/255) as u8,
      alpha: ((a.alpha as u16 * t1 + b.alpha as u16 * t2)/255) as u8
    }
  }
}