static mut X:i16 = 0;
static W:i16 = 21;
static Z:i16 = 43;

fn main() {
    unsafe {
        X = W + Z;
        println!("{} = {} + {}", X, W, Z);
    }
}
