use basel::number::modulo::*;
use basel::time_it;
#[test]
fn tonelli_shanks_test(){
    time_it!("test",let result = tonelli_shanks(5,41));
    assert_eq!(result,28);
}