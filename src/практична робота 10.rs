fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;

    // Нормалізуємо зсув, щоб він був у межах довжини рядка
    let n = n.rem_euclid(len) as usize;

    // Обертаємо рядок
    let (left, right) = s.split_at(s.len() - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.to_string(), *n), 
                exp.to_string()
            );
        });
    }
}

fn main() {
    // Ви можете викликати вашу функцію тут, якщо це потрібно.
    let s = "abcdefgh".to_string();
    let n = 2;
    let rotated = rotate(s, n);
    println!("Rotated string: {}", rotated);
}
