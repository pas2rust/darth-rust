#![cfg(all(feature = "build", feature = "math"))]

use darth_rust::DarthRust;
#[derive(Debug, DarthRust, Clone)]
pub struct CalcStruct {
    pub usize: usize,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub u128: u128,
    pub isize: isize,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
    pub f64: f64,
    pub f32: f32,
}
#[test]
fn inflate_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_usize(20.0);
    assert_eq!(calc_struct.usize, 12);
}

#[test]
fn inflate_u8() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_u8(20.0);
    assert_eq!(calc_struct.u8, 24);
}

#[test]
fn inflate_u16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_u16(20.0);
    assert_eq!(calc_struct.u16, 36);
}

#[test]
fn inflate_u32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_u32(20.0);
    assert_eq!(calc_struct.u32, 48);
}

#[test]
fn inflate_u64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_u64(20.0);
    assert_eq!(calc_struct.u64, 60);
}

#[test]
fn inflate_u128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_u128(20.0);
    assert_eq!(calc_struct.u128, 72);
}

#[test]
fn inflate_isize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_isize(20.0);
    assert_eq!(calc_struct.isize, 84);
}

#[test]
fn inflate_i16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_i16(20.0);
    assert_eq!(calc_struct.i16, 96);
}

#[test]
fn inflate_i32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_i32(20.0);
    assert_eq!(calc_struct.i32, 108);
}

#[test]
fn inflate_i64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_i64(20.0);
    assert_eq!(calc_struct.i64, 120);
}

#[test]
fn inflate_i128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_i128(20.0);
    assert_eq!(calc_struct.i128, 132);
}

#[test]
fn inflate_f64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_f64(20.0);
    assert_eq!(calc_struct.f64, 144.0);
}

#[test]
fn inflate_f32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .u8::<u8>(20)
        .u16::<u16>(30)
        .u32::<u32>(40)
        .u64::<u64>(50)
        .u128::<u128>(60)
        .isize::<isize>(70)
        .i16::<i16>(80)
        .i32(90)
        .i64(100)
        .i128(110)
        .f64(120.0)
        .f32(130.0)
        .build()
        .unwrap();
    calc_struct.inflate_f32(20.0);
    assert_eq!(calc_struct.f32, 156.0);
}
