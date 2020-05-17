fn main() {
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is {}", x);
    let tup = (300, 2.4, 1);
    let (x, _y, z) = tup;
    println!("Value of z: {}", z);
    println!("Second value in tup: {}", tup.1);
    let _arr = [2,3,4,5];

    if x < 10 {
        println!("Too small");
    } else {
        println!("Too big");
    }
}
