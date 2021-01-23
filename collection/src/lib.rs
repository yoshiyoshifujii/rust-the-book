#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /// 新しいVectorを生成する
    #[test]
    fn new_vector() {
        let v: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
        println!("{:?} {:?}", v, v2);
    }

    /// ベクタを更新する
    #[test]
    fn update_vector() {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        assert_eq!(vec![5, 6, 7, 8], v);
    }

    /// ベクタの要素を読む
    #[test]
    fn read_vector() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("{}", third);
        let third: Option<&i32> = v.get(2);
        println!("{:?}", third);

        let maybe = v.get(20);
        assert_eq!(maybe, None);

        let mut v2 = vec![1, 2, 3, 4, 5];
        let first = &v2[0];
        println!("The first element is: {}", first);
        v2.push(6);
    }

    /// ベクタの値を走査する
    #[test]
    fn run_vector() {
        let v = vec![100,32, 57];
        for i in &v {
            println!("{}", i);
        }
        let mut v = vec![100,32, 57];
        for i in &mut v {
            *i += 50
        }
        assert_eq!(vec![150, 82, 107], v);
    }

    /// Enumを使って複数の型を保持する
    #[test]
    fn keep_some_type_use_to_enum() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
