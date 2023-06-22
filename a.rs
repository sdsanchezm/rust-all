fn main() {
    println!("Hallo Welt!");
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C
    println!("{number:>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
    // error here:
    // println!("My name is {0}, {1} {0}", "Bond");
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
