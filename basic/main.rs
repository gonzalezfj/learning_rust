fn main() {
    let oranges = 10;
    println!("oranges === {}", oranges);
    {
        let oranges = 25 + oranges;
        println!("oranges === {}", oranges)
    }
    println!("oranges === {}", oranges)
}
