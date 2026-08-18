// Base Tests

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


// assert! - этот макрос принимает в параметры bool выражение если true - ok, false - panik

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic] <- key moves
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// without panic
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// // ANCHOR: here
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() -> Result<(), String> {
//         let result = add(2, 2);

//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }


// $ cargo test -- --test-threads=1 однопоточный запуск
// $ cargo test - многопоточный запуск
// $ cargo test -- --show-output - показать вывод у проверяющих функций
// $ cargo test name_functions - запуск определённых тестов

    // #[test]
    // #[ignore]
    // fn expensive_test() - игнор теста


// Модульное тестирование
// тестирование каждого модуля по отдельности 
// pub fn add_two(a: u64) -> u64 {
//     internal_adder(a, 2)
// }

// fn internal_adder(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn internal() {
//         let result = internal_adder(2, 2);
//         assert_eq!(result, 4);
//     }
// }


// Интегрированное тестирование
// Тестирование модуля как одно целое
// adder
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     └── integration_test.rs
// use adder::add_two;

// #[test]
// fn it_adds_two() {
//     let result = add_two(2);
//     assert_eq!(result, 4);
// }
// cargo test --test integration_test