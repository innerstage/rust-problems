/*static X: f32 = 4.129;
static mut Y: f32 = 1.7;
static mut W: f32 = 2.34;
static mut Z: f32 = 9.175;

fn main() {
    unsafe {
        println!("x: {}, y: {}, w: {}, z: {}", X, Y, W, Z);
        Y = X;
        W = X;
        Z = X;
        println!("x: {}, y: {}, w: {}, z: {}", X, Y, W, Z);
    }
}*/

static X: f32 = 4.129;
static mut Y: f32 = 1.7;
static mut W: f32 = 2.34;
static mut Z: f32 = 9.175;

fn main() {
    unsafe {
        println!("x: {}, y: {}, w: {}, z: {}", X, Y, W, Z);
        let mut vars = vec![Y, W, Z];
        for i in 0..vars.len()-1 {
            vars[i] = X;
        }
        println!("x: {}, y: {}, w: {}, z: {}", X, vars[0], vars[1], vars[2]);
    }
}