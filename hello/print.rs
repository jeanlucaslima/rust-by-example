fn main() {
    // When printing, `{}` is replaced with any arguments, that will be stringfied
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32, if you wanna change the type 31 is
    // by providing a suffix. For example, to be an i64, type 31i64
    println!("{} days", 31i64);

    // There are various optional patterns this works with, for example:
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // It also can use named arguments
    println!("{subject} {verb} {object}",
                object = "the lazy dog",
                subject = "the quick brown fox",
                verb = "jumps over");
    
    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // The tutorial passes through this previous example without noticing anything, 
    // but if I understand, it reads 2 as binary and stringfy it? Interesting.

    // You can right-align text with a specified width. This will output 
    // "     1", 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can also numberpad with extra zeroes. This will output "000001".
    println!("{number:>0width$}",number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // oh my, the error messages are beautiful. 

    // This creates a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //println!("This struct `{}` won't print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
