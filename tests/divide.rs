mod config;
pub use config::CalcStruct;

#[test]
fn test_divide_usize() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_usize(5);
    calc_struct.print_err_usize("");
    assert_eq!(calc_struct.usize, 2);
}

#[test]
fn test_divide_u8() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_u8(5);
    calc_struct.print_err_u8("");
    assert_eq!(calc_struct.u8, 4);
}

#[test]
fn test_divide_u16() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_u16(5);
    calc_struct.print_err_u16("");
    assert_eq!(calc_struct.u16, 6);
}

#[test]
fn test_divide_u32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_u32(5);
    calc_struct.print_err_u32("");
    assert_eq!(calc_struct.u32, 8);
}

#[test]
fn test_divide_u64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_u64(5);
    calc_struct.print_err_u64("");
    assert_eq!(calc_struct.u64, 10);
}

#[test]
fn test_divide_u128() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_u128(5);
    calc_struct.print_err_u128("");
    assert_eq!(calc_struct.u128, 12);
}

#[test]
fn test_divide_isize() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_isize(5);
    calc_struct.print_err_isize("");
    assert_eq!(calc_struct.isize, 14);
}

#[test]
fn test_divide_i16() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_i16(5);
    calc_struct.print_err_i16("");
    assert_eq!(calc_struct.i16, 16);
}

#[test]
fn test_divide_i32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_i32(5);
    calc_struct.print_err_i32("");
    assert_eq!(calc_struct.i32, 18);
}

#[test]
fn test_divide_i64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_i64(5);
    calc_struct.print_err_i64("");
    assert_eq!(calc_struct.i64, 20);
}

#[test]
fn test_divide_i128() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_i128(5);
    calc_struct.print_err_i128("");
    assert_eq!(calc_struct.i128, 22);
}

#[test]
fn test_divide_f64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_f64(5.0);
    calc_struct.print_err_f64("");
    assert_eq!(calc_struct.f64, 24.0);
}

#[test]
fn test_divide_f32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.divide_f32(5.0);
    calc_struct.print_err_f32("");
    assert_eq!(calc_struct.f32, 26.0);
}
