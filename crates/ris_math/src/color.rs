#![allow(clippy::excessive_precision)]

use crate::vector::Vec3;
use crate::vector::Vec4;

pub const MIN_NORM: f32 = 1.0 / 255.0;

//
// errors
//

#[derive(Debug)]
pub struct NotEnoughElements;

#[derive(Debug)]
pub struct InvalidHex;

impl std::fmt::Display for NotEnoughElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough elements")
    }
}

impl std::fmt::Display for InvalidHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid hex")
    }
}

impl std::error::Error for NotEnoughElements {}
impl std::error::Error for InvalidHex {}

//
// traits
//

pub trait Color<const N: usize>: std::fmt::Debug + Default + Clone + Copy {
    fn try_from_f32(v: impl IntoIterator<Item = f32>) -> Result<Self, NotEnoughElements> where Self: Sized {
        let mut iterator = v.into_iter();
        let mut channels = [0.0; N];
        for entry in channels.iter_mut() {
            let Some(value) = iterator.next() else {
                return Err(NotEnoughElements);
            };

            *entry = value;
        }

        Ok(Self::from_f32(channels))
    }

    fn from_f32(values: [f32; N]) -> Self;
    fn to_f32(self) -> [f32; N];
}

pub trait Color3 : Color<3> {
    fn from_vec3(v: Vec3) -> Self {
        Self::from_f32(v.into())
    }

    fn to_vec3(self) -> Vec3 {
        self.to_f32().into()
    }
}

pub trait Color4 : Color<4> {
    fn from_vec3(v: Vec4) -> Self {
        Self::from_f32(v.into())
    }

    fn to_vec4(self) -> Vec4 {
        self.to_f32().into()
    }
}

pub trait ByteColor<const N: usize> : Color<N> {
    fn from_u8(v: [u8; N]) -> Self {
        let mut channels = [0.0; N];
        for (i, value) in v.into_iter().enumerate() {
            channels[i] = value as f32 / 255.0;
        }

        Self::from_f32(channels)
    }

    fn to_u8(self) -> [u8; N] {
        let channels = self.to_f32();
        let mut bytes = [0u8; N];
        for (i, value) in channels.into_iter().enumerate() {
            let value = value.clamp(0.0, 1.0);
            bytes[i] = (value * 255.0) as u8;
        }

        bytes
    }

    fn from_hex(hex: impl AsRef<str>) -> Result<Self, InvalidHex> {
        let hex = hex.as_ref();
        let offset = if hex.starts_with('#') {
            1
        } else {
            0
        };

        let hex_chars = hex.chars().collect::<Vec<_>>();

        let mut channels = [0u8; N];
        for i in 0..N {
            let m = 2 * i + offset;
            let n = m + 1;

            let a = u8::from_str_radix(&hex_chars[m].to_string(), 16).map_err(|_| InvalidHex)?;
            let b = u8::from_str_radix(&hex_chars[n].to_string(), 16).map_err(|_| InvalidHex)?;

            let entry = a << 4 | b;
            channels[i] = entry;
        }

        Ok(Self::from_u8(channels))
    }

    fn to_hex(self) -> String {
        let mut hex = "#".to_string();
        for byte in self.to_u8() {
            let value = format!("{:02X}", byte);
            hex.push_str(&value)
        }

        hex
    }
}

//
// structs
//

/// r: red
/// g: green
/// b: blue
#[derive(Debug, Default, Clone, Copy)]
pub struct Rgb(pub f32, pub f32, pub f32);

/// OK LAB by Björn Ottoson: <https://bottosson.github.io/posts/oklab/>
///
/// l: luminocity / preceived lightness
/// a: how green/red the color is
/// b: how blue/yellow the color is
#[derive(Debug, Default, Clone, Copy)]
pub struct OkLab(pub f32, pub f32, pub f32);

/// OK LAB by Björn Ottoson: <https://bottosson.github.io/posts/oklab/>
///
/// l: luminocity / preceived lightness
/// c: chroma / saturation
/// h: hue
#[derive(Debug, Default, Clone, Copy)]
pub struct OkLch(pub f32, pub f32, pub f32);

/// r: red
/// g: green
/// b: blue
/// a: alpha
#[derive(Debug, Default, Clone, Copy)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

/// OK LAB by Björn Ottoson: <https://bottosson.github.io/posts/oklab/>
///
/// l: luminocity / preceived lightness
/// a: how green/red the color is
/// b: how blue/yellow the color is
/// a: alpha
#[derive(Debug, Default, Clone, Copy)]
pub struct OkLaba(pub f32, pub f32, pub f32, pub f32);

