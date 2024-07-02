use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};
fn main() {
    println!("Nhap vao:");
    let secrect_number = rand::thread_rng().gen_range(1..=100);
    println!("The secrect number is = {}", secrect_number);
    let mut guess = String::new(); //Variable mutabel(co the thay doi duoc)
    let apple = 5; // Variable immutable khong the thay doi duoc.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed : {guess}");
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Loi trong qua trinh thuc hienj");
    match guess.cmp(&secrect_number) {
        Ordering::Less => println!("To small !"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("To big"),
    }
    println!("Apple: {apple}");
    println!("x = {}, y= {}", guess, apple);
}
