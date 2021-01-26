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
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // println!("{:?}", f);

    // エラーを委譲する
    // use std::io;
    // use std::io::Read;
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    //     let mut s = String::new();
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }
    // read_username_from_file().unwrap();

    // エラー委譲のショートカット: ?演算子
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    read_username_from_file().unwrap();
}

