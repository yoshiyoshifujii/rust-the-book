use std::fs::File;

fn main() {
    // panicを起こす
    // panic!("crash and burn");

    // panic!バックトレースを使用する
    // let v = vec![1,2,3];
    // v[99];

    // Resultで回復可能なエラー
    // use std::io::ErrorKind;
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };
    // println!("{:?}", f);

    // エラー時にパニックするショートカット: unwrapとexpect
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:?}", f);

}

