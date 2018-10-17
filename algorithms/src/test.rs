
/*
Testing Macro
macro_rules! zoom_and_enhance {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }

            fn field_types() -> &'static [&'static str] {
                 static NAMES: &'static [&'static str] = &[$(stringify!($ftype)),*];
                 NAMES
            }

        }
    }
}

zoom_and_enhance!{
struct Export {
        first_name: String,
        last_name: String,
        gender: String,
        date_of_birth: String,
        address: String
    }
}

fn main() {
    println!("{:?}", Export::field_names());
    println!("{:?}", Export::field_types());
}

*/


/*
fn double<F>(val1: i32, val2: i32, f: F)
    where F: Fn(i32, i32) -> std::cmp::Ordering {
    match f(val1, val2) {
        std::cmp::Ordering::Less => print!("Lesser"),
        _ => print!("Equal or greater")
    }
}

fn a(){
    println!("calling a");
}

Testing Trait as Interface
pub trait A {
    fn abc(&self);
}

struct A1 {
    name: String
}

impl A for A1 {
    fn abc(&self) {
        println!("{}", self.name);
    }
}

struct B1 {
    id: i32
}

impl A for B1 {
    fn abc(&self) {
        println!("{}", self.id);
    }
}


fn main() {
    let mut abcd = Vec::new();
    abcd.push(Box::new(A1 { name: "Abrar".into() }) as Box<A>);
    abcd.push(Box::new(B1 { id: 5 }) as Box<A>);
    for x in abcd.iter() {
        x.abc();
    }
    double(15, 10, |x: i32, y: i32| x.cmp(&y));

    let fn_arr: Vec<fn()> = vec![a, || {println!("calling b")}];
    for fun in fn_arr.iter(){
        fun();
    }
}
*/