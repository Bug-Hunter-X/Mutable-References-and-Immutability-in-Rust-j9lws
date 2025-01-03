fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 6; 
    let z = &mut x; 
    *z = 7; 
    println!("{}", x); // This will print 7
    let a = &x; 
    // *a = 8; // This will cause compile time error because of immutability
    println!("{}", *a); // this will print 7
}