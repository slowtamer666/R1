fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}

fn main() {
    // Ви можете викликати вашу функцію тут, якщо це потрібно.
    let numbers = [123, 121, 1221];
    for &num in &numbers {
        println!("Is {} a palindrome? {}", num, is_palindrome(num));
    }
}
