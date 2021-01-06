fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 変数とデータの相互作用:ムーブ
    {
        let x = 5;
        let y = x;
        println!("{}", y);

        let s1 = String::from("hello");
        println!("{}, world!", s1);
        let s2 = s1;
        println!("{}, world!", s2);
    }

    // 変数とデータの相互作用:クローン
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    // 所有権と関数
    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("{}", s); ここではもう有効ではない

        let x = 5;
        makes_copy(x);
        println!("{}", x); // i32はCopyなので使っても大丈夫

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        fn makes_copy(some_integer: i32) {
            println!("{}", some_integer);
        }
    }

    // 戻り値とスコープ
    {
        let s1 = gives_ownership();
        println!("{}", s1);

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
        println!("{}", s3);

        fn gives_ownership() -> String {
            let some_string = String::from("hello");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }
    }
}
