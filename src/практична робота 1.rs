use std::f64::consts::PI;
use std::io;

fn main() {
    // Введення значення x
    println!("Введіть значення x:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося зчитати рядок");
    
    // Перетворення введеного значення в число
    let x: f64 = input.trim().parse().expect("Введіть коректне число");

    // Перевірка, щоб sin(x) не дорівнював нулю
    if x % PI == 0.0 {
        println!("Помилка: значення sin(x) не може дорівнювати нулю.");
        return;
    }

    // Обчислення y
    let y = (1.0 / (x.powi(2) + 2.0).tan() + (x.powi(2) - 8.0).exp()) / x.sin();

    // Виведення результату
    println!("Значення y: {:.5}", y);
}

