mod config;
pub use config::CalcStruct;

#[test]
fn test_multiply_usize() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_usize(5);
    assert_eq!(calc_struct.usize, 5);
}

#[test]
fn test_multiply_u8() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_u8(5);
    assert_eq!(calc_struct.u8, 10);
}

#[test]
fn test_multiply_u16() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_u16(5);
    assert_eq!(calc_struct.u16, 15);
}

#[test]
fn test_multiply_u32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_u32(5);
    assert_eq!(calc_struct.u32, 20);
}

#[test]
fn test_multiply_u64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_u64(5);
    assert_eq!(calc_struct.u64, 25);
}

#[test]
fn test_multiply_u128() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_u128(5);
    assert_eq!(calc_struct.u128, 30);
}

#[test]
fn test_multiply_isize() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_isize(5);
    assert_eq!(calc_struct.isize, 35);
}

#[test]
fn test_multiply_i16() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_i16(5);
    assert_eq!(calc_struct.i16, 40);
}

#[test]
fn test_multiply_i32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_i32(5);
    assert_eq!(calc_struct.i32, 45);
}

#[test]
fn test_multiply_i64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_i64(5);
    assert_eq!(calc_struct.i64, 50);
}

#[test]
fn test_multiply_i128() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_i128(5);
    assert_eq!(calc_struct.i128, 55);
}

#[test]
fn test_multiply_f64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_f64(5.0);
    assert_eq!(calc_struct.f64, 60.0);
}

#[test]
fn test_multiply_f32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.multiply_f32(5.0);
    assert_eq!(calc_struct.f32, 65.0);
}
