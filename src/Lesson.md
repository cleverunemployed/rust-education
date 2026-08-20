## 1. **Объекты (данные + поведение)**
В Rust структуры и перечисления содержат данные, а методы определяются в блоках `impl`.

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn bark(&self) {
        println!("{} says: Woof!", self.name);
    }
}

fn main() {
    let dog = Dog { name: "Rex".to_string(), age: 3 };
    dog.bark(); // Rex says: Woof!
}
```

---

## 2. **Инкапсуляция (скрытие деталей)**
С помощью `pub` можно управлять видимостью. Поля по умолчанию приватны.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,      // приватное поле
    average: f64,        // приватное поле
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {   // приватный метод
        self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}
```

Внешний код работает только через публичные методы `add` и `average`, внутреннее состояние скрыто.

---

## 3. **Наследование — заменяется типажами (traits)**
Rust не поддерживает наследование классов, но есть:
- **Реализация методов по умолчанию** в типажах (повторное использование кода)
- **Типажи-объекты** (полиморфизм)

```rust
trait Sound {
    fn make_sound(&self) {
        println!("Some sound"); // реализация по умолчанию
    }
}

struct Cat;
impl Sound for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

struct Robot;
impl Sound for Robot {} // использует реализацию по умолчанию

fn main() {
    let cat = Cat;
    let robot = Robot;
    cat.make_sound(); // Meow!
    robot.make_sound(); // Some sound
}
```

---

## 4. **Полиморфизм через типажи-объекты**
Вместо наследования — обобщённые типы и типажи-объекты (`dyn Trait`).

```rust
fn make_noise(animal: &dyn Sound) {
    animal.make_sound();
}

fn main() {
    let cat = Cat;
    let robot = Robot;
    make_noise(&cat);   // Meow!
    make_noise(&robot); // Some sound
}
```

---

## Вывод
| ООП-характеристика | Поддержка в Rust |
|-------------------|------------------|
| Объекты (данные + методы) | ✅ (структуры + impl) |
| Инкапсуляция | ✅ (pub / приватные поля) |
| Наследование | ❌ (заменяется типажами) |
| Полиморфизм | ✅ (типажи, обобщения, dyn Trait) |



# Краткое объяснение типаж-объектов в Rust

## Что такое типаж-объекты?

Типаж-объекты позволяют хранить значения **разных типов**, которые реализуют один и тот же типаж, в одной коллекции. Синтаксис: `Box<dyn Trait>` или `&dyn Trait`.

## Основная проблема

Векторы в Rust могут хранить только элементы **одного типа**. Но иногда нужно хранить разнотипные объекты с общим поведением.

## Решение

```rust
// Определяем типаж с общим поведением
pub trait Draw {
    fn draw(&self);
}

// Структура, хранящая типаж-объекты
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,  // типаж-объект
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // динамическая диспетчеризация
        }
    }
}
```

## Реализация разными типами

```rust
// Кнопка
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Рисую кнопку: {}", self.label);
    }
}

// Поле ввода
pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Рисую поле: {}", self.placeholder);
    }
}

// Пользовательский тип
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Рисую список выбора");
    }
}
```

## Использование

```rust
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(TextField {
                width: 30,
                height: 10,
                placeholder: String::from("Введите текст"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Да"),
                    String::from("Нет"),
                ],
            }),
        ],
    };
    
    screen.run();
    // Рисую кнопку: OK
    // Рисую поле: Введите текст  
    // Рисую список выбора
}
```

## Ключевые отличия

| **Обобщённые типы** | **Типаж-объекты** |
|-------------------|------------------|
| `struct Screen<T: Draw>` | `struct Screen { components: Vec<Box<dyn Draw>> }` |
| Один конкретный тип | Много разных типов |
| Статическая диспетчеризация | Динамическая диспетчеризация |
| Быстрее (мономорфизация) | Медленнее (поиск во время выполнения) |
| Компилятор знает все типы | Гибкость для пользователей |

## Ошибка при неверном типе

```rust
let screen = Screen {
    components: vec![
        Box::new(String::from("Привет")),  // ОШИБКА!
    ],
};
// String не реализует Draw
```

## Когда использовать?

- **Типаж-объекты**: когда нужна гибкость и коллекция может содержать разные типы
- **Обобщённые типы**: когда коллекция однородна и важна производительность


# Краткое объяснение паттерна "Состояние" в Rust

## Что такое паттерн "Состояние"?

Паттерн "Состояние" позволяет объекту изменять свое поведение при изменении его внутреннего состояния. Объект делегирует поведение объектам состояния, которые инкапсулируют логику для каждого состояния.

## Два подхода в Rust

### 1. Традиционный ООП-подход (с типаж-объектами)

```rust
// Определение состояний
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Конкретные состояния
struct Draft;
struct PendingReview;
struct Published;

// Структура поста
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }
    
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
}
```

**Преимущества**: 
- Инкапсуляция логики переходов
- Легко добавлять новые состояния
- Пользователь не знает о внутренних состояниях

**Недостатки**:
- Нарушение принципа владения (использование `Option`)
- Динамическая диспетчеризация

### 2. Type-State подход (безопасный во время компиляции)

```rust
// Разные типы для разных состояний
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

**Преимущества**:
- Невалидные состояния невозможны во время компиляции
- Нет необходимости в `Option`
- Безопасность типов

**Недостатки**:
- Больше кода для преобразований
- Пользователь должен знать о типах

## Пример использования

```rust
fn main() {
    // Традиционный подход
    let mut post = Post::new();
    post.add_text("Текст статьи");
    assert_eq!("", post.content()); // Пусто
    
    post.request_review();
    assert_eq!("", post.content()); // Все еще пусто
    
    post.approve();
    assert_eq!("Текст статьи", post.content()); // Опубликовано!
    
    // Type-State подход
    let mut draft = DraftPost::new();
    draft.add_text("Текст статьи");
    // draft.content() - ошибка компиляции!
    
    let pending = draft.request_review();
    // pending.content() - ошибка компиляции!
    
    let published = pending.approve();
    assert_eq!("Текст статьи", published.content());
}
```

## Когда использовать?

**Традиционный подход**:
- Когда состояния и переходы часто меняются
- Когда нужно скрыть детали от пользователя
- Когда важно минимизировать усилия по поддержке

**Type-State подход**:
- Когда нужна максимальная безопасность
- Когда состояния фиксированы и известны
- Когда важно предотвратить ошибки на этапе компиляции