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
fn sum_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_usize(5);
    assert_eq!(calc_struct.usize, 6);
}

#[test]
fn sum_u8() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_u8(5);
    assert_eq!(calc_struct.u8, 7);
}

#[test]
fn sum_u16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_u16(5);

    assert_eq!(calc_struct.u16, 8);
}

#[test]
fn sum_u32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_u32(5);

    assert_eq!(calc_struct.u32, 9);
}

#[test]
fn sum_u64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_u64(5);

    assert_eq!(calc_struct.u64, 10);
}

#[test]
fn sum_u128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_u128(5);
    assert_eq!(calc_struct.u128, 11);
}

#[test]
fn sum_isize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_isize(5);
    assert_eq!(calc_struct.isize, 12);
}

#[test]
fn sum_i16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_i16(5);

    assert_eq!(calc_struct.i16, 13);
}

#[test]
fn sum_i32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_i32(5);

    assert_eq!(calc_struct.i32, 14);
}

#[test]
fn sum_i64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_i64(5);

    assert_eq!(calc_struct.i64, 15);
}

#[test]
fn sum_i128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_i128(5);
    assert_eq!(calc_struct.i128, 16);
}

#[test]
fn sum_f64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_f64(5.0);

    assert_eq!(calc_struct.f64, 17.0);
}

#[test]
fn sum_f32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.sum_f32(5.0);

    assert_eq!(calc_struct.f32, 18.0);
}
