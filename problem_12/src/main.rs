use std::io;

fn main() {
    println!("Enter the value of w:");
    let mut w = String::new();
    io::stdin().read_line(&mut w).expect("Failed to read line");
    let w: i32 = w.trim().parse().expect("Please type a number!");

    println!("Enter the value of z:");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");
    let z: i32 = z.trim().parse().expect("Please type a number!");

    let x: i32 = w + z;
    println!("The values {} and {}, read from input, add up to {}", w, z, x);
}
