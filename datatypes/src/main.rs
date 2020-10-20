fn main() {
    println!("Hello, data types!");

    // Primitive
    // Boolean: bool
    let a = true;

    if a {
        println!("a is true")
    } else {
        println!("a is false")
    }

    // Integer: i and u
    let x: i8 = -2;
    let y: i16 = -432;
    let z: i32 = -32132;
    let b: i64 = -431433143433242;
    let c: u8 = 2;
    let d: u16 = 432;
    let e: u32 = 3432432132;
    let f: u64 = 433321433143433242;

    let j: isize = -43243242423423324;
    let k: usize = 43243242423423324;

    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", x, y, z, b, c, d, e, f, j, k);

    // Floating point: f
    let g: f32 = 0.1;
    let h: f64 = 3.1415926535;

    println!("{} and {}", g, h);

    // Character: char
    let i: char = '&';

    println!("{}", i);

    // Built in types
    // Tuples
    let tup = (1, 'c', true);
    let (l, m, n) = tup;

    println!("{}, {}, {}", l, m, n);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // Arrays
    let mut arr = [0.0, 3.14, 4.938474838, 4.543, 2.32423];
    arr[0] = -9.12332;
    
    println!("{:?}", arr);

    // Slices
    let slice = &arr[1..3];

    println!("{:?}", slice);
}
