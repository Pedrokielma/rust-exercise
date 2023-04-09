// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.



fn main() {
    let x = 100;
    let mut y = x;
    
    println!("print y {}", y);
    let z = &mut y;
    println!("print z {}", z);
    *z += 1000;
    println!("print z {}", z);
    println!("print x {}", x);
    assert_eq!(y, 1100);
}
