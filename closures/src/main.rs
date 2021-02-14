use std::time::Duration;
use std::thread;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    {
        fn add_one_v1(x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1};
        let add_one_v3 = |x| { x + 1 };
        let add_one_v4 = |x| x + 1;

        add_one_v1(1);
        add_one_v2(1);
        add_one_v3(1);
        add_one_v4(1);
    }

    // 2つの異なる型で型が推論されるクロージャの呼び出しを試みる
    {
        let example_closure = |x| x;
        example_closure(String::from("hello"));
        // example_closure(5); // ←ここでコンパイルエラー
    }
}

#[cfg(test)]
mod tests {
    use crate::Cacher;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);
        // ↓以下を通すためには、Cacherの保持するvalueをOptionからMapに変えるといい
        // let v2 = c.value(2);
        // assert_eq!(v2, 2);
    }

    // クロージャで環境をキャプチャする
    #[test]
    fn capture_env() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));

        // 以下はコンパイルエラー
        // 関数で上記と同じようなことはできないため
        // {
        //     let x = 4;
        //     fn equal_to_x(z: i32) -> bool { z == x}
        //     let y = 4;
        //     assert!(equal_to_x(y));
        // }
    }

    // クロージャが使用している値の所有権を奪うことをクロージャに強制する場合
    #[test]
    fn move_to_closure() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x); // xがclosureに所有権移動したのに呼び出そうとしているからここでコンパイルエラー
        let y  = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}