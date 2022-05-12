use std::io;  // lib use std::io;

fn main() {

println!("Угадайте число!");
println!("Пожалуйста, введите предположение.");

let mut guess = String::new();

io::stdin()
.read_line(&mut guess)
.ok()
.expect("Не удалось прочитать строку");     // io::stdin().read_line(&mut guess).ok().expect("Не удалось прочитать строку");

println!("Ваша попытка: {}", guess);

}