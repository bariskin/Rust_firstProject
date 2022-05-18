
use std::thread;

struct Philosopher {
       name: String,
}

    impl Philosopher {

        fn new(name: &str) -> Philosopher {
           Philosopher {
           name: name.to_string(),
          }
       }

       fn eat(&self){
           println!("{} начала есть.", self.name);                    /* пример вывода на печать */

           thread::sleep_ms(1000);

           println!("{} закончил есть.\n", self.name);
       }
   }
 
fn main() {

    let philosophers = vec![                                    /* пример создание вектора */
            Philosopher::new("Джудит Батлер"),
            Philosopher::new("Рая Дунаевская"),
            Philosopher::new("Зарубина Наталья"),
            Philosopher::new("Эмма Гольдман"),
            Philosopher::new("Анна Шмидт"),
          ];     

        for p in &philosophers {
              p.eat();
         }    
    }