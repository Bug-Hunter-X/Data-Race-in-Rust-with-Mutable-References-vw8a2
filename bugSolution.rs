fn main() {
    let mut x = 5;
    { // Using a block to limit the scope of y 
        let y = &mut x;
        *y += 1;
    }
    { // Using a block to limit the scope of z 
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
