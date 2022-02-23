mod channel;
mod mutex;

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn joinハンドルで全スレッドの終了を待つ() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn スレッドでmoveクロージャを使用する() {
        let v = vec![1, 2, 3];
        let v2 = v.clone();

        let handle = thread::spawn(move || {
            println!("Here is a vector: {:?}", v);
        });

        println!("Here is cloned vector: {:?}", v2);

        handle.join().unwrap();
    }
}
