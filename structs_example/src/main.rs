fn main() {
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the ractangle is {} square pixels",
            area(width1, height1)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

    }

    // タプルでリファクタリングする
    {
        let rect1 = (30, 50);

        println!(
            "The area of the ractangle is {} square pixels",
            area(rect1)
        );

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    // 構造体でリファクタリングする
    {
        struct Rectangles {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangles { width: 30, height: 50 };

        println!(
            "The area of the ractangle is {} square pixels",
            area(&rect1)
        );

        fn area(rectangle: &Rectangles) -> u32 {
            rectangle.width * rectangle.height
        }
    }

    // トレイトの導出で有用な機能を追加する
    {
        #[derive(Debug)]
        struct Rectangles {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangles { width: 30, height: 50 };

        println!("rect1 is {:#?}", rect1);
    }

    // メソッドを定義する
    {
        #[derive(Debug)]
        struct Rectangles {
            width: u32,
            height: u32,
        }

        impl Rectangles {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect1 = Rectangles { width: 30, height: 50 };

        println!(
            "The area of the ractangle is {} square pixels",
            rect1.area()
        );
    }

    // より引数の多いメソッド
    {
        #[derive(Debug)]
        struct Rectangles {
            width: u32,
            height: u32,
        }

        impl Rectangles {

            fn can_hold(&self, _rectangle: &Rectangles) -> bool {
                self.width > _rectangle.width && self.height > _rectangle.height
            }
        }

        let rect1 = Rectangles { width: 30, height: 50 };
        let rect2 = Rectangles { width: 10, height: 40 };
        let rect3 = Rectangles { width: 60, height: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    }

    // 関連関数
    {
        #[derive(Debug)]
        struct Rectangles {
            width: u32,
            height: u32,
        }

        impl Rectangles {
            fn square(size: u32) -> Rectangles {
                Rectangles { width: size, height: size }
            }
        }

        println!("{:#?}", Rectangles::square(30));

    }
}

