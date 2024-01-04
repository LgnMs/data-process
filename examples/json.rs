fn main() {
    let s = "abc".to_string();

    println!("{:?}", s.get(0..0+1).unwrap() == "a");
    println!("{}", &s[0..1]);
}