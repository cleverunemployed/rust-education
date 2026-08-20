# Краткое объяснение продвинутых типажей в Rust

## 1. Ассоциированные типы (Associated Types)

**Что это:** Типы-заполнители внутри трейта, которые конкретизируются при реализации.

**Зачем:** Позволяют трейту использовать типы без необходимости указывать их заранее.

```rust
// Определение трейта с ассоциированным типом
trait Iterator {
    type Item;  // Ассоциированный тип
    fn next(&mut self) -> Option<Self::Item>;
}

// Реализация с конкретным типом
impl Iterator for Counter {
    type Item = u32;  // Указываем конкретный тип
    
    fn next(&mut self) -> Option<Self::Item> {
        // реализация
    }
}
```

**Отличие от обобщений:** 
- С обобщениями можно реализовать трейт несколько раз для одного типа
- С ассоциированными типами - только один раз

```rust
// С обобщениями - можно несколько реализаций
impl Iterator<String> for Counter { /* ... */ }
impl Iterator<u32> for Counter { /* ... */ }

// С ассоциированными типами - только одна реализация
impl Iterator for Counter {
    type Item = u32;  // Только один тип
}
```

---

## 2. Параметры типа по умолчанию и перегрузка операторов

**Что это:** Возможность задать тип по умолчанию для обобщенного параметра.

```rust
use std::ops::Add;

// Определение трейта Add с параметром по умолчанию
trait Add<Rhs = Self> {  // Rhs по умолчанию = Self
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Простой случай - складываем одинаковые типы
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Сложный случай - разные типы
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}
```

---

## 3. Полностью квалифицированный синтаксис

**Что это:** Способ указать, какой именно метод вызывать при конфликте имен.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up, up, and away!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    
    // По умолчанию - метод самого типа
    person.fly();  // *waving arms furiously*
    
    // Указываем, какой трейт использовать
    Pilot::fly(&person);   // This is your captain speaking.
    Wizard::fly(&person);  // Up, up, and away!
    
    // Для статических методов:
    // <Тип as Трейт>::функция()
}
```

**Для статических методов:**

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // Полный синтаксис для статического метода
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // A baby dog is called a puppy
}
```

---

## 4. Супертрейты (Supertraits)

**Что это:** Трейт, который требует, чтобы тип реализовывал другой трейт.

```rust
use std::fmt::Display;

// OutlinePrint требует, чтобы тип реализовывал Display
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Используем Display
        let len = output.len();
        
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Сначала реализуем Display
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Затем OutlinePrint
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
```

---

## 5. Шаблон Newtype

**Что это:** Обёртка для внешнего типа, позволяющая реализовать внешние трейты.

```rust
use std::fmt;

// Создаём обёртку для Vec<String>
struct Wrapper(Vec<String>);

// Реализуем Display для обёртки
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    
    println!("w = {}", w);  // w = [hello, world]
}
```

**Обход правила сироты:**
- Нельзя реализовать внешний трейт для внешнего типа
- Newtype создаёт локальный тип, для которого можно реализовать что угодно

**Минусы:**
- Нужно делегировать методы вручную
- Или использовать `Deref` для автоматического делегирования

---

## Итог

| Концепция | Когда использовать |
|-----------|-------------------|
| Ассоциированные типы | Когда трейту нужен один конкретный тип на реализацию |
| Параметры по умолчанию | Для расширения трейтов без поломки кода или для удобства |
| Полный синтаксис | При конфликте имен методов из разных трейтов |
| Супертрейты | Когда один трейт зависит от функциональности другого |
| Newtype | Для реализации внешних трейтов для внешних типов |