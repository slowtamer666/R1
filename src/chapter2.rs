use std::io;

fn main() {
    println!("Введіть висоту ромба (наприклад, 5):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося зчитати рядок");
    
    let height: usize = input.trim().parse().expect("Введіть коректне число");

    // Верхня половина ромба
    for i in 0..height {
        // Друкуємо пробіли
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        // Друкуємо зірочки
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    // Нижня половина ромба
    for i in (0..height - 1).rev() {
        // Друкуємо пробіли
        for _ in 0..(height - i - 1) {
            print!(" ");
        }
        // Друкуємо зірочки
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

