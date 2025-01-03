fn main() {
    let mut x = 5;
    { // Create a scope
        let y = &mut x;
        *y = 6;
    }
    { //Create another scope
        let z = &mut x; 
        *z = 7;
    }

    println!("{}", x); // This will print 7
    let a = &x; 
    println!("{}", *a); // this will print 7
} 