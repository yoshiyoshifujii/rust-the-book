struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}!`", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::drop::CustomSmartPointer;

    #[test]
    fn 片付け() {
        CustomSmartPointer { data: String::from("my stuff") };
        CustomSmartPointer { data: String::from("other stuff") };
        println!("CustomSmartPointer created.")
    }

    #[test]
    fn 落ちる() {
        let c = CustomSmartPointer { data: String::from("some data") };
        print!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
