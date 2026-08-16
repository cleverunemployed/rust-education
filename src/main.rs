// Вот небольшая проблема программирования: напишите функцию, которая принимает строку слов,
// разделённых пробелами, и возвращает первое слово, которое она находит в этой строке.
// Если функция не находит пробела в строке, вся строка должна состоять из одного слова,
// поэтому должна быть возвращена вся строка.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}




// String - нереализует Copy и тем самым передаёт своим владением
// i, u, f, bool - реализуют Copy

// &s - неизменяемая ссылка сколько угодно
// &mut s - изменяемая ссылка только одна

// &str - это строковый срез
// &[start..end] - это срез, ссылка на элементы
