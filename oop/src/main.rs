mod state_pattern;
mod state_pattern_2;

fn main() {
    println!("Hello, world!");
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.width);
    }
}

pub struct SelectBox {
    width: u32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: {}", self.width);
    }
}

#[cfg(test)]
mod test {
    use crate::{Button, Draw, Screen, SelectBox};

    #[test]
    fn test_1() {
        let components: Vec<Box<dyn Draw>> = vec![
            Box::new(SelectBox { width: 75 }),
            Box::new(Button { width: 50 }),
        ];
        let screen = Screen { components };

        screen.run();
    }
}
