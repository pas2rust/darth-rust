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
fn subtract_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_usize(5);

    assert_eq!(calc_struct.usize, 1);
}

#[test]
fn subtract_u8() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_u8(5);
    assert_eq!(calc_struct.u8, 2);
}

#[test]
fn subtract_u16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_u16(5);
    assert_eq!(calc_struct.u16, 3);
}

#[test]
fn subtract_u32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_u32(5);
    assert_eq!(calc_struct.u32, 4);
}

#[test]
fn subtract_u64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_u64(5);
    assert_eq!(calc_struct.u64, 5);
}

#[test]
fn subtract_u128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_u128(5);
    assert_eq!(calc_struct.u128, 6);
}

#[test]
fn subtract_isize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_isize(5);

    assert_eq!(calc_struct.isize, 7);
}

#[test]
fn subtract_i16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_i16(5);
    assert_eq!(calc_struct.i16, 8);
}

#[test]
fn subtract_i32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_i32(5);
    assert_eq!(calc_struct.i32, 9);
}

#[test]
fn subtract_i64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_i64(5);
    assert_eq!(calc_struct.i64, 10);
}

#[test]
fn subtract_i128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_i128(5);
    assert_eq!(calc_struct.i128, 11);
}

#[test]
fn subtract_f64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_f64(5.0);
    assert_eq!(calc_struct.f64, 12.0);
}

#[test]
fn subtract_f32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.subtract_f32(5.0);
    assert_eq!(calc_struct.f32, 13.0);
}
