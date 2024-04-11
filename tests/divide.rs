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
fn divide_usize() {
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
    calc_struct.divide_usize(5);

    assert_eq!(calc_struct.usize, 2);
}

#[test]
fn divide_u8() {
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
    calc_struct.divide_u8(5);
    assert_eq!(calc_struct.u8, 4);
}

#[test]
fn divide_u16() {
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
    calc_struct.divide_u16(5);
    assert_eq!(calc_struct.u16, 6);
}

#[test]
fn divide_u32() {
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
    calc_struct.divide_u32(5);
    assert_eq!(calc_struct.u32, 8);
}

#[test]
fn divide_u64() {
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
    calc_struct.divide_u64(5);
    assert_eq!(calc_struct.u64, 10);
}

#[test]
fn divide_u128() {
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
    calc_struct.divide_u128(5);
    assert_eq!(calc_struct.u128, 12);
}

#[test]
fn divide_isize() {
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
    calc_struct.divide_isize(5);

    assert_eq!(calc_struct.isize, 14);
}

#[test]
fn divide_i16() {
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
    calc_struct.divide_i16(5);
    assert_eq!(calc_struct.i16, 16);
}

#[test]
fn divide_i32() {
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
    calc_struct.divide_i32(5);
    assert_eq!(calc_struct.i32, 18);
}

#[test]
fn divide_i64() {
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
    calc_struct.divide_i64(5);
    assert_eq!(calc_struct.i64, 20);
}

#[test]
fn divide_i128() {
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
    calc_struct.divide_i128(5);
    assert_eq!(calc_struct.i128, 22);
}

#[test]
fn divide_f64() {
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
    calc_struct.divide_f64(5.0);
    assert_eq!(calc_struct.f64, 24.0);
}

#[test]
fn divide_f32() {
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
    calc_struct.divide_f32(5.0);
    assert_eq!(calc_struct.f32, 26.0);
}
