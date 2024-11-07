fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Якщо загальна кількість вантажу не ділиться на кількість кораблів, повертаємо None
    if total % n != 0 {
        return None;
    }

    // Обчислюємо цільову середню вагу для кожного корабля
    let target = total / n;

    // Рахуємо сумарну кількість перенесень
    let mut balance = 0;
    let mut moves = 0;
    for &load in shipments {
        balance += load as i32 - target as i32;
        moves += balance.abs() as usize;
    }
    Some(moves)
}
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![4; n];
    let mut extra = 20 % n as u32;

    // Додаємо залишкову вагу до перших кораблів
    for i in 0..extra as usize {
        shipments[i] += 1;
    }
    shipments
}
fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    println!("Приклад 1: {:?}", count_permutation(&example1)); // Очікувано: Some(4)
    println!("Приклад 2: {:?}", count_permutation(&example2)); // Очікувано: Some(7)
}
