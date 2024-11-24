use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::Write;

use ris_math::matrix::Mat2;
use ris_math::matrix::Mat2x3;
use ris_math::matrix::Mat2x4;
use ris_math::matrix::Mat3x2;
use ris_math::matrix::Mat3;
use ris_math::matrix::Mat3x4;
use ris_math::matrix::Mat4x2;
use ris_math::matrix::Mat4x3;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;
use ris_math::vector::Bvec2;
use ris_math::vector::Bvec3;
use ris_math::vector::Bvec4;

use crate::FatPtr;

pub fn write_vec2(stream: &mut (impl Write + Seek), value: Vec2) -> Result<FatPtr> {
    let ptr_x = crate::write_f32(stream, value.x())?;
    let ptr_y = crate::write_f32(stream, value.y())?;
    Ok(FatPtr{
        addr: ptr_x.addr,
        len: ptr_x.len + ptr_y.len,
    })
}

pub fn read_vec2(stream: &mut (impl Read + Seek)) -> Result<Vec2> {
    let x = crate::read_f32(stream)?;
    let y = crate::read_f32(stream)?;
    Ok(Vec2(x, y))
}

pub fn write_vec3(stream: &mut (impl Write + Seek), value: Vec3) -> Result<FatPtr> {
    let ptr_x = crate::write_f32(stream, value.x())?;
    let ptr_y = crate::write_f32(stream, value.y())?;
    let ptr_z = crate::write_f32(stream, value.z())?;
    Ok(FatPtr{
        addr: ptr_x.addr,
        len: ptr_x.len + ptr_y.len + ptr_z.len,
    })
}

pub fn read_vec3(stream: &mut (impl Read + Seek)) -> Result<Vec3> {
    let x = crate::read_f32(stream)?;
    let y = crate::read_f32(stream)?;
    let z = crate::read_f32(stream)?;
    Ok(Vec3(x, y, z))
}

pub fn write_vec4(stream: &mut (impl Write + Seek), value: Vec4) -> Result<FatPtr> {
    let ptr_x = crate::write_f32(stream, value.x())?;
    let ptr_y = crate::write_f32(stream, value.y())?;
    let ptr_z = crate::write_f32(stream, value.z())?;
    let ptr_w = crate::write_f32(stream, value.w())?;
    Ok(FatPtr{
        addr: ptr_x.addr,
        len: ptr_x.len + ptr_y.len + ptr_z.len + ptr_w.len,
    })
}

pub fn read_vec4(stream: &mut (impl Read + Seek)) -> Result<Vec4> {
    let x = crate::read_f32(stream)?;
    let y = crate::read_f32(stream)?;
    let z = crate::read_f32(stream)?;
    let w = crate::read_f32(stream)?;
    Ok(Vec4(x, y, z, w))
}

pub fn write_bvec2(stream: &mut (impl Write + Seek), value: Bvec2) -> Result<FatPtr> {
    let x = u8::from(value.x());
    let y = u8::from(value.y());
    let flags = x | y << 1;
    crate::write_u8(stream, flags)
}

pub fn read_bvec2(stream: &mut (impl Read + Seek)) -> Result<Bvec2> {
    let flags = crate::read_u8(stream)?;
    let x = (flags & 1) != 0;
    let y = ((flags >> 1) & 1) != 0;
    Ok(Bvec2(x, y))
}

pub fn write_bvec3(stream: &mut (impl Write + Seek), value: Bvec3) -> Result<FatPtr> {
    let x = u8::from(value.x());
    let y = u8::from(value.y());
    let z = u8::from(value.z());
    let flags = x | y << 1 | z << 2;
    crate::write_u8(stream, flags)
}

pub fn read_bvec3(stream: &mut (impl Read + Seek)) -> Result<Bvec3> {
    let flags = crate::read_u8(stream)?;
    let x = (flags & 1) != 0;
    let y = ((flags >> 1) & 1) != 0;
    let z = ((flags >> 2) & 1) != 0;
    Ok(Bvec3(x, y, z))
}

pub fn write_bvec4(stream: &mut (impl Write + Seek), value: Bvec4) -> Result<FatPtr> {
    let x = u8::from(value.x());
    let y = u8::from(value.y());
    let z = u8::from(value.z());
    let w = u8::from(value.w());
    let flags = x | y << 1 | z << 2 | w << 3;
    crate::write_u8(stream, flags)
}

pub fn read_bvec4(stream: &mut (impl Read + Seek)) -> Result<Bvec4> {
    let flags = crate::read_u8(stream)?;
    let x = (flags & 1) != 0;
    let y = ((flags >> 1) & 1) != 0;
    let z = ((flags >> 2) & 1) != 0;
    let w = ((flags >> 3) & 1) != 0;
    Ok(Bvec4(x, y, z, w))
}

pub fn write_quat(stream: &mut (impl Write + Seek), value: Quat) -> Result<FatPtr> {
    let vec4 = Vec4::from(value);
    write_vec4(stream, vec4)
}

pub fn read_quat(stream: &mut (impl Read + Seek)) -> Result<Quat> {
    let vec4 = read_vec4(stream)?;
    Ok(Quat::from(vec4))
}

pub fn write_mat2(stream: &mut (impl Write + Seek), value: Mat2) -> Result<FatPtr> {
    let ptr_0 = write_vec2(stream, value.0)?;
    let ptr_1 = write_vec2(stream, value.1)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len
    })
}

