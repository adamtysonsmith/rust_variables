fn main() {
    // Variable bindings are immutable by default
    let x = 4;
    // x = "Now I'm a string!"; COMPILE ERROR!!
    println!("x = {}", x);

    // Mutable bindings
    let mut orange = "green";
    println!("orange before mutation = {}", orange);

    orange = "orange";
    println!("orange after mutation = {}", orange);

    // Type annotations in variable bindings
    // let whatever: type = value;
    /* Types include:
        Numeric
        - i64, i32, i16, i8
        - u64, u32, u16, u8
        - isize, usize
        - f32, f64

        Non Numeric
        - bool
        - char (4 bytes)
        - str

        Data Structures
        - [T; N] (array)
        - &[T] (slice)
        - (T, T, etc) (tuples)

        https://doc.rust-lang.org/stable/book/primitive-types.html
    */
    let signed_sixty_four   : i64 = 64; // 64 bit signed integer
    let unsigned_sixty_four : u64 = 64; // 64 bit unsigned integer
    println!("Signed Sixty Four = {}", signed_sixty_four);
    println!("Unsigned Sixty Four = {}", unsigned_sixty_four);

    // Patterns
    // This is possible because it evaluates as an expression
    let (a, b) = ("apple", 2);
    println!("a = {}", a);
    println!("b = {}", b);

    // Shadowing
    // Allows variable rebinding (reinitialization)
    let something = 100;
    println!("something = {}", something);

    let something = "Now it's a string";
    println!("something is now = {}", something);

    // With shadowing you can make a variable mutable/immutable also
    let mut number: i32 = 900;
    println!("number is = {}", number);

    number = 7;
    println!("number is now = {}", number);

    let number = number; // Now number is immutable, and bound to 7
    println!("Immutable number = {}", number);

    // Scoping
    // You can use block scope {}
    // Functions are block scoped also
    let scoped = "Scope 1";
    {
        println!("Scoped is {}", scoped); // Prints Scope 1
        let scoped = "Scope 2";
        println!("Scoped is now {}", scoped); // Prints Scope 2
    }
    println!("Scoped is still {}", scoped); // Prints Scope 1

}
