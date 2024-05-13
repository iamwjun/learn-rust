struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Pos<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pos<T, U> {
    fn mixup<V, W>(self, other: Pos<V, W>) -> Pos<T, W> {
        Pos {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));

    let p = Point { x: 5, y: 10 };
    println!("p.x is {}", p.x());

    let p1 = Pos { x: 5, y: 10 };
    let p2 = Pos { x: 8, y: 16 };
    println!("after mix up is {:#?}", p1.mixup(p2));

    let f = Point { x: 1.2f32, y: 1.5f32 };
    println!("float point is {}", f.distance_from_origin());

    let arr1: [i32; 2] = [1, 2];
    display_array(&arr1);

    let arr2: [i32; 3] = [1, 2, 3];
    display_array(&arr2);

    let arr3: [i32; 5] = [1, 2, 3, 4, 5];
    let arr4: [i32; 6] = [1, 2, 3, 4, 5, 6];
    display_arr(&arr3);
    display_arr(&arr4);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("arr is {:?}", arr)
}

fn display_arr<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("arr is {:?}", arr)
}