pub fn read_mat2(stream: &mut (impl Read + Seek)) -> Result<Mat2> {
    let mut m = Mat2::default();
    m.0 = read_vec2(stream)?;
    m.1 = read_vec2(stream)?;
    Ok(m)
}


pub fn write_mat2x3(stream: &mut (impl Write + Seek), value: Mat2x3) -> Result<FatPtr> {
    let ptr_0 = write_vec3(stream, value.0)?;
    let ptr_1 = write_vec3(stream, value.1)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len
    })
}

pub fn read_mat2x3(stream: &mut (impl Read + Seek)) -> Result<Mat2x3> {
    let mut m = Mat2x3::default();
    m.0 = read_vec3(stream)?;
    m.1 = read_vec3(stream)?;
    Ok(m)
}


pub fn write_mat2x4(stream: &mut (impl Write + Seek), value: Mat2x4) -> Result<FatPtr> {
    let ptr_0 = write_vec4(stream, value.0)?;
    let ptr_1 = write_vec4(stream, value.1)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len
    })
}

pub fn read_mat2x4(stream: &mut (impl Read + Seek)) -> Result<Mat2x4> {
    let mut m = Mat2x4::default();
    m.0 = read_vec4(stream)?;
    m.1 = read_vec4(stream)?;
    Ok(m)
}


pub fn write_mat3x2(stream: &mut (impl Write + Seek), value: Mat3x2) -> Result<FatPtr> {
    let ptr_0 = write_vec2(stream, value.0)?;
    let ptr_1 = write_vec2(stream, value.1)?;
    let ptr_2 = write_vec2(stream, value.2)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len,
    })
}

pub fn read_mat3x2(stream: &mut (impl Read + Seek)) -> Result<Mat3x2> {
    let mut m = Mat3x2::default();
    m.0 = read_vec2(stream)?;
    m.1 = read_vec2(stream)?;
    m.2 = read_vec2(stream)?;
    Ok(m)
}


pub fn write_mat3(stream: &mut (impl Write + Seek), value: Mat3) -> Result<FatPtr> {
    let ptr_0 = write_vec3(stream, value.0)?;
    let ptr_1 = write_vec3(stream, value.1)?;
    let ptr_2 = write_vec3(stream, value.2)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len,
    })
}

pub fn read_mat3(stream: &mut (impl Read + Seek)) -> Result<Mat3> {
    let mut m = Mat3::default();
    m.0 = read_vec3(stream)?;
    m.1 = read_vec3(stream)?;
    m.2 = read_vec3(stream)?;
    Ok(m)
}


pub fn write_mat3x4(stream: &mut (impl Write + Seek), value: Mat3x4) -> Result<FatPtr> {
    let ptr_0 = write_vec4(stream, value.0)?;
    let ptr_1 = write_vec4(stream, value.1)?;
    let ptr_2 = write_vec4(stream, value.2)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len,
    })
}

pub fn read_mat3x4(stream: &mut (impl Read + Seek)) -> Result<Mat3x4> {
    let mut m = Mat3x4::default();
    m.0 = read_vec4(stream)?;
    m.1 = read_vec4(stream)?;
    m.2 = read_vec4(stream)?;
    Ok(m)
}


pub fn write_mat4x2(stream: &mut (impl Write + Seek), value: Mat4x2) -> Result<FatPtr> {
    let ptr_0 = write_vec2(stream, value.0)?;
    let ptr_1 = write_vec2(stream, value.1)?;
    let ptr_2 = write_vec2(stream, value.2)?;
    let ptr_3 = write_vec2(stream, value.3)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len + ptr_3.len,
    })
}

pub fn read_mat4x2(stream: &mut (impl Read + Seek)) -> Result<Mat4x2> {
    let mut m = Mat4x2::default();
    m.0 = read_vec2(stream)?;
    m.1 = read_vec2(stream)?;
    m.2 = read_vec2(stream)?;
    m.3 = read_vec2(stream)?;
    Ok(m)
}


pub fn write_mat4x3(stream: &mut (impl Write + Seek), value: Mat4x3) -> Result<FatPtr> {
    let ptr_0 = write_vec3(stream, value.0)?;
    let ptr_1 = write_vec3(stream, value.1)?;
    let ptr_2 = write_vec3(stream, value.2)?;
    let ptr_3 = write_vec3(stream, value.3)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len + ptr_3.len,
    })
}

pub fn read_mat4x3(stream: &mut (impl Read + Seek)) -> Result<Mat4x3> {
    let mut m = Mat4x3::default();
    m.0 = read_vec3(stream)?;
    m.1 = read_vec3(stream)?;
    m.2 = read_vec3(stream)?;
    m.3 = read_vec3(stream)?;
    Ok(m)
}


pub fn write_mat4(stream: &mut (impl Write + Seek), value: Mat4) -> Result<FatPtr> {
    let ptr_0 = write_vec4(stream, value.0)?;
    let ptr_1 = write_vec4(stream, value.1)?;
    let ptr_2 = write_vec4(stream, value.2)?;
    let ptr_3 = write_vec4(stream, value.3)?;
    Ok(FatPtr{
        addr: ptr_0.addr,
        len: ptr_0.len + ptr_1.len + ptr_2.len + ptr_3.len,
    })
}

pub fn read_mat4(stream: &mut (impl Read + Seek)) -> Result<Mat4> {
    let mut m = Mat4::default();
    m.0 = read_vec4(stream)?;
    m.1 = read_vec4(stream)?;
    m.2 = read_vec4(stream)?;
    m.3 = read_vec4(stream)?;
    Ok(m)
}

