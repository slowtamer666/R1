const HEIGHT: usize = 5;  // Висота ялинки

fn main() {
    // Малюємо ялинку
    for i in 0..HEIGHT {
        // Пробіли для центрованого вирівнювання ялинки
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        
        // Зірочки для кожного рівня ялинки
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        
        println!();
    }
    for i in 0..HEIGHT {
        // Пробіли для центрованого вирівнювання ялинки
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        
        // Зірочки для кожного рівня ялинки
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        
        println!();
    }for i in 0..HEIGHT {
        // Пробіли для центрованого вирівнювання ялинки
        for _ in 0..(HEIGHT - i - 1) {
            print!(" ");
        }
        
        // Зірочки для кожного рівня ялинки
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        
        println!();
    }
    // Малюємо стовбур розміром 2 на 3
    for _ in 0..3 {
        // Пробіли для центрованого вирівнювання стовбура
        for _ in 0..(HEIGHT - 2) {
            print!(" ");
        }
        println!("**");
    }
}


