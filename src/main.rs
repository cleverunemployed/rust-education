fn main() {
    let x = 9;
    let x = x + 3;

    {
        let x = x - 10;
        println!("x = {x}");
    }

    println!("x = {x}");

    let tup: (&str, u8, bool) = ("dfdfd", 90, true);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let victory: Vec<u8> = vec![];
    let victory: Vec<u8> = Vec::new();

    println!("x = {}", add_five(x));

    if true {
        println!("x = {}", add_five(7));
    } else {
        println!("x = {}", add_five(8));
    }

    'counter: loop {
        loop {
            if true {
                break 'counter;
            }
        }
    }

    for a in (1..11).rev() {
        print!("{a}->")
    }
}

fn add_five(x: i32) -> i32 {
    x + 5
}

// mut -> даёт возможность изменять переменную
// shading -> даёт возможность переписывать тип и значение переменной

// кортеж -> неизменяемый набор данных различных типов определённой длины
// массив -> неизменяемый набор данных одного типа определённой длины
// вектор -> изменяемый набор данных одного типа
