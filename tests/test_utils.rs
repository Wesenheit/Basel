use basel::*;
use basel::number::modulo::*;

#[test]
fn tonelli_shanks_test(){
    time_it!(let result = tonelli_shanks(5,41));
    assert_eq!(result,28);
}