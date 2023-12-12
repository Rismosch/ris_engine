// OK LAB by Björn Ottoson: https://bottosson.github.io/posts/oklab/

#![allow(clippy::excessive_precision)]

#[derive(Debug, Default, Clone, Copy)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Lab {
    pub fn chroma(&self) -> f32 {
        crate::sqrt(self.a * self.a + self.b * self.b)
    }

    pub fn set_chroma(&mut self, chroma: f32) {
        let hue = self.hue();
        self.set_chroma_and_hue(chroma, hue);
    }

    pub fn hue(&self) -> f32 {
        crate::atan2(self.b, self.a)
    }

    pub fn set_hue(&mut self, hue: f32) {
        let chroma = self.chroma();
        self.set_chroma_and_hue(chroma, hue);
    }

    pub fn set_chroma_and_hue(&mut self, chroma: f32, hue: f32) {
        let a = chroma * crate::cos(hue);
        let b = chroma * crate::sin(hue);

        self.a = a;
        self.b = b;
    }
}

impl From<Rgb> for Lab {
    fn from(value: Rgb) -> Self {
        let c = value;

        let l = 0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b;
        let m = 0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b;
        let s = 0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b;

        let l_ = crate::cbrt(l);
        let m_ = crate::cbrt(m);
        let s_ = crate::cbrt(s);

        Self {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

impl From<Lab> for Rgb {
    fn from(value: Lab) -> Self {
        let c = value;

        let l_ = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
        let m_ = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
        let s_ = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        Self {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        }
    }
}

pub const LAB_BLACK: Lab = Lab {
    l: 0.,
    a: 0.,
    b: 0.,
};
pub const LAB_WHITE: Lab = Lab {
    l: 1.,
    a: 0.,
    b: 0.,
};
pub const LAB_RED: Lab = Lab {
    l: 0.6279554,
    a: 0.22486307,
    b: 0.1258463,
};
pub const LAB_GREEN: Lab = Lab {
    l: 0.8664396,
    a: -0.2338874,
    b: 0.1794985,
};
pub const LAB_BLUE: Lab = Lab {
    l: 0.45201376,
    a: -0.032456964,
    b: -0.31152812,
};
pub const LAB_CYAN: Lab = Lab {
    l: 0.9053992,
    a: -0.14944354,
    b: -0.039398193,
};
pub const LAB_MAGENTA: Lab = Lab {
    l: 0.7016738,
    a: 0.2745664,
    b: -0.16915607,
};
pub const LAB_YELLOW: Lab = Lab {
    l: 0.9679827,
    a: -0.07136908,
    b: 0.19856977,
};

pub const RGB_BLACK: Rgb = Rgb {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const RGB_WHITE: Rgb = Rgb {
    r: 1.,
    g: 1.,
    b: 1.,
};
pub const RGB_RED: Rgb = Rgb {
    r: 1.,
    g: 0.,
    b: 0.,
};
pub const RGB_GREEN: Rgb = Rgb {
    r: 0.,
    g: 1.,
    b: 0.,
};
pub const RGB_BLUE: Rgb = Rgb {
    r: 0.,
    g: 0.,
    b: 1.,
};
pub const RGB_CYAN: Rgb = Rgb {
    r: 0.,
    g: 1.,
    b: 1.,
};
pub const RGB_MAGENTA: Rgb = Rgb {
    r: 1.,
    g: 0.,
    b: 1.,
};
pub const RGB_YELLOW: Rgb = Rgb {
    r: 1.,
    g: 1.,
    b: 0.,
};

