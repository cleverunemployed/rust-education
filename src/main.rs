// Замыкание 

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

// Владение
// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let mut borrows_mutably = || list.push(7);

//     borrows_mutably();
//     println!("After calling closure: {list:?}");
// }


// move - ключевое слово для владение переменной
// use std::thread;

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(move || {
//         list.push(6);
//         println!("From thread: {list:?}");
//     })
//         .join()
//         .unwrap();
// }

// 1. FnOnce применяется к замыканиям, которые могут быть вызваны один раз. Все замыкания реализуют по 
// крайней мере этот трейт, потому что все замыкания могут быть вызваны. Замыкание, которое перемещает 
// захваченные значения из своего тела, реализует только FnOnce и ни один из других признаков Fn, потому 
// что оно может быть вызвано только один раз.

// 2. FnMut применяется к замыканиям, которые не перемещают захваченные значения из своего тела, но могут 
// изменять захваченные значения. Такие замыкания могут вызываться более одного раза.

// 3. Fn применяется к замыканиям, которые не перемещают захваченные значения из своего тела и не 
// модифицируют захваченные значения, а также к замыканиям, которые ничего не захватывают из своего окружения. 
// Такие замыкания могут выполняться более одного раза и не меняют ничего в своём окружении, что важно в таких 
// случаях, как одновременный вызов замыкания несколько раз.

// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }


// Итераторы

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();
// }

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     assert_eq!(v2, vec![2, 3, 4]);
// }

// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }
