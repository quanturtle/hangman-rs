# hangman-rs
sample hangman built in rust to get the hang of it

## Basics

### define function
```
// hello.rs
fn main() {
    println!("Hello, World!");
}
```
### compile and run
```
rustc hello.rs
./hello
> Hello, World!
```

### `for` loop
```
fn main() {
    for i in 0..5 {
        println!("Hello {}", i);
    }
}
```

### declaring variables  
Note: `mut` and casting (`as`)
```
fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}
```

### functions types are explicit 
```
fn sqr(x: f64) -> f64 {
    x * x
}

fn main() {
    let squared_n = sqr(2)
    println!("sqr is {}", squared_n);
}
```

### mutable references
```
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}
```

### arrays
```
fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }

    println!("length {}", arr.len());
}
```

### array slicing generates views of underlying array
Note: `:?` = debug print
```
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}
```

### optional values (`Some()` or `None`)
```
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };
}
```
or
```
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    
    let maybe_last = slice.get(5);
    if let Some(last) = maybe_last {
        last.unwrap()
    } else {
        -1
    };
}
```
or
```
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    
    let maybe_last = slice.get(5);
    match maybe_last {
        Some(last) => *maybe_last.unwrap()
        None => -1
    };
}
```

which can be expressed as
```
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let last = *slice.get(5).unwrap_or(&-1);
}
```
safe unwrap with `unwrap_or()`
```
let value = some_option.unwrap_or(default_value);
```


# `Result<T, E>` type
```
enum MathError {
    DivisionByZero
} 

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    }
    Ok(a / b)
}

fn main() -> Result<(), MathError> {
    let first_result = divide(10, 2)?;
    let second_result = divide(1, 0)?;
    println!("{}", first_result)
    println!("{}", second_result)

    Ok(())
}
```
the `?` operator propagates errors up the stack
```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```
use `ok_or()` to wrap error values and return `Err(err)`
converts a `Result<T, E>` into a `Result<T, F>`, where `F` is the type of the provided error (err) argument
```
let new_result = result.ok_or(error_value);
```


### vectors
```
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
```
```
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
```

### iterators
```
fn main() {
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}
```

### command line args
```
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}
```

```
rustc args0.rs
./args0 42 'hello dolly' frodo
```

```
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if 0 < args.len() { // we have args!
        ...
    }
}
```

### `match`
```
fn main() {
    let n: i32 = 5;
    let text = match n {
        0...3 => "small",
        4...6 => "medium",
        _ => "large",
     };
}
```

### tuples
Note: tuples may contain different types
```
fn add_mul(x: f64, y: f64) -> (f64,f64) {
    (x + y, x * y)
}

fn main() {
    let t = add_mul(2.0,10.0);

    // can debug print
    println!("t {:?}", t);

    // can 'index' the values
    println!("add {} mul {}", t.0,t.1);

    // can _extract_ values
    let (add,mul) = t;
    println!("add {} mul {}", add,mul);
}
```

### `struct`
`impl` defines struct methods  
`&self` to pass self reference to function  
`&mut self` to pass self references to be modified by function  
`#[derive(Debug)]` naive debug representation of struct
```
use std::fmt;

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{}", self.full_name())
    }
}

fn main() {
    let p = Person::new("John","Smith");
    println!("person {} {}", p.first_name,p.last_name);
    
    println!("full_name {} {}", p.full_name());
    
    p.set_first_name("George");
    println!("full_name {} {}", p.full_name());

    println!("tuple {}", p.to_tuple());

    println!("{:?}", p);
}
```



### traits
```
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer: i32 = 42;
    let maybe_pi: f64 = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}
```