/// OK LAB by Björn Ottoson: <https://bottosson.github.io/posts/oklab/>
///
/// l: luminocity / preceived lightness
/// c: chroma / saturation
/// h: hue
/// a: alpha
#[derive(Debug, Default, Clone, Copy)]
pub struct OkLcha(pub f32, pub f32, pub f32, pub f32);

//
// constructors
//

impl Rgb {
    pub fn black() -> Rgb {
        Rgb(0.0, 0.0, 0.0)
    }

    pub fn white() -> Rgb {
        Rgb(1.0, 1.0, 1.0)
    }

    pub fn red() -> Rgb {
        Rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Rgb {
        Rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Rgb {
        Rgb(0.0, 0.0, 1.0)
    }

    pub fn cyan() -> Rgb {
        Rgb(0.0, 1.0, 1.0)
    }

    pub fn magenta() -> Rgb {
        Rgb(1.0, 0.0, 1.0)
    }

    pub fn yellow() -> Rgb {
        Rgb(1.0, 1.0, 0.0)
    }

    pub fn gray() -> Rgb {
        Rgb(0.5, 0.5, 0.5)
    }

    pub fn grey() -> Rgb {
        Self::gray()
    }
}

//
// conversion
//

impl From<Rgb> for OkLab {
    fn from(value: Rgb) -> Self {
        let c = value;

        let l = 0.4122214708 * c.0 + 0.5363325363 * c.1 + 0.0514459929 * c.2;
        let m = 0.2119034982 * c.0 + 0.6806995451 * c.1 + 0.1073969566 * c.2;
        let s = 0.0883024619 * c.0 + 0.2817188376 * c.1 + 0.6299787005 * c.2;

        let l_ = f32::cbrt(l);
        let m_ = f32::cbrt(m);
        let s_ = f32::cbrt(s);

        Self(
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

        Self(
            4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        )
    }
}

impl From<OkLch> for OkLab {
    fn from(value: OkLch) -> Self {
        let l = value.0;
        let a = value.1 * f32::cos(value.2);
        let b = value.1 * f32::sin(value.2);

        Self(l, a, b)
    }
}

impl From<OkLab> for OkLch {
    fn from(value: OkLab) -> Self {
        let l = value.0;
        let c = f32::sqrt(value.1 * value.1 + value.2 * value.2);
        let h = f32::atan2(value.2, value.1);

        Self(l, c, h)
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

impl From<Rgba> for OkLaba {
    fn from(value: Rgba) -> Self {
        let OkLab(l, a, b) = OkLab::from(value.without_alpha());
        Self(l, a, b, value.alpha())
    }
}

impl From<OkLaba> for Rgba {
    fn from(value: OkLaba) -> Self {
        let Rgb(r, g, b) = Rgb::from(value.without_alpha());
        Self(r, g, b, value.alpha())
    }
}

impl From<OkLcha> for OkLaba {
    fn from(value: OkLcha) -> Self {
        let OkLab(l, a, b) = OkLab::from(value.without_alpha());
        Self(l, a, b, value.alpha())
    }
}

impl From<OkLaba> for OkLcha {
    fn from(value: OkLaba) -> Self {
        let OkLch(l, c, h) = OkLch::from(value.without_alpha());
        Self(l, c, h, value.alpha())
    }
}

impl From<Rgba> for OkLcha {
    fn from(value: Rgba) -> Self {
        let OkLch(l, c, h) = OkLch::from(value.without_alpha());
        Self(l, c, h, value.alpha())
    }
}

impl From<OkLcha> for Rgba {
    fn from(value: OkLcha) -> Self {
        let Rgb(r, g, b) = Rgb::from(value.without_alpha());
        Self(r, g, b, value.alpha())
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

    pub fn set_l(&mut self, l: f32) {
        self.0 = l;
    }

    pub fn set_a(&mut self, a: f32) {
        self.1 = a;
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

    pub fn set_l(&mut self, l: f32) {
        self.0 = l;
    }

    pub fn set_c(&mut self, c: f32) {
        self.1 = c;
    }

    pub fn set_h(&mut self, h: f32) {
        self.2 = h;
    }
}

impl Rgba {
    pub fn r(self) -> f32 {
        self.0
    }

    pub fn g(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn alpha(self) -> f32 {
        self.3
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

    pub fn set_alpha(&mut self, alpha: f32) {
        self.3 = alpha;
    }
}

impl OkLaba {
    pub fn l(self) -> f32 {
        self.0
    }

    pub fn a(self) -> f32 {
        self.1
    }

    pub fn b(self) -> f32 {
        self.2
    }

    pub fn alpha(self) -> f32 {
        self.3
    }

    pub fn set_l(&mut self, l: f32) {
        self.0 = l;
    }

    pub fn set_a(&mut self, a: f32) {
        self.1 = a;
    }

    pub fn set_b(&mut self, b: f32) {
        self.2 = b;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.3 = alpha;
    }
}

impl OkLcha {
    pub fn l(self) -> f32 {
        self.0
    }

    pub fn c(self) -> f32 {
        self.1
    }

    pub fn h(self) -> f32 {
        self.2
    }

    pub fn alpha(self) -> f32 {
        self.3
    }

    pub fn set_l(&mut self, l: f32) {
        self.0 = l;
    }

    pub fn set_c(&mut self, c: f32) {
        self.1 = c;
    }

    pub fn set_h(&mut self, h: f32) {
        self.2 = h;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.3 = alpha;
    }
}

impl std::ops::Index<usize> for Rgb
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Rgb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for OkLab
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for OkLab {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for OkLch
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for OkLch {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Rgba
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Rgba {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for OkLaba
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for OkLaba {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for OkLcha
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for OkLcha {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => unreachable!(),
        }
    }
}

//
// functions
//

impl Rgb {
    pub fn with_alpha(self, alpha: f32) -> Rgba {
        Rgba(self.0, self.1, self.2, alpha)
    }
}

impl OkLab {
    pub fn with_alpha(self, alpha: f32) -> OkLaba {
        OkLaba(self.0, self.1, self.2, alpha)
    }
}

impl OkLch {
    pub fn with_alpha(self, alpha: f32) -> OkLcha {
        OkLcha(self.0, self.1, self.2, alpha)
    }
}

impl Rgba {
    pub fn without_alpha(self) -> Rgb {
        Rgb(self.0, self.1, self.2)
    }
}

impl OkLaba {
    pub fn without_alpha(self) -> OkLab {
        OkLab(self.0, self.1, self.2)
    }
}

impl OkLcha {
    pub fn without_alpha(self) -> OkLch {
        OkLch(self.0, self.1, self.2)
    }
}


//
// trait impls
//

impl Color<3> for Rgb {
    fn from_f32(values: [f32; 3]) -> Self {
        Self(values[0], values[1], values[2])
    }

    fn to_f32(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
}

impl Color<3> for OkLab {
    fn from_f32(values: [f32; 3]) -> Self {
        Self(values[0], values[1], values[2])
    }

    fn to_f32(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
}

impl Color<3> for OkLch {
    fn from_f32(values: [f32; 3]) -> Self {
        Self(values[0], values[1], values[2])
    }

    fn to_f32(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
}

impl Color<4> for Rgba {
    fn from_f32(values: [f32; 4]) -> Self {
        Self(values[0], values[1], values[2], values[3])
    }

    fn to_f32(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl Color<4> for OkLaba {
    fn from_f32(values: [f32; 4]) -> Self {
        Self(values[0], values[1], values[2], values[3])
    }

    fn to_f32(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl Color<4> for OkLcha {
    fn from_f32(values: [f32; 4]) -> Self {
        Self(values[0], values[1], values[2], values[3])
    }

    fn to_f32(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl Color3 for Rgb {}
impl Color3 for OkLab {}
impl Color3 for OkLch {}
impl Color4 for Rgba {}
impl Color4 for OkLaba {}
impl Color4 for OkLcha {}

impl ByteColor<3> for Rgb {}
impl ByteColor<4> for Rgba {}

//
// other
//

#[derive(Debug, Default, Clone)]
pub struct Gradient<T: Color<N>, const N: usize>(Vec<T>);

impl<T: Color<N>, const N: usize> Gradient<T, N> {
    pub fn try_from(value: impl IntoIterator<Item = T>) -> Result<Self, NotEnoughElements> {
        let colors = value.into_iter().collect::<Vec<_>>();
        if colors.is_empty() {
            Err(NotEnoughElements)
        } else {
            Ok(Self(colors))
        }
    }

    pub fn sample(&self, x: f32) -> T {
        if self.0.len() == 1 {
            return self.0[0];
        }

        if x <= 0.0 {
            return self.0[0];
        }

        if x >= 1.0 {
            let last_index = self.0.len() - 1;
            return self.0[last_index]
        }

        let splits = (self.0.len() - 1) as f32;
        let scaled = x * splits;
        let lower = scaled.floor() as usize;
        let upper = scaled.ceil() as usize;
        let lerp = scaled % 1.0;

        let color_1 = self.0[lower].to_f32();
        let color_2 = self.0[upper].to_f32();

        let mut mix = [0.0; N];
        for i in 0..N {
            let a = color_1[i];
            let b = color_2[i];
            mix[i] = crate::common::mix(a, b, lerp);
        }

        T::from_f32(mix)
    }
}
