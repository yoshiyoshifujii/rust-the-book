#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// 新規ハッシュマップを生成する
    #[test]
    fn new_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:#?}", scores);

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:#?}", scores);
    }

    /// ハッシュマップと所有権
    #[test]
    fn hashmap_and_ownership() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        println!("{:#?}", map);
        // println!("{}", field_value); // ←コンパイルエラー、所有権がHashMapに移動してる
    }

    /// ハッシュマップの値にアクセスする
    #[test]
    fn get_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("team name is {}, score is {:?}", team_name, score);

        for (key, value) in &scores {
            println!("{}, {}", key, value);
        }
    }

    /// ハッシュマップを更新する
    #[test]
    fn update_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:#?}", scores);

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        println!("{:#?}", scores);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:#?}", scores);
    }

    /// 古い値に基づいて値を更新する
    #[test]
    fn update_the_value_based_on_the_old_value() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }

    /// 整数のリストが与えられ、mean, median, modeを取得する
    #[test]
    fn sample1() {
        let input: Vec<usize> = vec![10, 30, 20];

        let sum = &input.iter().sum();
        let count = &input.len();
        let avg = sum / count;
        assert_eq!(20, avg);

        fn median(_in: &Vec<usize>) -> usize {
            let mut sorted = _in.clone();
            sorted.sort();
            let count = &sorted.len();
            match count % 2 {
                0 => {
                    let n = count;
                    let e1 = n / 2;
                    let x1 = &sorted[e1 - 1];
                    let e2 = (n / 2) + 1;
                    let x2 = &sorted[e2 - 1];
                    (x1 + x2) / 2
                },
                _ => {
                    let n = count;
                    let e = (n + 1) / 2;
                    *&sorted[e - 1]
                },
            }
        }
        assert_eq!(20, median(&input));
        assert_eq!(15, median(&vec![10, 20, 30, 5]))
    }
}
