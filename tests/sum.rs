mod config;
pub use config::CalcStruct;

#[test]
fn test_sum_usize() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_usize(5);
    calc_struct.print_err_usize();
    assert_eq!(calc_struct.usize, 6);
}

#[test]
fn test_sum_u8() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_u8(5);
    calc_struct.print_err_u8();
    assert_eq!(calc_struct.u8, 7);
}

#[test]
fn test_sum_u16() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_u16(5);
    calc_struct.print_err_u16();
    assert_eq!(calc_struct.u16, 8);
}

#[test]
fn test_sum_u32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_u32(5);
    calc_struct.print_err_u32();
    assert_eq!(calc_struct.u32, 9);
}

#[test]
fn test_sum_u64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_u64(5);
    calc_struct.print_err_u64();
    assert_eq!(calc_struct.u64, 10);
}

#[test]
fn test_sum_u128() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_u128(5);
    calc_struct.print_err_u128();
    assert_eq!(calc_struct.u128, 11);
}

#[test]
fn test_sum_isize() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_isize(5);
    calc_struct.print_err_isize();
    assert_eq!(calc_struct.isize, 12);
}

#[test]
fn test_sum_i16() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_i16(5);
    calc_struct.print_err_i16();
    assert_eq!(calc_struct.i16, 13);
}

#[test]
fn test_sum_i32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_i32(5);
    calc_struct.print_err_i32();
    assert_eq!(calc_struct.i32, 14);
}

#[test]
fn test_sum_i64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_i64(5);
    calc_struct.print_err_i64();
    assert_eq!(calc_struct.i64, 15);
}

#[test]
fn test_sum_i128() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_i128(5);
    calc_struct.print_err_i128();
    assert_eq!(calc_struct.i128, 16);
}

#[test]
fn test_sum_f64() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_f64(5.0);
    calc_struct.print_err_f64();
    assert_eq!(calc_struct.f64, 17.0);
}

#[test]
fn test_sum_f32() {
    let mut calc_struct =
        CalcStruct::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12.0, 13.0);
    calc_struct.sum_f32(5.0);
    calc_struct.print_err_f32();
    assert_eq!(calc_struct.f32, 18.0);
}
