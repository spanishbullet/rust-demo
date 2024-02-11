fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no semicolon, so this is an expression
    };
    println!("The value of y is: {}", y);
}
