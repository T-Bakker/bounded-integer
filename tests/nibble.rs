#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[allow(dead_code)]
#[repr(u8)]
enum SNibble {
    N8 = 248, N7, N6, N5, N4, N3, N2, N1, U0 = 0, P1, P2, P3, P4, P5, P6, P7
}
bounded_integer_impls!(SNibble, i8, SNibble::N8, SNibble::P7);
bounded_integer_partial_ord_impl!(SNibble);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
#[repr(u8)]
enum NZUNibble {
    U1 = 1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15
}
bounded_integer_impls!(NZUNibble, u8, NZUNibble::U1, NZUNibble::U15);

#[test]
fn from_repr() {
    assert_eq!(Some(SNibble::U0), SNibble::from_repr(0i8));
    assert_eq!(Some(SNibble::N8), SNibble::from_repr(-8i8));
    assert_eq!(Some(SNibble::P7), SNibble::from_repr(7i8));
}

#[test]
fn to_repr() {
    assert_eq!(0i8, SNibble::U0.to_repr());
    assert_eq!(-8i8, SNibble::N8.to_repr());
    assert_eq!(7i8, SNibble::P7.to_repr());
}

#[test]
fn min_value() {
    assert_eq!(SNibble::N8, SNibble::min_value());
}

#[test]
fn max_value() {
    assert_eq!(SNibble::P7, SNibble::max_value());
}

#[test]
fn checked_add() {
    assert_eq!(Some(SNibble::P3), SNibble::P1.checked_add(SNibble::P2));
    assert_eq!(Some(SNibble::N3), SNibble::N1.checked_add(SNibble::N2));
    assert_eq!(Some(SNibble::N1), SNibble::P1.checked_add(SNibble::N2));
    assert_eq!(None, SNibble::P7.checked_add(SNibble::P1));
    assert_eq!(None, SNibble::N8.checked_add(SNibble::N1));
}

#[test]
fn checked_sub() {
    assert_eq!(Some(SNibble::P1), SNibble::P3.checked_sub(SNibble::P2));
    assert_eq!(Some(SNibble::N1), SNibble::N3.checked_sub(SNibble::N2));
    assert_eq!(Some(SNibble::P1), SNibble::N1.checked_sub(SNibble::N2));
    assert_eq!(None, SNibble::N8.checked_sub(SNibble::P1));
    assert_eq!(None, SNibble::P7.checked_sub(SNibble::N1));
}

#[test]
fn checked_mul() {
    assert_eq!(Some(SNibble::P6), SNibble::P2.checked_mul(SNibble::P3));
    assert_eq!(Some(SNibble::P6), SNibble::N2.checked_mul(SNibble::N3));
    assert_eq!(Some(SNibble::N6), SNibble::N2.checked_mul(SNibble::P3));
    assert_eq!(None, SNibble::P2.checked_mul(SNibble::P4));
    assert_eq!(None, SNibble::N2.checked_mul(SNibble::N4));
    assert_eq!(None, SNibble::N3.checked_mul(SNibble::P3));
}

#[test]
fn checked_div() {
    assert_eq!(Some(SNibble::P2), SNibble::P6.checked_div(SNibble::P3));
    assert_eq!(Some(SNibble::P2), SNibble::N6.checked_div(SNibble::N3));
    assert_eq!(Some(SNibble::N2), SNibble::N6.checked_div(SNibble::P3));
    assert_eq!(None, NZUNibble::U1.checked_div(NZUNibble::U2));
}

#[test]
fn checked_rem() {
    assert_eq!(Some(SNibble::P1), SNibble::P3.checked_rem(SNibble::P2));
    assert_eq!(Some(SNibble::N1), SNibble::N3.checked_rem(SNibble::P2));
    assert_eq!(None, NZUNibble::U2.checked_rem(NZUNibble::U2));
}

#[test]
fn checked_neg() {
    assert_eq!(Some(SNibble::N3), SNibble::P3.checked_neg());
    assert_eq!(Some(SNibble::P3), SNibble::N3.checked_neg());
    assert_eq!(Some(SNibble::U0), SNibble::U0.checked_neg());
    assert_eq!(None, SNibble::N8.checked_neg());
    assert_eq!(None, NZUNibble::U3.checked_neg());
}

#[test]
fn checked_add_repr() {
    assert_eq!(Some(SNibble::P3), SNibble::P1.checked_add_repr(2));
    assert_eq!(Some(SNibble::N3), SNibble::N1.checked_add_repr(-2));
    assert_eq!(Some(SNibble::N1), SNibble::P1.checked_add_repr(-2));
    assert_eq!(None, SNibble::P7.checked_add_repr(1));
    assert_eq!(None, SNibble::N8.checked_add_repr(-1));
}

