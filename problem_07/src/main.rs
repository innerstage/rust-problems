fn abs(x:i8) -> i8 {
    if x >= 0 {
        return x;
    } else {
        return -x;
    }
}

fn main() {
    for i in 1..=17 {
        let a = -abs(i-9)+9;
        for _j in 1..a {
            print!("{} ",a);
        }
        println!("{}",a);
    }
}
