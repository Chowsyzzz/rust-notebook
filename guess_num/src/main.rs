use core::error;

use rand::Rng;

fn main() {
    println!("Welcome to the guessing!");

    let secret_num = rand::thread_rng().gen_range(1, 101); // generate random number
    println!("The secret number is {secret_num}!");

    loop {
      println!("Guess the number:");
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).expect("failed to read");
  
      // trim去除空格 parse字符串数字转换
      let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      match input.cmp(&secret_num) {
          std::cmp::Ordering::Less => println!("Too small."),
          std::cmp::Ordering::Equal => {
            println!("You win.");
            break;
          },
          std::cmp::Ordering::Greater => println!("Too big."),
      }
    }
}
