fn main() {
    // Enumを定義する
    {
        #[derive(Debug)]
        enum IpAddKind {
            V4,
            V6,
        }

        let four = IpAddKind::V4;
        let six = IpAddKind::V6;

        fn route(ip_type: &IpAddKind) {
            println!("ip type is {:#?}", ip_type);
        }

        route(&four);
        route(&six);

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddKind::V6,
            address: String::from("::1"),
        };

        println!("{:#?}, {:#?}", home, loopback)
    }

    // Enumの値
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

        println!("{:?}, {:?}", home, loopback)
    }

    // Enumの値2
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(u32, u32, u32, u32),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        println!("{:?}, {:?}", home, loopback)
    }

    // Option enumとnullに勝る利点
    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<u32> = None;

        println!("{:?} {:?} {:?}", some_number, some_string, absent_number);

        // let x: i8 = 5;
        // let y: Option<i8> = Some(5);
        // let sum = x + y;
    }

    // matchフロー制御演算子
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        println!("{}", value_in_cents(Coin::Dime));
        println!("{}", value_in_cents(Coin::Penny));
        println!("{}", value_in_cents(Coin::Nickel));
        println!("{}", value_in_cents(Coin::Quarter));
    }

    // 値に束縛されるパターン
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}", state);
                    25
                }
            }
        }
        println!("{}", value_in_cents(Coin::Dime));
        println!("{}", value_in_cents(Coin::Penny));
        println!("{}", value_in_cents(Coin::Nickel));
        println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
        println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    }

    // Option<T>とのマッチ
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?} {:?}", six, none);
    }

    // if letで簡潔なフロー制御
    {
        let some_u8_value = Some(3);
        match some_u8_value {
            Some(3) => println!("there"),
            _ => (),
        }

        if let Some(3) = some_u8_value {
            println!("there");
        }
    }
}
