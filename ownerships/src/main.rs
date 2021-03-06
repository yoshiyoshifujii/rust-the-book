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

    // 参照と借用
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    // 可変な参照
    {
        // let s = String::from("hello");
        // change(&s);
        //
        // fn change(some_string: &String) {
        //     some_string.push_str(", world!");
        // }
    }
    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world!");
        }

    }

    // 宙に浮いた参照
    {
        let reference_to_nothing = dangle();
        println!("{}", reference_to_nothing);

        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
        fn dangle() -> String {
            let s = String::from("hello");
            s
        }
    }

    // スライス型
    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }

        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("{} is {}", s, word);
        s.clear();
        println!("{} is {}", s, word);
    }

    // 文字列スライス
    {
        {
            let s = String::from("hello world");
            let hello = &s[0..5];
            let world = &s[6..11];
            println!("{} {}", hello, world);
        }

        {
            fn first_word(s: &String) -> &str {
                let bytes = s.as_bytes();

                for (i, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return &s[0..i];
                    }
                }

                &s[..]
            }

            let mut s = String::from("hello world");
            let word = first_word(&s);
            println!("{} is {}", s, word);
            s.clear();
        }

        // {
        //     let s = "Hello, world!";
        // }

    }

    // 引数としての文字列スライス
    {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let my_string = String::from("hello world");
        let word = first_word(&my_string[..]);
        println!("first word is {}", word);

        let my_string_literal = "hello world";
        let word = first_word(&my_string_literal[..]);
        println!("first word is {}", word);

        let word = first_word(my_string_literal);
        println!("first word is {}", word);

    }

}
