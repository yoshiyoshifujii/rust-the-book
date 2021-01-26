use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sample_pair::Pair;
    use std::fmt;
    use std::cmp::Ordering;

    #[test]
    fn test_pair() {
        let pair = Pair::new(1, 2);
        pair.cmp_display();

        struct NgStruct {
            a: String,
        }

        impl fmt::Display for NgStruct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.a.fmt(f)
            }
        }

        impl PartialEq for NgStruct {
            fn eq(&self, other: &Self) -> bool {
                unimplemented!()
            }

            fn ne(&self, other: &Self) -> bool {
                unimplemented!()
            }
        }

        impl PartialOrd for NgStruct {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                unimplemented!()
            }

            fn lt(&self, other: &Self) -> bool {
                unimplemented!()
            }

            fn le(&self, other: &Self) -> bool {
                unimplemented!()
            }

            fn gt(&self, other: &Self) -> bool {
                unimplemented!()
            }

            fn ge(&self, other: &Self) -> bool {
                self.a > other.a
            }
        }

        let ng1 = NgStruct { a: String::from("1") };
        let ng2 = NgStruct { a: String::from("2") };
        let pair = Pair::new(ng1, ng2);
        pair.cmp_display();
    }

}