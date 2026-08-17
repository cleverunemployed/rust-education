// panic! - критическая ошибка для выхода из программы
// [profile.release]
// panic = 'abort' - ускорение кода

// Result - обработка ошибки, которая не видёт к экстреннему завершению 

// enum Result<T, E> {
//     Ok(T), 
//     Err(E),
// }

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });


// ? - знак помогает для проброса ошибки, использовать при идентичной ошибке что и в Err
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// Box<dyn Error> - любая ошибка