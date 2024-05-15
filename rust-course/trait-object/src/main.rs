#[derive(Debug)]
enum UiObject {
    Button,
    SelectBox
}

pub trait Draw {
    fn draw(&self);
}

#[derive(Clone)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone()
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        
    }
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

trait Paint {
    fn draw(&self) -> String;
}

impl Paint for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Paint for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn paint1(x: Box<dyn Paint>) {
    x.draw();
}

fn paint2(x: Box<dyn Paint>) {
    x.draw();
}

fn main() {
    let objects =[
        UiObject::Button,
        UiObject::SelectBox
    ];

    for o in objects {
        draw(o);
    }

    println!("x is {:#?}", paint1(Box::new(8)));
    println!("x is {:#?}", paint1(Box::new(9.1)));
}

fn draw(o: UiObject) {
    println!("o is {:#?}", o);
}