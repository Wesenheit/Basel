use basel::number::core::*;
use basel::number::modulo::*;
#[test]
fn is_prime_test() {
    let result: bool = is_prime(7);
    assert_eq!(result, true);
}

#[test]
fn test_pow() {
    let result: u64 = mod_pow(4, 13, 497);
    assert_eq!(result, 445);
}

#[test]
fn tonelli_shanks_test() {
    let result: u64 = tonelli_shanks(5, 41);
    assert_eq!(result, 28);
}

#[test]
fn legandr_test() {
    let result: u64 = legandre(5, 41);
    assert_eq!(result, 1);
}

#[test]
fn test_inverse() {
    let result: u64 = inverse(13, 16);
    assert_eq!(result, 5);
}
