mod config;
pub use config::CalcStruct;

#[test]
fn test_discount_usize() {
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
    calc_struct.discount_usize(20.0);
    calc_struct.print_err_usize("");
    assert_eq!(calc_struct.usize, 8);
}

#[test]
fn test_discount_u8() {
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
    calc_struct.discount_u8(20.0);
    calc_struct.print_err_u8("");
    assert_eq!(calc_struct.u8, 16);
}

#[test]
fn test_discount_u16() {
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
    calc_struct.discount_u16(20.0);
    calc_struct.print_err_u16("");
    assert_eq!(calc_struct.u16, 24);
}

#[test]
fn test_discount_u32() {
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
    calc_struct.discount_u32(20.0);
    calc_struct.print_err_u32("");
    assert_eq!(calc_struct.u32, 32);
}

#[test]
fn test_discount_u64() {
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
    calc_struct.discount_u64(20.0);
    calc_struct.print_err_u64("");
    assert_eq!(calc_struct.u64, 40);
}

#[test]
fn test_discount_u128() {
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
    calc_struct.discount_u128(20.0);
    calc_struct.print_err_u128("");
    assert_eq!(calc_struct.u128, 48);
}

#[test]
fn test_discount_isize() {
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
    calc_struct.discount_isize(20.0);
    calc_struct.print_err_isize("");
    assert_eq!(calc_struct.isize, 56);
}

#[test]
fn test_discount_i16() {
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
    calc_struct.discount_i16(20.0);
    calc_struct.print_err_i16("");
    assert_eq!(calc_struct.i16, 64);
}

#[test]
fn test_discount_i32() {
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
    calc_struct.discount_i32(20.0);
    calc_struct.print_err_i32("");
    assert_eq!(calc_struct.i32, 72);
}

#[test]
fn test_discount_i64() {
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
    calc_struct.discount_i64(20.0);
    calc_struct.print_err_i64("");
    assert_eq!(calc_struct.i64, 80);
}

#[test]
fn test_discount_i128() {
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
    calc_struct.discount_i128(20.0);
    calc_struct.print_err_i128("");
    assert_eq!(calc_struct.i128, 88);
}

#[test]
fn test_discount_f64() {
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
    calc_struct.discount_f64(20.0);
    calc_struct.print_err_f64("");
    assert_eq!(calc_struct.f64, 96.0);
}

#[test]
fn test_discount_f32() {
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
    calc_struct.discount_f32(20.0);
    calc_struct.print_err_f32("");
    assert_eq!(calc_struct.f32, 104.0);
}
