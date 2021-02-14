#[cfg(test)]
mod tests {

    #[test]
    fn iterator_demonstration() {
        // 不変参照のイテレータ
        {
            let v1 = vec![1, 2, 3];

            let mut v1_iter = v1.iter();

            assert_eq!(v1_iter.next(), Some(&1));
            assert_eq!(v1_iter.next(), Some(&2));
            assert_eq!(v1_iter.next(), Some(&3));
            assert_eq!(v1_iter.next(), None);
            assert_eq!(v1_iter.next(), None);
        }
        // v1の所有権を奪い、所有された値を返すイテレータ
        {
            let v1 = vec![1, 2, 3];

            let mut v1_iter = v1.into_iter();

            assert_eq!(v1_iter.next(), Some(1));
            assert_eq!(v1_iter.next(), Some(2));
            assert_eq!(v1_iter.next(), Some(3));
            assert_eq!(v1_iter.next(), None);
        }
        // v1の可変参照を繰り返すイテレータ
        {
            let mut v1 = vec![1, 2, 3];

            let mut v1_iter = v1.iter_mut();

            assert_eq!(v1_iter.next(), Some(&mut 1));
            assert_eq!(v1_iter.next(), Some(&mut 2));
            assert_eq!(v1_iter.next(), Some(&mut 3));
            assert_eq!(v1_iter.next(), None);
        }
    }

    // イテレータを消費するメソッド
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        // println!("{:?}", v1_iter); // sumメソッドを呼ぶことで、v1_iterの所有権を奪っているため、この行は呼び出しできない
        assert_eq!(total, 6);
    }

    // 他のイテレータを生成するメソッド
    #[test]
    fn generate_other_iterator() {
        let v1 = vec![1, 2, 3];
        // v1.iter().map(|x| x + 1); // イテレータアダプタはlazyなのでこのコードだけだと実行されない。というWarningがでる。

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    // 環境をキャプチャするクロージャを使用する
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

    // Iteratorトレイトで独自のイテレータを作成する
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum) // 2 * 3 + 3 * 4
    }
}

