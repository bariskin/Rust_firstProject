use std::io;  // lib use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

println!("Угадайте число!");

let secret_number = rand::thread_rng().gen_range(1, 101);

println!("Загаданное число: {}", secret_number);

loop {

  println!("Пожалуйста, введите предположение.");

  let mut guess = String::new();

   io::stdin()
   .read_line(&mut guess)
   .ok()
   .expect("Не удалось прочитать строку");     // io::stdin().read_line(&mut guess).ok().expect("Не удалось прочитать строку");

   let guess: u32 = guess.trim().parse()       // string convert to int
   .ok()
   .expect("Пожалуйста, введите число!");

   println!("Ваша попытка: {}", guess);

   match guess.cmp(&secret_number) {            // сравнение чисел
      Ordering::Less => println!("Слишком маленькое!"),
      Ordering::Greater => println!("Слишком большое!"),
      Ordering::Equal => {
                          println!("Вы выиграли!");
                          break;
    }
  }
 }
}