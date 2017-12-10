use std::cmp::Ordering;

pub fn solve(captcha: &str) -> u32 {
    let n1 = get_digit(captcha, 0);
    let n2 = get_digit(captcha, 1);
    let mut sum = 0;
    sum += match n1.cmp(&n2) {
        Ordering::Equal => n1 + n2,
        _ => 0,
    };

    return sum;
}

fn get_digit(s: &str, i: usize) -> u32 {
    return s.chars().nth(i).unwrap().to_digit(10).unwrap();
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
