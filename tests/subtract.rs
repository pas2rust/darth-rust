mod config;
pub use config::CalcStruct;

#[test]
fn test_subtract_usize() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_usize(5);
    calc_struct.print_err_usize();
    assert_eq!(calc_struct.usize, 1);
}

#[test]
fn test_subtract_u8() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_u8(5);
    calc_struct.print_err_u8();
    assert_eq!(calc_struct.u8, 2);
}

#[test]
fn test_subtract_u16() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_u16(5);
    calc_struct.print_err_u16();
    assert_eq!(calc_struct.u16, 3);
}

#[test]
fn test_subtract_u32() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_u32(5);
    calc_struct.print_err_u32();
    assert_eq!(calc_struct.u32, 4);
}

#[test]
fn test_subtract_u64() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_u64(5);
    calc_struct.print_err_u64();
    assert_eq!(calc_struct.u64, 5);
}

#[test]
fn test_subtract_u128() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_u128(5);
    calc_struct.print_err_u128();
    assert_eq!(calc_struct.u128, 6);
}

#[test]
fn test_subtract_isize() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_isize(5);
    calc_struct.print_err_isize();
    assert_eq!(calc_struct.isize, 7);
}

#[test]
fn test_subtract_i16() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_i16(5);
    calc_struct.print_err_i16();
    assert_eq!(calc_struct.i16, 8);
}

#[test]
fn test_subtract_i32() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_i32(5);
    calc_struct.print_err_i32();
    assert_eq!(calc_struct.i32, 9);
}

#[test]
fn test_subtract_i64() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_i64(5);
    calc_struct.print_err_i64();
    assert_eq!(calc_struct.i64, 10);
}

#[test]
fn test_subtract_i128() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_i128(5);
    calc_struct.print_err_i128();
    assert_eq!(calc_struct.i128, 11);
}

#[test]
fn test_subtract_f64() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_f64(5.0);
    calc_struct.print_err_f64();
    assert_eq!(calc_struct.f64, 12.0);
}

#[test]
fn test_subtract_f32() {
    let mut calc_struct =
        CalcStruct::new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17.0, 18.0);
    calc_struct.subtract_f32(5.0);
    calc_struct.print_err_f32();
    assert_eq!(calc_struct.f32, 13.0);
}
