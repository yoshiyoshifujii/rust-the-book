#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        // let r;
        // {
        //     let x = 5;
        //     r = &x;
        // }
        // println!("r: {}", r);

        let x = 5;
        let r = &x;
        println!("r : {}", r);
    }

    // ライフタイム注釈
    #[test]
    fn test_2() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("hoge");
    //     result.as_str()
    // }
}
