const WIDTH: usize = 20;
const HEIGHT: usize = 10;

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 {
                // Малюємо рамку конверта
                output.push('*');
            } else if i == j || j == WIDTH - i - 1 {
                // Діагональні лінії конверта
                output.push('*');
            } else {
                // Порожній простір всередині
                output.push(' ');
            }
        }
        output.push('\n');
    }

    // Вивід усієї побудованої форми за допомогою однієї команди println!
    println!("{}", output);
}
