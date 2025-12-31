fn main() {
    // Immutable is like a Python constant BUT can be redefined by the same name later IF there is
    // a Let (so it is 'Shadowed')
    // Mutable is a variable
    // Const is immutable & can't be redefined later: Convention is all caps
    let x = 5;
    println!("x = {x}");
    let mut x = 6;          // This line throws a warning but it shows mutable can be changed later
    println!("x = {x}");

    // Below is fine despite the type changing due to shadowing
    // Mutable spaces would error as spaces would always be a str
    let spaces = "   ";
    let spaces = spaces.len();
    
}
