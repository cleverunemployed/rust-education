# Краткое объяснение макросов в Rust

## Что такое макросы?
Макросы - это способ метапрограммирования, позволяющий писать код, который генерирует другой код во время компиляции.

## Отличия макросов от функций
- **Макросы** могут принимать переменное число аргументов
- **Макросы** раскрываются до компиляции, функции - во время выполнения
- **Макросы** объявляются до использования, функции - в любом месте

## 1. Декларативные макросы (`macro_rules!`)

Самый распространённый тип, работает по принципу сопоставления с образцом.

```rust
// Пример упрощённого макроса vec!
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Использование
let v = vec![1, 2, 3];  // Создаёт вектор [1, 2, 3]
let v2 = vec![1, 2, 3, 4, 5];  // Работает с любым количеством элементов
```

## 2. Процедурные макросы

Принимают `TokenStream` как вход и возвращают `TokenStream` как выход.

### А) Выводимые (`derive`) макросы
Генерируют реализации типажей для структур и перечислений.

```rust
// В крейте hello_macro_derive
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Использование пользователем
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();  // Выведет: Hello, Macro! My name is Pancakes!
}
```

### Б) Похожие на атрибуты макросы
Могут применяться к любым элементам (функции, структуры, модули).

```rust
// Определение
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr - содержимое атрибута: GET, "/"
    // item - тело элемента: fn index() {}
    // Генерируем код...
}

// Использование
#[route(GET, "/")]
fn index() {
    // ...
}
```

### В) Похожие на функции макросы
Похожи на вызов функций, но работают с `TokenStream`.

```rust
// Определение
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Парсим и проверяем SQL-запрос
    // Генерируем код
}

// Использование
let query = sql!(SELECT * FROM users WHERE id = 1);
```

## Ключевые крейты для процедурных макросов
- **`proc_macro`** - API для работы с токенами
- **`syn`** - парсинг Rust кода в структуры данных
- **`quote`** - генерация Rust кода из структур

## Практический пример
```rust
// Файл Cargo.toml для крейта с макросом
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"

// Код макроса
#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Обработка входных данных
    // Генерация кода
}
```

## Когда использовать макросы?
- Когда нужна генерация повторяющегося кода
- Для создания DSL (предметно-ориентированных языков)
- Для автоматической реализации типажей
- Для обработки кода во время компиляции