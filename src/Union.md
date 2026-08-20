Unions

    Syntax
    Union :
       union IDENTIFIER Generics? WhereClause? {StructFields }

A union declaration uses the same syntax as a struct declaration, except with union in place of struct.


#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

The key property of unions is that all fields of a union share common storage. As a result writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field.

A value of a union type can be created using the same syntax that is used for struct types, except that it must specify exactly one field:


let u = MyUnion { f1: 1 };

The expression above creates a value of type MyUnion with active field f1. Active field of a union can be accessed using the same syntax as struct fields:

let f = u.f1;

Inactive fields can be accessed as well (using the same syntax) if they are sufficiently layout compatible with the current value kept by the union. Reading incompatible fields results in undefined behavior. However, the active field is not generally known statically, so all reads of union fields have to be placed in unsafe blocks.


unsafe {
    let f = u.f1;
}

Writes to Copy union fields do not require reads for running destructors, so these writes don't have to be placed in unsafe blocks


u.f1 = 2;

Commonly, code using unions will provide safe wrappers around unsafe union field accesses.

Another way to access union fields is to use pattern matching. Pattern matching on union fields uses the same syntax as struct patterns, except that the pattern must specify exactly one field. Since pattern matching accesses potentially inactive fields it has to be placed in unsafe blocks as well.


fn f(u: MyUnion) {
    unsafe {
        match u {
            MyUnion { f1: 10 } => { println!("ten"); }
            MyUnion { f2 } => { println!("{}", f2); }
        }
    }
