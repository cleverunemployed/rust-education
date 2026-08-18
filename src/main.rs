// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// } -> только для float

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }



// Traits

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }

//     fn get(&self) -> ();
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {}

// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!()
// }

// fn returns_summarizable(switch: bool) -> impl Summary {

// impl<T: Display> ToString for T {
//     // --snip--
// } -- общая реализация для всех кто реализует Display


// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }


// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest_with_an_announcement(
//         string1.as_str(),
//         string2,
//         "Today is someone's birthday!",
//     );
//     println!("The longest string is {result}");
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {ann}");
//     if x.len() > y.len() { x } else { y }
// }

// Каждая ссылка имеет время жизни, и оно не может быть короче объекта, на который ссылается.

// Если в функции одна входная ссылка, то выходная ссылка (результат) получит её время жизни.

// Если в функции несколько входных ссылок, и одна из них — &self или &mut self (метод структуры),
// то выходная ссылка получает время жизни self.