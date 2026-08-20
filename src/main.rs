поле для хранения значения i32 в куче:

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}

Поскольку Box<T> является указателем, Rust всегда знает, сколько места нужно Box<T>: размер указателя не 
меняется в зависимости от объёма данных, на которые он указывает. Это означает, что мы можем поместить Box<T> 
внутрь экземпляра Cons вместо значения List напрямую. Box<T> будет указывать на значение очередного List, 
который будет находиться в куче, а не внутри экземпляра Cons. Концептуально у нас все ещё есть список, 
созданный из списков, содержащих другие списки, но эта реализация теперь больше похожа на размещение элементов 
рядом друг с другом, а не внутри друг друга.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


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


Как разыменованное приведение взаимодействует с изменяемостью
Подобно тому, как вы используете типаж Deref для переопределения оператора * у неизменяемых ссылок, 
вы можете использовать типаж DerefMut для переопределения оператора * у изменяемых ссылок.

Rust выполняет разыменованное приведение, когда находит типы и реализации типажей в трёх случаях:

Из типа &T в тип &U когда верно T: Deref<Target=U>
Из типа &mut T в тип &mut U когда верно T: DerefMut<Target=U>
Из типа &mut T в тип &U когда верно T: Deref<Target=U>
Первые два случая идентичны друг другу, за исключением того, что второй реализует изменяемость. В первом 
случае говорится, что если у вас есть &T, а T реализует Deref для некоторого типа U, вы сможете прозрачно 
получить &U. Во втором случае говорится, что такое же разыменованное приведение происходит и для изменяемых 
ссылок.

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


fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}


enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// --snip--

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
