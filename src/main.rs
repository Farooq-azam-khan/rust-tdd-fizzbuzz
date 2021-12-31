fn main() {
    println!("Hello, world!");
}
fn fizz_buzz(n: u32) -> String {
    let mut ret: String = String::from(""); 
    for i in 1..=n {
        let val: String = is_fizz_or_buzz(i);
        if i == n {
            ret = format!("{}{}",ret, val); 
        } else {
            ret = format!("{}{}\n", ret, val);
        }
    }
    ret
}

fn is_fizz_or_buzz(i: u32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string() 
    } else {
        i.to_string()
    }
}

#[test]
fn test_fizz_buzz_1() {
    assert_eq!(fizz_buzz(1), String::from("1")); 
}

#[test]
fn test_fizz_buzz_2() {
    assert_eq!(fizz_buzz(2), "1\n2".to_string());   
}

#[test]
fn test_fizz_buzz_3() {
    assert_eq!(fizz_buzz(3), "1\n2\nFizz".to_string());
}

#[test]
fn test_fizz_buzz_5() {
    assert_eq!(fizz_buzz(5), "1\n2\nFizz\n4\nBuzz".to_string()); 
}

#[test]
fn test_fizz_buzz_15() {
    assert_eq!(fizz_buzz(15), "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz");
}