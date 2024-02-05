mod config;
pub use config::CalcStruct;

#[test]
fn test_inflate_usize() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_usize(20.0);
    calc_struct.print_err_usize("");
    assert_eq!(calc_struct.usize, 12);
}

#[test]
fn test_inflate_u8() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_u8(20.0);
    calc_struct.print_err_u8("");
    assert_eq!(calc_struct.u8, 24);
}

#[test]
fn test_inflate_u16() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_u16(20.0);
    calc_struct.print_err_u16("");
    assert_eq!(calc_struct.u16, 36);
}

#[test]
fn test_inflate_u32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_u32(20.0);
    calc_struct.print_err_u32("");
    assert_eq!(calc_struct.u32, 48);
}

#[test]
fn test_inflate_u64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_u64(20.0);
    calc_struct.print_err_u64("");
    assert_eq!(calc_struct.u64, 60);
}

#[test]
fn test_inflate_u128() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_u128(20.0);
    calc_struct.print_err_u128("");
    assert_eq!(calc_struct.u128, 72);
}

#[test]
fn test_inflate_isize() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_isize(20.0);
    calc_struct.print_err_isize("");
    assert_eq!(calc_struct.isize, 84);
}

#[test]
fn test_inflate_i16() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_i16(20.0);
    calc_struct.print_err_i16("");
    assert_eq!(calc_struct.i16, 96);
}

#[test]
fn test_inflate_i32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_i32(20.0);
    calc_struct.print_err_i32("");
    assert_eq!(calc_struct.i32, 108);
}

#[test]
fn test_inflate_i64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_i64(20.0);
    calc_struct.print_err_i64("");
    assert_eq!(calc_struct.i64, 120);
}

#[test]
fn test_inflate_i128() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_i128(20.0);
    calc_struct.print_err_i128("");
    assert_eq!(calc_struct.i128, 132);
}

#[test]
fn test_inflate_f64() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_f64(20.0);
    calc_struct.print_err_f64("");
    assert_eq!(calc_struct.f64, 144.0);
}

#[test]
fn test_inflate_f32() {
    let mut calc_struct = CalcStruct::new(
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120.0, 130.0,
    );
    calc_struct.inflate_f32(20.0);
    calc_struct.print_err_f32("");
    assert_eq!(calc_struct.f32, 156.0);
}
