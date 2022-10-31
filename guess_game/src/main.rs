fn main() {
   println!("Guess the new Number!");

   println!("Kindly input your guess!");

   let mut guess = String::new();

   io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read this line");
    println!("You guessed: {guess}");
}
