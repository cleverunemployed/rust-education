// Vectors

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {third}");

//     let third: Option<&i32> = v.get(5);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }

//     for i in &mut v {
//         *i += 50;
//     }

//     for i in &v {
//         println!("{i}");
//     }
// }

// String


// fn main() {
//     for c in "Зд".chars() {
//         println!("{c}");
//     }
//     for b in "Зд".bytes() {
//         println!("{b}");
//     }
// }

// HashMap

// fn main() {
//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
// {"world": 2, "wonderful": 1, "hello": 1}
//     println!("{map:?}");
// }


// 1. Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: среднее значение;
// медиану (значение элемента из середины списка после его сортировки); моду списка (mode of list, 
// то значение которое встречается в списке наибольшее количество раз; HashMap будет полезна в данном случае).

// 2. Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin). Первая согласная каждого слова 
// перемещается в конец и к ней добавляется окончание "ay", так "first" станет "irst-fay". Слову, начинающемуся 
// на гласную, в конец добавляется "hay" ("apple" становится "apple-hay"). Помните о деталях работы с кодировкой 
// UTF-8!

// 3.Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю добавлять имена
// сотрудников к названию отдела компании. Например, "Add Sally to Engineering" или "Add Amir to Sales".
// Затем позвольте пользователю получить список всех людей из отдела или всех людей в компании,
// отсортированных по отделам в алфавитном порядке.
