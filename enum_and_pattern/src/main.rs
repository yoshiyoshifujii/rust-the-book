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

        let home = IpAddr { kind: IpAddKind::V4, address: String::from("127.0.0.1") };
        let loopback = IpAddr { kind: IpAddKind::V6, address: String::from("::1") };

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
}
