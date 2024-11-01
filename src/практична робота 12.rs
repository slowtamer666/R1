use rand::Rng; // Для генерації випадкових чисел

// Генерація випадкового вектора з 20 значеннями в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

// Знаходження мінімальної сусідньої пари у векторі
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize, usize)> {
    if data.len() < 2 {
        return None;
    }
    
    let mut min_sum = i32::MAX;
    let mut indices = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            indices = (i, i + 1);
        }
    }

    Some((min_sum, indices.0, indices.1))
}

// Вивід даних у зрозумілому форматі
fn print_data(data: &[i32]) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{} ", i);
    }
    println!();

    // Вивід даних
    print!("data:    [");
    for (i, &value) in data.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", value);
    }
    println!("]");

    // Знаходження мінімальної сусідньої пари
    if let Some((min_sum, i, j)) = min_adjacent_sum(data) {
        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            data[i], data[j], min_sum, i, j
        );

        // Вивід індексів з підкресленнями
        print!("indexes: ");
        for k in 0..data.len() {
            if k == i {
                print!("\\__ ");
            } else if k == j {
                print!("__/");
            } else {
                print!("   ");
            }
        }
        println!();
    }
}

fn main() {
    // Генерація і вивід випадкових векторів
    for _ in 0..4 {
        let data = gen_random_vector(20);
        print_data(&data);
        println!();
    }
}
