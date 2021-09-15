// Keep in mind that Rust is a statically typed language, 
// which means that it must know the types of all variables at compile time
fn main() {
    
    // addition
    let sum = 5 + 10;
    println!("sum = {}", sum);


    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);

    // division
    let quotient = 56.7/32.2;
    println!("quotient = {}", quotient);

    //remainder
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);

    // bool
    let t = true;
    println!("{}", t);
    let t: bool = false;
    println!("{}", t);

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // tuple
    let tup = (1, 1.2, 'a');
    let (x, y, z) = tup; // destructoring
    println!("tup[0] = {}", x);
    println!("tup[1] = {}", y);
    println!("tup[2] = {}", z);

    let tup1: (i8, i32, i64) = (1, 2, 4);
    println!("tup1[0] = {}", tup1.0);

    // Array
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    println!("length of months = {}", months.len());

    let arr: [i64; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    let arr1 = [3; 5]; // same let arr1 = [3, 3, 3, 3, 3]
    println!("arr1[1] = {}", arr1[1]);

}
