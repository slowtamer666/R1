use std::cmp::{max, min};

// Представлення точки на площині
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Представлення прямокутника
#[derive(Clone, Copy)]
struct Rectangle {
    a: Point, // ліва верхня точка
    b: Point, // права нижня точка
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Функція, яка рахує фактичну зайняту площу
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    // Структура події для лінії сканування
    struct Event {
        x: i32,
        y1: i32,
        y2: i32,
        kind: i32, // 1 для відкриття, -1 для закриття
    }

    // Зібрати всі події
    let mut events = Vec::new();
    for rect in xs {
        events.push(Event { x: rect.a.x, y1: rect.a.y, y2: rect.b.y, kind: 1 });
        events.push(Event { x: rect.b.x, y1: rect.a.y, y2: rect.b.y, kind: -1 });
    }

    // Сортувати події за x-координатою
    events.sort_by(|a, b| a.x.cmp(&b.x));

    let mut occupied_area = 0;
    let mut prev_x = events[0].x;
    let mut y_intervals = vec![];

    // Проходимо по всіх подіях
    for event in events {
        // Довжина y-інтервалів для попередньої x-координати
        let mut y_covered = 0;
        let mut current_y = -1;

        for &(y_start, y_end) in &y_intervals {
            if y_start > current_y {
                y_covered += y_end - y_start;
                current_y = y_end;
            } else if y_end > current_y {
                y_covered += y_end - current_y;
                current_y = y_end;
            }
        }

        // Додати площу для попереднього x-інтервалу
        occupied_area += y_covered * (event.x - prev_x);

        // Оновити список y-інтервалів
        if event.kind == 1 {
            y_intervals.push((event.y1, event.y2));
            y_intervals.sort(); // підтримуємо відсортований порядок
        } else {
            y_intervals.retain(|&(y_start, y_end)| !(y_start == event.y1 && y_end == event.y2));
        }

        prev_x = event.x;
    }

    occupied_area
}

// Тест функції
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test пройдено!");
}
