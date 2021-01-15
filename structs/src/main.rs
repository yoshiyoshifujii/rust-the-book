fn main() {
    println!("Hello, world!");

    {
        let user1 = User {
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true,
        };

        print_user(&user1);
    }
    {
        let mut user1 = User {
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true,
        };

        user1.email = String::from("hoge@hoge.com");

        print_user(&user1);
    }
    {
        fn build_user(email: String, username: String) -> User {
            User {
                username,
                email,
                active: true,
                sign_in_count: 2,
            }
        }

        let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

        print_user(&user1);
    }
    // 構造体更新記法
    {
        fn build_user(email: String, username: String) -> User {
            User {
                username,
                email,
                active: true,
                sign_in_count: 2,
            }
        }

        let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

        print_user(&user1);

        let user2 = User {
            email: String:: from("another@example.com"),
            ..user1
        };

        print_user(&user2);
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_user(user: &User) {
    println!("{}, {}, {}, {}", user.email, user.username, user.sign_in_count, user.active);
}