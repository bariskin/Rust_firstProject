
use std::thread;
use std::sync::{Mutex, Arc};


struct Philosopher {
       name: String,
       //left: usize,
       //right:usize,
}

impl Philosopher {
    //fn new(name: &str, left: usize, right: usize) -> Philosopher {
        fn new(name: &str) -> Philosopher {  
          Philosopher {
           name: name.to_string(),
          //  left: left,
          // right: right,
         }

     }

    fn eat(&self){
    
      //  let _left = table.forks[self.left].lock().unwrap();
       // let _right = table.forks[self.right].lock().unwrap();

        println!("{} начала есть.", self.name);                    /* пример вывода на печать */

        thread::sleep_ms(3000);

           println!("{} закончил есть.\n", self.name);
       }
   }


struct Table {
     forks: Vec<Mutex<()>>,
}
 
fn main() {

    let philosophers = vec![                                             /* пример создание вектора */
            Philosopher::new("Джудит Батлер"),
            Philosopher::new("Рая Дунаевская"),
            Philosopher::new("Зарубина Наталья"),
            Philosopher::new("Эмма Гольдман"),
            Philosopher::new("Анна Шмидт"),
          ];     


      /* 
         Объявляем новое связанное имя handles . Мы задали такое имя, потому что собираемся
         создать несколько потоков, в результате чего получим для них дескрипторы, с помощью
         которых сможем контролировать их выполнение. Здесь нам нужно явно указать тип, а зачем
         это необходимо, мы расскажем чуть позже. _ - это заполнитель типа. Мы говорим
         
         компилятору:« handles — это вектор, содержащий элементы, тип которых Rust должен вывести самостоятельно».
      */

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
            thread::spawn(move || {
                    p.eat();
            })
        }).collect();  

     for h in handles {
              h.join().unwrap();
            }   
        
    }