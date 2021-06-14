use std::thread;
use std::time::Duration;

// 定义一个缓存结构体，将闭包作为结构体的构造参数传进去。
// 然后定义一个value用来缓存闭包获取的结果。
// 缺点就是无论传什么值，都只会获取第一个参数的结果
struct Cacche<T>
 where T: Fn(u32) -> u32 
 {
     calculating: T,
     value: Option<u32>,
}

impl<T> Cacche<T> 
where T: Fn(u32) -> u32
{
    fn new(calculating: T) -> Cacche<T> {
        Cacche {
            calculating, 
            value: None,
        }
    }

    fn value(&mut self, value: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculating)(value);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32)  {
    let mut expensive_closure = Cacche::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {}, pushups.", expensive_closure.value(intensity));
        println!("Next, do {}, sietups.", expensive_closure.value(intensity));
    } else {
        if random_number > 3 {
            println!("Take a break tody! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = 2;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}