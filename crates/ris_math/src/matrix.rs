use vulkano::buffer::BufferContents;

use crate::vector::Vec2;
use crate::vector::Vec3;
use crate::vector::Vec4;

//
// definition
//

// the first number in the type is the number of columns, the second is the number of rows. note
// that the vecs are columns, not rows!

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x2(pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x3(pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat2x4(pub Vec4, pub Vec4);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x2(pub Vec2, pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x3(pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat3x4(pub Vec4, pub Vec4, pub Vec4);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x2(pub Vec2, pub Vec2, pub Vec2, pub Vec2);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x3(pub Vec3, pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct Mat4x4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

// 
// constructors
//

impl Mat2x2 {
    pub fn init(value: f32) -> Self {
        Self(
            Vec2(value, 0.),
            Vec2(0., value),
        )
    }
}

impl Mat3x3 {
    pub fn init(value: f32) -> Self {
        Self(
            Vec3(value, 0., 0.),
            Vec3(0., value, 0.),
            Vec3(0., 0., value),
        )
    }
}

impl Mat4x4 {
    pub fn init(value: f32) -> Self {
        Self(
            Vec4(value, 0., 0., 0.),
            Vec4(0., value, 0., 0.),
            Vec4(0., 0., value, 0.),
            Vec4(0., 0., 0., value),
        )
    }
}

impl From<Mat2x2> for Mat3x3 {
    fn from(value: Mat2x2) -> Self {
        let mut r = Self::init(1.);

        r.0.0 = value.0.0;
        r.1.0 = value.1.0;

        r.0.1 = value.0.1;
        r.1.1 = value.1.1;

        r
    }
}

impl From<Mat2x2> for Mat4x4 {
    fn from(value: Mat2x2) -> Self {
        let mut r = Self::init(1.);

        r.0.0 = value.0.0;
        r.1.0 = value.1.0;

        r.0.1 = value.0.1;
        r.1.1 = value.1.1;

        r
    }
}

impl From<Mat3x3> for Mat4x4 {
    fn from(value: Mat3x3) -> Self {
        let mut r = Self::init(1.);

        r.0.0 = value.0.0;
        r.1.0 = value.1.0;
        r.2.0 = value.2.0;

        r.0.1 = value.0.1;
        r.1.1 = value.1.1;
        r.2.1 = value.2.1;

        r.0.2 = value.0.2;
        r.1.2 = value.1.2;
        r.2.2 = value.2.2;

        r
    }
}

//
// components
//

impl std::ops::Index<usize> for Mat2x2 {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat2x3 {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat2x4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);

        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<usize> for Mat3x2 {
    type Output = Vec2;

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

impl std::ops::IndexMut<usize> for Mat3x2 {
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

impl std::ops::Index<usize> for Mat3x3 {
    type Output = Vec3;

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

impl std::ops::IndexMut<usize> for Mat3x3 {
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

impl std::ops::Index<usize> for Mat3x4 {
    type Output = Vec4;

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

impl std::ops::IndexMut<usize> for Mat3x4 {
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

impl std::ops::Index<usize> for Mat4x2 {
    type Output = Vec2;

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

impl std::ops::IndexMut<usize> for Mat4x2 {
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

impl std::ops::Index<usize> for Mat4x3 {
    type Output = Vec3;

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

impl std::ops::IndexMut<usize> for Mat4x3 {
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

impl std::ops::Index<usize> for Mat4x4 {
    type Output = Vec4;

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

impl std::ops::IndexMut<usize> for Mat4x4 {
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
// operations
//

impl std::ops::Mul<Mat2x2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat2x2) -> Self::Output {
        let v = self;
        let mut u = Vec2::default();
        let m = rhs;

        u.0 = Vec2::dot(v, m.0);
        u.1 = Vec2::dot(v, m.1);

        u
    }
}

impl std::ops::Mul<Mat3x3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Mat3x3) -> Self::Output {
        let v = self;
        let mut u = Vec3::default();
        let m = rhs;

        u.0 = Vec3::dot(v, m.0);
        u.1 = Vec3::dot(v, m.1);
        u.2 = Vec3::dot(v, m.2);

        u
    }
}

impl std::ops::Mul<Mat4x4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let v = self;
        let mut u = Vec4::default();
        let m = rhs;

        u.0 = Vec4::dot(v, m.0);
        u.1 = Vec4::dot(v, m.1);
        u.2 = Vec4::dot(v, m.2);
        u.3 = Vec4::dot(v, m.3);

        u
    }
}

impl std::ops::Mul<Vec2> for Mat2x2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let v = rhs;
        let mut u = Vec2::default();
        let m = self;

        u.0 = m.0.0 * v.0 + m.1.0 * v.1;
        u.1 = m.0.1 * v.0 + m.1.1 * v.1;

        u
    }
}

impl std::ops::Mul<Vec3> for Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let v = rhs;
        let mut u = Vec3::default();
        let m = self;

        u.0 = m.0.0 * v.0 + m.1.0 * v.1 + m.2.0 * v.2;
        u.1 = m.0.1 * v.0 + m.1.1 * v.1 + m.2.1 * v.2;
        u.2 = m.0.2 * v.0 + m.1.2 * v.1 + m.2.2 * v.2;

        u
    }
}

impl std::ops::Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let v = rhs;
        let mut u = Vec4::default();
        let m = self;

        u.0 = m.0.0 * v.0 + m.1.0 * v.1 + m.2.0 * v.2 + m.3.0 * v.3;
        u.1 = m.0.1 * v.0 + m.1.1 * v.1 + m.2.1 * v.2 + m.3.1 * v.3;
        u.2 = m.0.2 * v.0 + m.1.2 * v.1 + m.2.2 * v.2 + m.3.2 * v.3;
        u.3 = m.0.3 * v.0 + m.1.3 * v.1 + m.2.3 * v.2 + m.3.3 * v.3;

        u
    }
}

impl std::ops::Mul<Mat2x2> for Mat2x2 {
    type Output = Mat2x2;

    fn mul(self, rhs: Mat2x2) -> Self::Output {
        let m = self;
        let n = rhs;
        let mut r = Mat2x2::default();

        r.0.0 = m.0.0 * n.0.0 + m.1.0 * n.0.1;
        r.1.0 = m.0.0 * n.1.0 + m.1.0 * n.1.1;

        r.0.1 = m.0.1 * n.0.0 + m.1.1 * n.0.1;
        r.1.1 = m.0.1 * n.1.0 + m.1.1 * n.1.1;

        r
    }
}

impl std::ops::Mul<Mat3x3> for Mat3x3 {
    type Output = Mat3x3;

    fn mul(self, rhs: Mat3x3) -> Self::Output {
        let m = self;
        let n = rhs;
        let mut r = Mat3x3::default();

        r.0.0 = m.0.0 * n.0.0 + m.1.0 * n.0.1 + m.2.0 * n.0.2;
        r.1.0 = m.0.0 * n.1.0 + m.1.0 * n.1.1 + m.2.0 * n.1.2;
        r.2.0 = m.0.0 * n.2.0 + m.1.0 * n.2.1 + m.2.0 * n.2.2;

        r.0.1 = m.0.1 * n.0.0 + m.1.1 * n.0.1 + m.2.1 * n.0.2;
        r.1.1 = m.0.1 * n.1.0 + m.1.1 * n.1.1 + m.2.1 * n.1.2;
        r.2.1 = m.0.1 * n.2.0 + m.1.1 * n.2.1 + m.2.1 * n.2.2;

        r.0.2 = m.0.2 * n.0.0 + m.1.2 * n.0.1 + m.2.2 * n.0.2;
        r.1.2 = m.0.2 * n.1.0 + m.1.2 * n.1.1 + m.2.2 * n.1.2;
        r.2.2 = m.0.2 * n.2.0 + m.1.2 * n.2.1 + m.2.2 * n.2.2;

        r
    }
}

impl std::ops::Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let m = self;
        let n = rhs;
        let mut r = Mat4x4::default();

        r.0.0 = m.0.0 * n.0.0 + m.1.0 * n.0.1 + m.2.0 * n.0.2 + m.3.0 * n.0.3;
        r.1.0 = m.0.0 * n.1.0 + m.1.0 * n.1.1 + m.2.0 * n.1.2 + m.3.0 * n.1.3;
        r.2.0 = m.0.0 * n.2.0 + m.1.0 * n.2.1 + m.2.0 * n.2.2 + m.3.0 * n.2.3;
        r.3.0 = m.0.0 * n.3.0 + m.1.0 * n.3.1 + m.2.0 * n.3.2 + m.3.0 * n.3.3;

        r.0.1 = m.0.1 * n.0.0 + m.1.1 * n.0.1 + m.2.1 * n.0.2 + m.3.1 * n.0.3;
        r.1.1 = m.0.1 * n.1.0 + m.1.1 * n.1.1 + m.2.1 * n.1.2 + m.3.1 * n.1.3;
        r.2.1 = m.0.1 * n.2.0 + m.1.1 * n.2.1 + m.2.1 * n.2.2 + m.3.1 * n.2.3;
        r.3.1 = m.0.1 * n.3.0 + m.1.1 * n.3.1 + m.2.1 * n.3.2 + m.3.1 * n.3.3;

        r.0.2 = m.0.2 * n.0.0 + m.1.2 * n.0.1 + m.2.2 * n.0.2 + m.3.2 * n.0.3;
        r.1.2 = m.0.2 * n.1.0 + m.1.2 * n.1.1 + m.2.2 * n.1.2 + m.3.2 * n.1.3;
        r.2.2 = m.0.2 * n.2.0 + m.1.2 * n.2.1 + m.2.2 * n.2.2 + m.3.2 * n.2.3;
        r.3.2 = m.0.2 * n.3.0 + m.1.2 * n.3.1 + m.2.2 * n.3.2 + m.3.2 * n.3.3;

        r.0.3 = m.0.3 * n.0.0 + m.1.3 * n.0.1 + m.2.3 * n.0.2 + m.3.3 * n.0.3;
        r.1.3 = m.0.3 * n.1.0 + m.1.3 * n.1.1 + m.2.3 * n.1.2 + m.3.3 * n.1.3;
        r.2.3 = m.0.3 * n.2.0 + m.1.3 * n.2.1 + m.2.3 * n.2.2 + m.3.3 * n.2.3;
        r.3.3 = m.0.3 * n.3.0 + m.1.3 * n.3.1 + m.2.3 * n.3.2 + m.3.3 * n.3.3;

        r
    }
}

//
// functions
//

impl Mat2x2 {
    /// multiply matrix self by matrix rhs component-wise. i.e., result.i.j is the scalar product
    /// of self.i.j and rhs.i.j
    ///
    /// Note: to get linear algebraic matrix multiplication, use the multiply operator(*)
    pub fn comp_mul(self, rhs: Self) -> Self {
        let x = self;
        let y = rhs;
        let mut r = Self::default();

        r.0.0 = x.0.0 * y.0.0;
        r.1.0 = x.1.0 * y.1.0;

        r.0.1 = x.0.1 * y.0.1;
        r.1.1 = x.1.1 * y.1.1;

        r
    }

    // treats the first parameter c as a column vector (matrix with one column) and the second
    // parameter r as a row vector (matrix with one row) and does a linear algebraic matrix
    // multiply c * r
    pub fn outer_product(c: Vec2, r: Vec2) -> Self {
        let mut r_ = Self::default();

        r_.0.0 = c.0 * r.0;
        r_.1.0 = c.0 * r.1;

        r_.0.1 = c.1 * r.0;
        r_.1.1 = c.1 * r.1;

        r_
    }
    
    /// returns a matrix that is the transpose of self
    pub fn transpose(self) -> Self {
        let mut r = Self::default();

        r.0.0 = self.0.0;
        r.1.0 = self.0.1;

        r.0.1 = self.1.0;
        r.1.1 = self.1.1;

        r
    }

    /// returns the determinant of m
    pub fn determinant(self) -> f32 {
        let Mat2x2(Vec2(a, b), Vec2(c, d)) = self;
        a * d - b * c
    }

    /// returns a matrix that is the inverse of self
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det < crate::MIN_NORM {
            return None; // matrix is not invertible
        }

        let Mat2x2(Vec2(a, b), Vec2(c, d)) = self;
        
        // adjoint matrix
        let mut r = Mat2x2(Vec2(d, -b), Vec2(-c,a));

        // multiply by 1 / det
        r.0 /= det;
        r.1 /= det;

        Some(r)
    }
}

impl Mat3x3 {
    /// multiply matrix self by matrix rhs component-wise. i.e., result.i.j is the scalar product
    /// of self.i.j and rhs.i.j
    ///
    /// Note: to get linear algebraic matrix multiplication, use the multiply operator(*)
    pub fn comp_mul(self, rhs: Self) -> Self {
        let x = self;
        let y = rhs;
        let mut r = Self::default();

        r.0.0 = x.0.0 * y.0.0;
        r.1.0 = x.1.0 * y.1.0;
        r.2.0 = x.2.0 * y.2.0;

        r.0.1 = x.0.1 * y.0.1;
        r.1.1 = x.1.1 * y.1.1;
        r.2.1 = x.2.1 * y.2.1;

        r.0.2 = x.0.2 * y.0.2;
        r.1.2 = x.1.2 * y.1.2;
        r.2.2 = x.2.2 * y.2.2;

        r
    }

    // treats the first parameter c as a column vector (matrix with one column) and the second
    // parameter r as a row vector (matrix with one row) and does a linear algebraic matrix
    // multiply c * r
    pub fn outer_product(c: Vec3, r: Vec3) -> Self {
        let mut r_ = Self::default();

        r_.0.0 = c.0 * r.0;
        r_.1.0 = c.0 * r.1;
        r_.2.0 = c.0 * r.2;

        r_.0.1 = c.1 * r.0;
        r_.1.1 = c.1 * r.1;
        r_.2.1 = c.1 * r.2;

        r_.0.2 = c.2 * r.0;
        r_.1.2 = c.2 * r.1;
        r_.2.2 = c.2 * r.2;

        r_
    }
    
    /// returns a matrix that is the transpose of self
    pub fn transpose(self) -> Self {
        let mut r = Self::default();

        r.0.0 = self.0.0;
        r.1.0 = self.0.1;
        r.2.0 = self.0.2;

        r.0.1 = self.1.0;
        r.1.1 = self.1.1;
        r.2.1 = self.1.2;

        r.0.2 = self.2.0;
        r.1.2 = self.2.1;
        r.2.2 = self.2.2;

        r
    }

    /// returns the determinant of m
    pub fn determinant(self) -> f32 {
        let Mat3x3(Vec3(a, b, c), Vec3(d, e, f), Vec3(g, h, i)) = self;
        let ma = Mat2x2(Vec2(e, f), Vec2(h, i));
        let md = Mat2x2(Vec2(b, c), Vec2(h, i));
        let mg = Mat2x2(Vec2(b, c), Vec2(e, f));

        a * ma.determinant() - d * md.determinant() + g * mg.determinant()
    }

    /// returns a matrix that is the inverse of self
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det < crate::MIN_NORM {
            return None; // matrix is not invertible
        }

        let Mat3x3(Vec3(a, b, c), Vec3(d, e, f), Vec3(g, h, i)) = self;

        // matrix of minors
        let mut mm = Mat3x3::default();

        mm.0.0 = Mat2x2(Vec2(e, f), Vec2(h, i)).determinant();
        mm.1.0 = Mat2x2(Vec2(b, c), Vec2(h, i)).determinant();
        mm.2.0 = Mat2x2(Vec2(b, c), Vec2(e, f)).determinant();

        mm.0.1 = Mat2x2(Vec2(d, f), Vec2(g, i)).determinant();
        mm.1.1 = Mat2x2(Vec2(a, c), Vec2(g, i)).determinant();
        mm.2.1 = Mat2x2(Vec2(a, c), Vec2(d, f)).determinant();

        mm.0.2 = Mat2x2(Vec2(d, e), Vec2(g, h)).determinant();
        mm.1.2 = Mat2x2(Vec2(a, b), Vec2(g, h)).determinant();
        mm.2.2 = Mat2x2(Vec2(a, b), Vec2(d, e)).determinant();
        
        // matrix of cofactors
        let mut mcf = mm;

        mcf.0.1 *= -1.;
        mcf.1.0 *= -1.;
        mcf.1.2 *= -1.;
        mcf.2.1 *= -1.;

        // adjucate matrix
        let madj = mcf.transpose();

        // multiply by 1 / det
        let r = Mat3x3(
            madj.0 / det,
            madj.1 / det,
            madj.2 / det,
        );

        Some(r)
    }
}

impl Mat4x4 {
    /// multiply matrix self by matrix rhs component-wise. i.e., result.i.j is the scalar product
    /// of self.i.j and rhs.i.j
    ///
    /// Note: to get linear algebraic matrix multiplication, use the multiply operator(*)
    pub fn comp_mul(self, rhs: Self) -> Self {
        let x = self;
        let y = rhs;
        let mut r = Self::default();

        r.0.0 = x.0.0 * y.0.0;
        r.1.0 = x.1.0 * y.1.0;
        r.2.0 = x.2.0 * y.2.0;
        r.3.0 = x.3.0 * y.3.0;

        r.0.1 = x.0.1 * y.0.1;
        r.1.1 = x.1.1 * y.1.1;
        r.2.1 = x.2.1 * y.2.1;
        r.3.1 = x.3.1 * y.3.1;

        r.0.2 = x.0.2 * y.0.2;
        r.1.2 = x.1.2 * y.1.2;
        r.2.2 = x.2.2 * y.2.2;
        r.3.2 = x.3.2 * y.3.2;

        r.0.3 = x.0.3 * y.0.3;
        r.1.3 = x.1.3 * y.1.3;
        r.2.3 = x.2.3 * y.2.3;
        r.3.3 = x.3.3 * y.3.3;

        r
    }

    // treats the first parameter c as a column vector (matrix with one column) and the second
    // parameter r as a row vector (matrix with one row) and does a linear algebraic matrix
    // multiply c * r
    pub fn outer_product(c: Vec4, r: Vec4) -> Self {
        let mut r_ = Self::default();

        r_.0.0 = c.0 * r.0;
        r_.1.0 = c.0 * r.1;
        r_.2.0 = c.0 * r.2;
        r_.3.0 = c.0 * r.3;

        r_.0.1 = c.1 * r.0;
        r_.1.1 = c.1 * r.1;
        r_.2.1 = c.1 * r.2;
        r_.3.1 = c.1 * r.3;

        r_.0.2 = c.2 * r.0;
        r_.1.2 = c.2 * r.1;
        r_.2.2 = c.2 * r.2;
        r_.3.2 = c.2 * r.3;

        r_.0.3 = c.3 * r.0;
        r_.1.3 = c.3 * r.1;
        r_.2.3 = c.3 * r.2;
        r_.3.3 = c.3 * r.3;

        r_
    }
    
    /// returns a matrix that is the transpose of self
    pub fn transpose(self) -> Self {
        let mut r = Self::default();

        r.0.0 = self.0.0;
        r.1.0 = self.0.1;
        r.2.0 = self.0.2;
        r.3.0 = self.0.3;

        r.0.1 = self.1.0;
        r.1.1 = self.1.1;
        r.2.1 = self.1.2;
        r.3.1 = self.1.3;

        r.0.2 = self.2.0;
        r.1.2 = self.2.1;
        r.2.2 = self.2.2;
        r.3.2 = self.2.3;

        r.0.3 = self.3.0;
        r.1.3 = self.3.1;
        r.2.3 = self.3.2;
        r.3.3 = self.3.3;

        r
    }

    /// returns the determinant of m
    pub fn determinant(self) -> f32 {
        let Mat4x4(Vec4(a, b, c, d),Vec4(e, f, g, h),Vec4(i, j, k, l),Vec4(m, n, o, p)) = self;
        let ma = Mat3x3(Vec3(f, g, h),Vec3(j, k, l),Vec3(n, o, p));
        let me = Mat3x3(Vec3(b, c, d),Vec3(j, k, l),Vec3(n, o, p));
        let mi = Mat3x3(Vec3(b, c, d),Vec3(f, g, h),Vec3(n, o, p));
        let mm = Mat3x3(Vec3(b, c, d),Vec3(f, g, h),Vec3(j, k, l));

        a * ma.determinant() - e * me.determinant() + i * mi.determinant() - m * mm.determinant()
    }

    /// returns a matrix that is the inverse of self
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det < crate::MIN_NORM {
            return None; // matrix is not invertible
        }

        let Mat4x4(Vec4(a,b,c,d), Vec4(e,f,g,h), Vec4(i,j,k,l), Vec4(m,n,o,p)) = self;

        // matrix of minors
        let mut mm = Mat4x4::default();

        mm.0.0 = Mat3x3(Vec3(f,g,h), Vec3(j,k,l), Vec3(n,o,p)).determinant();
        mm.1.0 = Mat3x3(Vec3(b,c,d), Vec3(j,k,l), Vec3(n,o,p)).determinant();
        mm.2.0 = Mat3x3(Vec3(b,c,d), Vec3(f,g,h), Vec3(n,o,p)).determinant();
        mm.3.0 = Mat3x3(Vec3(b,c,d), Vec3(f,g,h), Vec3(j,k,l)).determinant();

        mm.0.1 = Mat3x3(Vec3(e,g,h), Vec3(i,k,l), Vec3(m,o,p)).determinant();
        mm.1.1 = Mat3x3(Vec3(a,c,d), Vec3(i,k,l), Vec3(m,o,p)).determinant();
        mm.2.1 = Mat3x3(Vec3(a,c,d), Vec3(e,g,h), Vec3(m,o,p)).determinant();
        mm.3.1 = Mat3x3(Vec3(a,c,d), Vec3(e,g,h), Vec3(i,k,l)).determinant();

        mm.0.2 = Mat3x3(Vec3(e,f,h), Vec3(i,j,l), Vec3(m,n,p)).determinant();
        mm.1.2 = Mat3x3(Vec3(a,b,d), Vec3(i,j,l), Vec3(m,n,p)).determinant();
        mm.2.2 = Mat3x3(Vec3(a,b,d), Vec3(e,f,h), Vec3(m,n,p)).determinant();
        mm.3.2 = Mat3x3(Vec3(a,b,d), Vec3(e,f,h), Vec3(i,j,l)).determinant();

        mm.0.3 = Mat3x3(Vec3(e,f,g), Vec3(i,j,k), Vec3(m,n,o)).determinant();
        mm.1.3 = Mat3x3(Vec3(a,b,c), Vec3(i,j,k), Vec3(m,n,o)).determinant();
        mm.2.3 = Mat3x3(Vec3(a,b,c), Vec3(e,f,g), Vec3(m,n,o)).determinant();
        mm.3.3 = Mat3x3(Vec3(a,b,c), Vec3(e,f,g), Vec3(i,j,k)).determinant();

        // matrix of cofactors
        let mut mcf = mm;
        
        mcf.0.1 *= -1.;
        mcf.0.3 *= -1.;
        mcf.1.0 *= -1.;
        mcf.1.2 *= -1.;
        mcf.2.1 *= -1.;
        mcf.2.3 *= -1.;
        mcf.3.0 *= -1.;
        mcf.3.2 *= -1.;

        // adjugate matrix
        let madj = mcf.transpose();

        // multiply by 1 / det
        let r = Mat4x4(
            madj.0 / det,
            madj.1 / det,
            madj.2 / det,
            madj.3 / det,
        );

        Some(r)

    }
}
