use core::fmt;
use std::fmt::Debug;


fn main() {
    println!("Hello world!");
    println!("I'm a Rustacean!");
    println!("I am {} old", 34);
    println!("Norvald is {age} old", age = 1);
    println!("Marte is {age:>5} old", age = 30);
    println!("Pad {0} zeros to {number:0>5}", 5, number = 42);
    println!("{number:0>width$}", number=1, width=5);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
    // Ugly to use fmt::Debug so its better to implement fmt::Display
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        } 
    }

    let integer_one = 1;
    println!("Structure can now pretty print {}", Structure(integer_one));
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 mod 256 is : {}", 1000 % 256);
}

