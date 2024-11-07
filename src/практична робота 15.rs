fn gray(n: u8) -> Vec<String> {
    // Якщо n == 0, повертаємо порожній вектор з єдиним порожнім рядком
    if n == 0 {
        return vec!["".to_string()];
    }

    // Рекурсивно отримуємо попередні значення для n - 1
    let prev = gray(n - 1);
    let mut result = Vec::new();

    // Додаємо '0' до початку кожного рядка для першої половини
    for code in &prev {
        result.push(format!("0{}", code));
    }

    // Додаємо '1' до початку кожного рядка для другої половини (реверсованої)
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "11", "10"]),
            (3, vec!["000", "001", "011", "010", 
                     "110", "111", "101", "100"]),
            (4, vec!["0000", "0001", "0011", "0010", 
                     "0110", "0111", "0101", "0100", 
                     "1100", "1101", "1111", "1110",
                     "1010", "1011", "1001", "1000"]),
        ];

        for (n, expected) in test_data.iter() {
            assert_eq!(gray(*n), *expected);
        }
    }
}

fn main() {
    // Запускаємо тест
    tests::test();
    println!("Test пройдено!");
}
