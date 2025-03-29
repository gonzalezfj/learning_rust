fn main() {
    let x: i32 = {
        println!("Inside the block");
        10 + 5
    };
    println!("The result is {}", x)
}
