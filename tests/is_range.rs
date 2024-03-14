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

    assert!(calc.is_usize_range(0, 20).is_ok());
    assert!(calc.is_u8_range(0, 10).is_ok());
    assert!(calc.is_u16_range(0, 500).is_ok());
    assert!(calc.is_u32_range(0, 100000).is_ok());
    assert!(calc.is_u64_range(0, 10000000000).is_ok());
    assert!(calc.is_u128_range(0, 1000000000000000000000).is_ok());
    assert!(calc.is_isize_range(-10, 0).is_ok());
    assert!(calc.is_i16_range(-300, 0).is_ok());
    assert!(calc.is_i32_range(-50000, 0).is_ok());
    assert!(calc.is_i64_range(-3000000000, 0).is_ok());
    assert!(calc.is_i128_range(-1000000000000000000000, 0).is_ok());
    assert!(calc.is_f64_range(0.0, 2.0).is_ok());
    assert!(calc.is_f32_range(0.0, 1.0).is_ok());
}
