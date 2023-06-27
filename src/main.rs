#[derive(Debug)]
struct Rational {
    num: isize,
    den: isize,
}

impl Rational {
    fn new(n: isize, d: isize) -> Self {
        Self { num: n, den: d }
    }
    fn float(&self) -> f32 {
        self.num as f32 / self.den as f32
    }
    fn gcd(a: isize, b: isize) -> isize {
        if b == 0 {
            return a;
        }
        return Self::gcd(b, a % b);
    }
    fn reduse(&mut self) {
        let n = Self::gcd(self.num, self.den);
        self.num /= n;
        self.den /= n;
        if self.den < 0 {
            self.num *= -1;
            self.den *= -1;
        }
    }
    fn add(r1: &Self, r2: &Self) -> Self {
        Self {
            num: r1.num * r2.den + r2.num * r1.den,
            den: r1.den * r2.den,
        }
    }
    fn sub(r1: &Self, r2: &Self) -> Self {
        Self {
            num: r1.num * r2.den - r2.num * r1.den,
            den: r1.den * r2.den,
        }
    }
    fn mul(r1: &Self, r2: &Self) -> Self {
        Self {
            num: r1.num * r2.num,
            den: r1.den * r2.den,
        }
    }
    fn div(r1: &Self, r2: &Self) -> Self {
        Self {
            num: r1.num * r2.den,
            den: r1.den * r2.num,
        }
    }
}

fn main() {
    let r1 = Rational::new(1, 3);
    let r2 = Rational::new(2, 3);
    dbg!(&r1, &r2);
    let mut r = Rational::add(&r1, &r2);
    dbg!(&r);
    r.reduse();
    dbg!(&r);
    dbg!(r.float());
}
