fn find_solutions() -> Vec<(u32, u32, u32, u32, u32, u32, u32, u32)> {
    let mut solutions = Vec::new();

    for m in 1..=9 {
        for u in 0..=9 {
            if u == m { continue; }
            for x in 1..=9 {
                if x == m || x == u { continue; }
                for a in 0..=9 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=9 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 0..=9 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 0..=9 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 0..=9 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    // Обчислення чисел за буквами
                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let xa = 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    // Перевірка рівняння
                                    if muxa * xa == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

fn main() {
    let solutions = find_solutions();

    // Вивід результатів у потрібному форматі
    for (m, u, x, a, s, l, o, n) in &solutions {
        println!("  {}{}{}{}", m, u, x, a);
        println!("x     {}{}", x, a);
        println!("  ------");
        println!("    {}{}{}{}", s, l, o, n);
        println!();
    }

    println!("Кількість рішень: {}", solutions.len());
}
