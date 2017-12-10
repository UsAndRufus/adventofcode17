pub fn solve(captcha: &str) -> i32 {
    1
}


#[test]
fn test_1122() {
    assert_eq!(3, solve("1122"));
}
#[test]
fn test_1111() {
    assert_eq!(4, solve("1111"));
}
#[test]
fn test_1234() {
    assert_eq!(0, solve("1234"));
}
#[test]
fn test_91212129() {
    assert_eq!(9, solve("91212129"));
}
