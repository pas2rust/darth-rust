mod config;
pub use config::CalcStruct;

#[test]
fn is_range() {
    let calc = CalcStruct {
        usize: 10,
        u8: 5,
        u16: 300,
        u32: 70000,
        u64: 5000000000,
        u128: 100000000000000000000,
        isize: -5,
        i16: -200,
        i32: -40000,
        i64: -2000000000,
        i128: -500000000000000000000,
        f64: 1.23,
        f32: 0.12,
    };

    assert!(calc.is_range_usize(0, 20).is_ok());
    assert!(calc.is_range_u8(0, 10).is_ok());
    assert!(calc.is_range_u16(0, 500).is_ok());
    assert!(calc.is_range_u32(0, 100000).is_ok());
    assert!(calc.is_range_u64(0, 10000000000).is_ok());
    assert!(calc.is_range_u128(0, 1000000000000000000000).is_ok());
    assert!(calc.is_range_isize(-10, 0).is_ok());
    assert!(calc.is_range_i16(-300, 0).is_ok());
    assert!(calc.is_range_i32(-50000, 0).is_ok());
    assert!(calc.is_range_i64(-3000000000, 0).is_ok());
    assert!(calc.is_range_i128(-1000000000000000000000, 0).is_ok());
    assert!(calc.is_range_f64(0.0, 2.0).is_ok());
    assert!(calc.is_range_f32(0.0, 1.0).is_ok());
}
