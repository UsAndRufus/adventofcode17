use std::cmp::Ordering;

pub fn solve(captcha: &str) -> u32 {

    let mut sum = 0;

    let captcha: Vec<u32> = convert_string_to_vec_of_u32s(captcha);

    for i in 0..(captcha.len() - 1) {
        let n1 = captcha[i];
        let n2 = captcha[i + 1];
        sum += match n1.cmp(&n2) {
            Ordering::Equal => n1,
            _ => 0,
        };
    }

    sum += match captcha[captcha.len() - 1].cmp(&captcha[0]) {
        Ordering::Equal => captcha[0],
        _ => 0,
    };

    println!("Sum is {}", sum);

    return sum;
}

fn convert_string_to_vec_of_u32s(s: &str) -> Vec<u32> {
    let mut v = Vec::new();

    for c in s.chars() {
        v.push(c.to_digit(10).unwrap());
    }

    return v;
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
