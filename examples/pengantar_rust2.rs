//guesing game
use std::io;
fn main(){
    println!("guess the number");
    println!("please input the number");

    let mut guess= String::new();

    io::stdin()
    .read_line(& mut guess); 
    println!("guess")
}