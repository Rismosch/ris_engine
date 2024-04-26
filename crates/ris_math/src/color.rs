// OK LAB by Björn Ottoson: https://bottosson.github.io/posts/oklab/

#![allow(clippy::excessive_precision)]

use vulkano::buffer::BufferContents;

use crate::vector::Vec3;
use crate::vector::Vec4;

#[derive(Debug, Default, Clone, Copy, BufferContents)]
#[repr(C)]
pub struct Rgb(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy, BufferContents)]
#[repr(C)]
pub struct OkLab(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy, BufferContents)]
#[repr(C)]
pub struct OkLch(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Clone, Copy, BufferContents)]
#[repr(C)]
pub struct Alpha<T>{
    pub color: T,
    pub alpha: f32,
}

pub type Rgba = Alpha<Rgb>;
pub type OkLaba = Alpha<OkLab>;
pub type OkLcha = Alpha<OkLch>;

//
// constructors
//

impl From<Rgb> for OkLab {
    fn from(value: Rgb) -> Self {
        let c = value;

        let l = 0.4122214708 * c.0 + 0.5363325363 * c.1 + 0.0514459929 * c.2;
        let m = 0.2119034982 * c.0 + 0.6806995451 * c.1 + 0.1073969566 * c.2;
        let s = 0.0883024619 * c.0 + 0.2817188376 * c.1 + 0.6299787005 * c.2;

        let l_ = crate::cbrt(l);
        let m_ = crate::cbrt(m);
        let s_ = crate::cbrt(s);

        Self (
            0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        )
    }
}

impl From<OkLab> for Rgb {
    fn from(value: OkLab) -> Self {
        let c = value;

        let l_ = c.0 + 0.3963377774 * c.1 + 0.2158037573 * c.2;
        let m_ = c.0 - 0.1055613458 * c.1 - 0.0638541728 * c.2;
        let s_ = c.0 - 0.0894841775 * c.1 - 1.2914855480 * c.2;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        Self (
            4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        )
    }
}

impl From<OkLch> for OkLab {
    fn from(value: OkLch) -> Self {
        let l = value.0;
        let a = value.1 * crate::cos(value.2);
        let b = value.1 * crate::sin(value.2);

        Self(l, a, b)
    }
}

impl From<OkLab> for OkLch {
    fn from(value: OkLab) -> Self {
        let l = value.0;
        let c = crate::sqrt(value.1 * value.1 + value.2 * value.2);
        let h = crate::atan2(value.2, value.1);

        Self (l, c, h)
    }
}

impl From<Rgb> for OkLch {
    fn from(value: Rgb) -> Self {
        OkLch::from(OkLab::from(value))
    }
}

impl From<OkLch> for Rgb {
    fn from(value: OkLch) -> Self {
        Rgb::from(OkLab::from(value))
    }
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, alpha: f32) -> Self {
        Self {
            color: Rgb(r, g, b),
            alpha,
        }
    }
}

impl OkLaba {
    pub fn new (l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self {
            color: OkLab(l, a, b),
            alpha,
        }
    }
}

impl OkLcha {
    pub fn new (l: f32, c: f32, h: f32, alpha: f32) -> Self {
        Self {
            color: OkLch(l, c, h),
            alpha,
        }
    }
}

impl From<Vec3> for Rgb {
    fn from(value: Vec3) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<Rgb> for Vec3 {
    fn from(value: Rgb) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<Vec3> for OkLab {
    fn from(value: Vec3) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<OkLab> for Vec3 {
    fn from(value: OkLab) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<Vec3> for OkLch {
    fn from(value: Vec3) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<OkLch> for Vec3 {
    fn from(value: OkLch) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl<T: From<Vec3>> From<Vec4> for Alpha<T> {
    fn from(value: Vec4) -> Self {
        let color = T::from(Vec3::from(value));
        let alpha = value.3;

        Self {
            color,
            alpha,
        }
    }
}

impl<T: Into<Vec3>> From<Alpha<T>> for Vec4 where Vec3: From<T> {
    fn from(value: Alpha<T>) -> Self {
        let vec3 = Vec3::from(value.color);
        let alpha = value.alpha;
        Self(
            vec3.0,
            vec3.1,
            vec3.2,
            alpha,
        )
    }
}

//
// components
//

impl Rgb {
    pub fn r(self) -> f32 {
        self.0
    }

    pub fn g(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn set_r(&mut self, r: f32) {
        self.0 = r;
    }

    pub fn set_g(&mut self, g: f32) {
        self.1 = g;
    }

    pub fn set_b(&mut self, b: f32) {
        self.2 = b;
    }
}

impl OkLab {
    pub fn l(self) -> f32 {
        self.0
    }

    pub fn a(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn set_l(&mut self, r: f32) {
        self.0 = r;
    }

    pub fn set_a(&mut self, g: f32) {
        self.1 = g;
    }

    pub fn set_b(&mut self, b: f32) {
        self.2 = b;
    }
}


impl OkLch {
    pub fn l(self) -> f32 {
        self.0
    }

    pub fn c(self) -> f32 {
        self.1
    }

    pub fn h(self) -> f32 {
        self.2
    }

    pub fn set_l(&mut self, r: f32) {
        self.0 = r;
    }

    pub fn set_c(&mut self, g: f32) {
        self.1 = g;
    }

    pub fn set_h(&mut self, b: f32) {
        self.2 = b;
    }
}

//
// functions
//

impl Rgb {
    pub fn is_valid(&self) -> bool {
        self.0 >= 0. && self.0 <= 1. && self.1 >= 0. && self.1 <= 1. && self.2 >= 0. && self.2 <= 1.
    }

    pub fn with_alpha(self, alpha: f32) -> Alpha<Self> {
        Alpha {
            color: self,
            alpha,
        }
    }
}

impl OkLab {
    pub fn with_alpha(self, alpha: f32) -> Alpha<Self> {
        Alpha {
            color: self,
            alpha,
        }
    }
}

impl OkLch {
    pub fn with_alpha(self, alpha: f32) -> Alpha<Self> {
        Alpha {
            color: self,
            alpha,
        }
    }
}

//
// constants
//

pub const RGB_BLACK: Rgb = Rgb (0.,0.,0.);
pub const RGB_WHITE: Rgb = Rgb (1.,1.,1.);
pub const RGB_RED: Rgb = Rgb (1.,0.,0.);
pub const RGB_GREEN: Rgb = Rgb (0.,1.,0.);
pub const RGB_BLUE: Rgb = Rgb (0.,0.,1.);
pub const RGB_CYAN: Rgb = Rgb (0.,1.,1.);
pub const RGB_MAGENTA: Rgb = Rgb (1.,0.,1.);
pub const RGB_YELLOW: Rgb = Rgb (1.,1.,0.);
