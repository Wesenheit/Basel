use basel::number::modulo::*;
use basel::threadpool::Pool;
use basel::time_it;
#[test]
fn tonelli_shanks_test() {
    time_it!("test",let result = tonelli_shanks(5,41));
    assert_eq!(result, 28);
}

#[test]
fn test_para() {
    static N: u64 = 1024;
    static P: u64 = 17;
    static A: u64 = 3;
    let range = 1..N;
    let pool = Pool::new(4);
    let lambda = |x: u64| mod_pow(A, x, A);
    pool.reduce_mod(lambda, range.into_iter(), P);
}
