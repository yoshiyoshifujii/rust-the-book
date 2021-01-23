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
}

