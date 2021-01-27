#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention pleases: {}", announcement);
        self.part
    }
}

#[cfg(test)]
mod tests {
    use crate::sample_structure::ImportantExcerpt;

    #[test]
    fn test_1() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:?}", i);
        println!("{}", i.level());
        println!("{}", i.announce_and_return_part("hoge"));
    }
}
