* Указатель — это общая концепция для переменной, которая содержит адрес участка памяти. Этот адрес «относится к», или «указывает на» некоторые другие данные. Наиболее общая разновидность указателя в Rust — это ссылка
* Box<T> для распределения значений в куче (памяти)
* Rc<T> тип счётчика ссылок, который допускает множественное владение
* Типы Ref<T> и RefMut<T>, доступ к которым осуществляется через тип RefCell<T>, который обеспечивает правила заимствования во время выполнения вместо времени компиляции

## Box<T>
* Наиболее простой умный указатель - это box, чей тип записывается как Box<T>. Такие переменные позволяют хранить данные в куче, а не в стеке.
### Вы будете использовать его чаще всего в следующих ситуациях:
1. Когда у вас есть тип, размер которого невозможно определить во время компиляции, а вы хотите использовать значение этого типа в контексте, требующем точного размера.
2. Когда у вас есть большой объем данных и вы хотите передать владение, но при этом быть уверенным, что данные не будут скопированы
3. Когда вы хотите получить значение во владение и вас интересует только то, что оно относится к типу, реализующему определённый трейт, а не то, является ли оно значением какого-то конкретного типа

Пример:
```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

### Рекурсивное применение
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## Deref
* Используя трейт Deref, вы можете изменить поведение оператора разыменования * (не путать с операторами умножения или глобального подключения). Реализовав Deref таким образом, что умный указатель может рассматриваться как обычная ссылка, вы можете писать код, оперирующий ссылками, а также использовать этот код с умными указателями.

Пример:
```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Пример по отбрасывания Box если объект реализует Deref
```rust

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Без Deref код выглядел бы иначе:
```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

### Rust выполняет разыменованное приведение, когда находит типы и реализации типажей в трёх случаях:

1. Из типа &T в тип &U когда верно T: Deref<Target=U>
2. Из типа &mut T в тип &mut U когда верно T: DerefMut<Target=U>
3. Из типа &mut T в тип &U когда верно T: Deref<Target=U>


## Drop

* Вторым важным типажом умного указателя является Drop, который позволяет регулировать, что происходит, когда значение вот-вот выйдет из области видимости. Вы можете реализовать типаж Drop для любого типа, а также использовать этот код для высвобождения ресурсов, таких как файлы или сетевые соединения.

Пример: 
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}
```

* Функция std::mem::drop отличается от метода drop трейта Drop. Мы вызываем её, передавая в качестве аргумента значение, которое хотим принудительно уничтожить. Функция находится в прелюдии, поэтому мы можем изменить main в листинге 15-15 так, чтобы вызвать функцию drop, как показано в листинге 15-16:

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
```

## Rc<T>

* Вы должны включить множественное владение явно, используя тип Rust Rc<T>, который является аббревиатурой для подсчёта ссылок. Тип Rc<T> отслеживает количество ссылок на значение, чтобы определить, используется ли оно ещё. Если ссылок на значение нет, значение может быть очищено и при этом ни одна ссылка не станет недействительной.

* Тип Rc<T> используется, когда мы хотим разместить в куче некоторые данные для чтения несколькими частями нашей программы и не можем определить во время компиляции, какая из частей завершит использование данных последней. Если бы мы знали, какая часть завершит использование последней то, мы могли бы сделать эту часть владельцем данных и вступили бы в силу обычные правила владения, применяемые во время компиляции.

* Обратите внимание, что Rc<T> используется только в однопоточных сценариях.

Пример:
```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

## RefCell<T>

* Внутренняя изменяемость - это паттерн проектирования Rust, который позволяет вам изменять данные даже при наличии неизменяемых ссылок на эти данные; обычно такое действие запрещено правилами заимствования. Для изменения данных паттерн использует unsafe код внутри структуры данных, чтобы обойти обычные правила Rust, регулирующие изменяемость и заимствование. Небезопасный (unsafe) код даёт понять компилятору, что мы самостоятельно следим за соблюдением этих правил, а не полагаемся на то, что компилятор будет делать это для нас

* В отличие от Rc<T> тип RefCell<T> предоставляет единоличное владение данными, которые он содержит. В чем же отличие типа RefCell<T> от Box<T>? 

* С помощью ссылок и типа Box<T> инварианты правил заимствования применяются на этапе компиляции. С помощью RefCell<T> они применяются во время работы программы. Если вы нарушите эти правила, работая с ссылками, то будет ошибка компиляции. Если вы работаете с RefCell<T> и нарушите эти правила, то программа вызовет панику и завершится.

* Тип RefCell<T> полезен, когда вы уверены, что ваш код соответствует правилам заимствования, но компилятор не может понять и гарантировать этого.

* Вот список причин выбора типов Box<T>, Rc<T> или RefCell<T>:

1. Тип Rc<T> разрешает множественное владение одними и теми же данными; типы Box<T> и RefCell<T> разрешают иметь единственных владельцев.
2. Тип Box<T> разрешает неизменяемые или изменяемые владения, проверенные при компиляции; тип Rc<T> разрешает только неизменяемые владения, проверенные при компиляции; тип RefCell<T> разрешает неизменяемые или изменяемые владения, проверенные во время выполнения.
3. Поскольку RefCell<T> разрешает изменяемые заимствования, проверенные во время выполнения, можно изменять значение внутри RefCell<T> даже если RefCell<T> является неизменным.

Пример:
```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

Пример использования Rc<T> и RefCell<T>:

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```

## Ссылочное зацикливание

Пример цикла:

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

```

### Предотвращение ссылочной зацикленности: замена умного указателя Rc<T> на Weak<T>
* До сих пор мы демонстрировали, что вызов Rc::clone увеличивает strong_count экземпляра Rc<T>, а экземпляр Rc<T> удаляется, только если его strong_count равен 0. Вы также можете создать слабую ссылку на значение внутри экземпляра Rc<T>, вызвав Rc::downgrade и передав ссылку на Rc<T>. Сильные ссылки - это то с помощью чего вы можете поделиться владением экземпляра Rc<T>. Слабые ссылки не отражают связи владения, и их подсчёт не влияет на то, когда экземпляр Rc<T> будет очищен. Они не приведут к ссылочному циклу, потому что любой цикл, включающий несколько слабых ссылок, будет разорван, как только количество сильных ссылок для задействованных значений станет равным 0.

* Когда вы вызываете Rc::downgrade, вы получаете умный указатель типа Weak<T>. Вместо того чтобы увеличить strong_count в экземпляре Rc<T> на 1, вызов Rc::downgrade увеличивает weak_count на 1. Тип Rc<T> использует weak_count для отслеживания количества существующих ссылок Weak<T>, аналогично strong_count. Разница в том, что weak_count не должен быть равен 0, чтобы экземпляр Rc<T> мог быть удалён.

* Поскольку значение, на которое ссылается Weak<T> могло быть удалено, то необходимо убедиться, что это значение все ещё существует, чтобы сделать что-либо со значением на которое указывает Weak<T>. Делайте это вызывая метод upgrade у экземпляра типа Weak<T>, который вернёт Option<Rc<T>>. Вы получите результат Some, если значение Rc<T> ещё не было удалено и результат None, если значение Rc<T> было удалено. Поскольку upgrade возвращает тип Option<T>, Rust обеспечит обработку обоих случаев Some и None и не будет некорректного указателя.

Пример:
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

```

Пример изменения:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

```