#[test]
fn checked_sub_repr() {
    assert_eq!(Some(SNibble::P1), SNibble::P3.checked_sub_repr(2));
    assert_eq!(Some(SNibble::N1), SNibble::N3.checked_sub_repr(-2));
    assert_eq!(Some(SNibble::P1), SNibble::N1.checked_sub_repr(-2));
    assert_eq!(None, SNibble::N8.checked_sub_repr(1));
    assert_eq!(None, SNibble::P7.checked_sub_repr(-1));
}

#[test]
fn checked_mul_repr() {
    assert_eq!(Some(SNibble::P6), SNibble::P2.checked_mul_repr(3));
    assert_eq!(Some(SNibble::P6), SNibble::N2.checked_mul_repr(-3));
    assert_eq!(Some(SNibble::N6), SNibble::N2.checked_mul_repr(3));
    assert_eq!(None, SNibble::P2.checked_mul_repr(4));
    assert_eq!(None, SNibble::N2.checked_mul_repr(-4));
    assert_eq!(None, SNibble::N3.checked_mul_repr(3));
}

#[test]
fn checked_div_repr() {
    assert_eq!(Some(SNibble::P2), SNibble::P6.checked_div_repr(3));
    assert_eq!(Some(SNibble::P2), SNibble::N6.checked_div_repr(-3));
    assert_eq!(Some(SNibble::N2), SNibble::N6.checked_div_repr(3));
    assert_eq!(None, NZUNibble::U1.checked_div_repr(2));
}

#[test]
fn checked_rem_repr() {
    assert_eq!(Some(SNibble::P1), SNibble::P3.checked_rem_repr(2));
    assert_eq!(Some(SNibble::N1), SNibble::N3.checked_rem_repr(2));
    assert_eq!(None, NZUNibble::U2.checked_rem_repr(2));
}

#[test]
fn saturating_add() {
    assert_eq!(SNibble::P7, SNibble::P4.saturating_add(SNibble::P4));
    assert_eq!(SNibble::N8, SNibble::N4.saturating_add(SNibble::N5));
}

#[test]
fn saturating_sub() {
    assert_eq!(SNibble::P7, SNibble::P4.saturating_sub(SNibble::N4));
    assert_eq!(SNibble::N8, SNibble::N4.saturating_sub(SNibble::P5));
}

#[test]
fn saturating_mul() {
    assert_eq!(SNibble::P7, SNibble::P2.saturating_mul(SNibble::P4));
    assert_eq!(SNibble::P7, SNibble::N2.saturating_mul(SNibble::N4));
    assert_eq!(SNibble::N8, SNibble::N2.saturating_mul(SNibble::P5));
    assert_eq!(SNibble::N8, SNibble::P5.saturating_mul(SNibble::N2));
}

#[test]
fn saturating_add_repr() {
    assert_eq!(SNibble::P7, SNibble::P4.saturating_add_repr(4));
    assert_eq!(SNibble::N8, SNibble::N4.saturating_add_repr(-5));
}

#[test]
fn saturating_sub_repr() {
    assert_eq!(SNibble::P7, SNibble::P4.saturating_sub_repr(-4));
    assert_eq!(SNibble::N8, SNibble::N4.saturating_sub_repr(5));
}

#[test]
fn saturating_mul_repr() {
    assert_eq!(SNibble::P7, SNibble::P2.saturating_mul_repr(4));
    assert_eq!(SNibble::P7, SNibble::N2.saturating_mul_repr(-4));
    assert_eq!(SNibble::N8, SNibble::N2.saturating_mul_repr(5));
    assert_eq!(SNibble::N8, SNibble::P5.saturating_mul_repr(-2));
}

#[test]
fn into_repr() {
    assert_eq!(0i8, SNibble::U0.into());
    assert_eq!(-8i8, SNibble::N8.into());
    assert_eq!(7i8, SNibble::P7.into());
}

#[test]
fn add_self() {
    assert_eq!(SNibble::P3, SNibble::P1 + SNibble::P2);
    assert_eq!(SNibble::P3, SNibble::P1 + &SNibble::P2);
    assert_eq!(SNibble::P3, &SNibble::P1 + SNibble::P2);
    assert_eq!(SNibble::P3, &SNibble::P1 + &SNibble::P2);
}

#[test]
#[should_panic]
fn add_self_overflow() {
    let _ = SNibble::P7 + SNibble::P1;
}
