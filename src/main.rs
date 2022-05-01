fn main() {
    for x in 1..=100 {
        fizz_buzz(x);
    }
}

fn fizz_buzz(x: i32) {
    match x {
        x if x % 3 == 0 && x % 5 == 0 => println!("fizzbuzz"),
        x if x % 3 == 0 => println!("fizz"),
        x if x % 5 == 0 => println!("buzz"),
        _ => println!("{}", x),
    }
}