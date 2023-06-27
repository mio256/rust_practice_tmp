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
    fn zero(&mut self) {
        self.width = 0;
        self.height = 0;
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rect1.area());
    dbg!(rect1.area_double());
    dbg!(rect1.zero());
    dbg!(rect1.area());
    dbg!(rect1.area_double());
}
