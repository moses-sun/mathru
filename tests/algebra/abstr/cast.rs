use crate::mathru::algebra::abstr::cast::FromPrimitive;
use mathru::algebra::abstr::cast::ToPrimitive;

#[test]
fn u32_from_i8() {
    let v = u32::from_i8(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_i16() {
    let v = u32::from_i16(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_i32() {
    let v = u32::from_i32(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_i64() {
    let v = u32::from_i64(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_i128() {
    let v = u32::from_i128(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_u8() {
    let v = u32::from_u8(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_u16() {
    let v = u32::from_u16(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_u32() {
    let v = u32::from_u32(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_u64() {
    let v = u32::from_u64(13);

    assert_eq!(13, v);
}

#[test]
fn u32_from_u128() {
    let v = u32::from_u128(13);

    assert_eq!(13, v);
}

#[test]
fn u32_to_i8() {
    let v = 13u32.to_i8();

    assert_eq!(13, v);
}

#[test]
fn u32_to_i16() {
    let v = 13u32.to_i16();

    assert_eq!(13, v);
}

#[test]
fn u32_to_i32() {
    let v = 13u32.to_i32();

    assert_eq!(13, v);
}

#[test]
fn u32_to_i64() {
    let v = 13u32.to_i64();

    assert_eq!(13, v);
}

#[test]
fn u32_to_i128() {
    let v = 13u32.to_i128();

    assert_eq!(13, v);
}

#[test]
fn u32_to_u8() {
    let v = 13u32.to_u8();

    assert_eq!(13, v);
}

#[test]
fn u32_to_u16() {
    let v = 13u32.to_u16();

    assert_eq!(13, v);
}

#[test]
fn u32_to_u32() {
    let v = 13u32.to_u32();

    assert_eq!(13, v);
}

#[test]
fn u32_to_u64() {
    let v = 13u32.to_u64();

    assert_eq!(13, v);
}

#[test]
fn u32_to_u128() {
    let v = 13u32.to_u128();

    assert_eq!(13, v);
}

#[test]
fn u32_to_f32() {
    let v = 13u32.to_f32();

    assert_eq!(13.0, v);
}

#[test]
fn u32_to_f64() {
    let v = 13u32.to_f64();

    assert_eq!(13.0, v);
}

#[test]
fn i32_from_i8() {
    let v = i32::from_i8(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_i16() {
    let v = i32::from_i16(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_i32() {
    let v = i32::from_i32(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_i64() {
    let v = i32::from_i64(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_i128() {
    let v = i32::from_i128(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_u8() {
    let v = i32::from_u8(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_u16() {
    let v = i32::from_u16(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_u32() {
    let v = i32::from_u32(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_u64() {
    let v = i32::from_u64(13);

    assert_eq!(13, v);
}

#[test]
fn i32_from_u128() {
    let v = i32::from_u128(13);

    assert_eq!(13, v);
}

#[test]
fn i32_to_i8() {
    let v = 13i32.to_i8();

    assert_eq!(13, v);
}

#[test]
fn i32_to_i16() {
    let v = 13i32.to_i16();

    assert_eq!(13, v);
}

#[test]
fn i32_to_i32() {
    let v = 13i32.to_i32();

    assert_eq!(13, v);
}

#[test]
fn i32_to_i64() {
    let v = 13i32.to_i64();

    assert_eq!(13, v);
}

#[test]
fn i32_to_i128() {
    let v = 13i32.to_i128();

    assert_eq!(13, v);
}

#[test]
fn i32_to_u8() {
    let v = 13i32.to_u8();

    assert_eq!(13, v);
}

#[test]
fn i32_to_u16() {
    let v = 13i32.to_u16();

    assert_eq!(13, v);
}

#[test]
fn i32_to_u32() {
    let v = 13i32.to_u32();

    assert_eq!(13, v);
}

#[test]
fn i32_to_u64() {
    let v = 13i32.to_u64();

    assert_eq!(13, v);
}

#[test]
fn i32_to_u128() {
    let v = 13i32.to_u128();

    assert_eq!(13, v);
}

#[test]
fn i32_to_f32() {
    let v = 13i32.to_f32();

    assert_eq!(13.0, v);
}

#[test]
fn i32_to_f64() {
    let v = 13i32.to_f64();

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_i8() {
    let v = f64::from_i8(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_i16() {
    let v = f64::from_i16(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_i32() {
    let v = f64::from_i32(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_i64() {
    let v = f64::from_i64(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_i128() {
    let v = f64::from_i128(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_u8() {
    let v = f64::from_u8(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_u16() {
    let v = f64::from_u16(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_u32() {
    let v = f64::from_u32(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_u64() {
    let v = f64::from_u64(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_from_u128() {
    let v = f64::from_u128(13);

    assert_eq!(13.0, v);
}

#[test]
fn f64_to_i8() {
    let v = 13.0f64.to_i8();

    assert_eq!(13, v);
}

#[test]
fn f64_to_i16() {
    let v = 13.0f64.to_i16();

    assert_eq!(13, v);
}

#[test]
fn f64_to_i32() {
    let v = 13.0f64.to_i32();

    assert_eq!(13, v);
}

#[test]
fn f64_to_i64() {
    let v = 13.0f64.to_i64();

    assert_eq!(13, v);
}

#[test]
fn f64_to_i128() {
    let v = 13.0f64.to_i128();

    assert_eq!(13, v);
}

#[test]
fn f64_to_u8() {
    let v = 13.0f64.to_u8();

    assert_eq!(13, v);
}

#[test]
fn f64_to_u16() {
    let v = 13.0f64.to_u16();

    assert_eq!(13, v);
}

#[test]
fn f64_to_u32() {
    let v = 13.0f64.to_u32();

    assert_eq!(13, v);
}

#[test]
fn f64_to_u64() {
    let v = 13.0f64.to_u64();

    assert_eq!(13, v);
}

#[test]
fn f64_to_u128() {
    let v = 13.0f64.to_u128();

    assert_eq!(13, v);
}

#[test]
fn f64_to_f32() {
    let v = 13f64.to_f32();

    assert_eq!(13.0, v);
}

#[test]
fn f64_to_f64() {
    let v = 13f64.to_f64();

    assert_eq!(13.0, v);
}
