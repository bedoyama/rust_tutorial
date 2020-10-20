fn main() {
    println!("Hello, world!");

    let mut x: i32 = 5;
    x += 3;
    let y = 6;
    let z = x * y;
    println!("z is {}", z);

    let flag = true;

    let name = "Rust in Motion";
    let big_name = name.to_uppercase();
    println!("{} and {} and {}", big_name, z, flag)
}
