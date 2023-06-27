#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn area_double(&self) -> u32 {
        self.width * self.height * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rect1.area());
    dbg!(rect1.area_double());
}
