fn main() {
    label_loop();
    println!("returning: {}", returning());
    conditional_with_while();
    while_with_array();
    for_loop();
    for_range();
}

// label for loop
fn label_loop() -> (){
    let mut count = 0;
    'counting_up: loop{
        println!("count= {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

// returning values from loops: using break to return value from loops
fn returning() -> i32{
    let mut counter = 0;
    return loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }
}

// conditional loops with a while
fn conditional_with_while() ->(){
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

// while with array
fn while_with_array() -> (){
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("the value is: {}", a[i]);
        i += 1;
    }
}

// for_loop
fn for_loop() -> (){
    let a = [10, 20, 30, 40, 50];
    for ele in a {
        println!("{}", ele);
    }
}

// using rev() to reverse the range from 1 -> 4
fn for_range() -> (){
    for number in (1..4).rev(){
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
