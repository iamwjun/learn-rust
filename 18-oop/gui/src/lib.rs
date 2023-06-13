pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
 {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width: u32,
    heigth: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

#[cfg(test)]
mod tests {}
