fn main() {
    println!("Hello, world!");
    let x = get_time("abc");
    print!("{}",x);
}

fn get_time(s: &str) -> &str {
    s
}