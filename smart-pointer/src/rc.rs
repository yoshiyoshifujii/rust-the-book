#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    #[test]
    fn demo() {
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a)); // value used here after move
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests2 {
    use std::rc::Rc;
    use crate::rc::tests2::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    #[test]
    fn demo() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }

    #[test]
    fn クローンするとカウントが増える() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}
