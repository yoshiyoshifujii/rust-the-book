#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /// 文字列を生成する
    #[test]
    fn new_string() {
        let mut s = String::new();
        s.push('a');
        println!("{}", s);

        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);

        let s = "initial contents".to_string();
        println!("{}", s);
    }

    /// push_strとpushで文字列に追加する
    #[test]
    fn push_str() {
        let mut s = String::from("foo");
        s.push_str("bar");
        assert_eq!(String::from("foobar"), s);

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        assert_eq!(String::from("bar"), s2);

        let mut s2 = String::from("lo");
        s2.push('l');
        assert_eq!("lol".to_string(), s2);
    }

    /// +演算子、またはformat!マクロで連結
    #[test]
    fn join() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("{}, {}", s2, s3);

        let a1 = String::from("tic");
        let a2 = String::from("tac");
        let a3 = String::from("toe");
        let formatted = format!("{}-{}-{}", a1, a2, a3);
        println!("{}", formatted);
    }

    /// 内部表現
    #[test]
    fn inside() {
        let len = String::from("Hola").len();
        println!("{}", len);

        let len = String::from("にほんご").len();
        println!("{}", len);

        // let hello = String::from("こんにちは");
        // let answer = &hello[0]; // ←コンパイルエラーになる
    }

    /// 文字列をスライスする
    #[test]
    fn slice_string() {
        let hello = "こんにちは";
        let s = &hello[0..3];
        println!("{}", s);
    }

    /// 文字列を走査するメソッド群
    #[test]
    fn run_string() {
        for c in "こんにちは".chars() {
            println!("{}", c);
        }
        for b in "こんにちは".bytes() {
            println!("{}", b);
        }
    }
}
