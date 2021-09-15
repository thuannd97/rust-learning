fn main() {
    println!("Hello, world!");
    another_function();

    let x = 5;
    let y = {
        //let x = 1;
        x + 1
    };
    println!("y = {}", y);
    
    // return value
    println!("fn five = {}", five(12));
}

fn another_function(){
    println!("another function!");
}

fn five(x: i64) -> i64 {
    x + 12
}