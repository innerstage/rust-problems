fn main() {
    for i in 1..=9 {
        for _j in 1..i {
            print!("{} ",i);
        }
        println!("{}",i);
    }
}
