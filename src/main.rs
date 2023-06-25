fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);
}
