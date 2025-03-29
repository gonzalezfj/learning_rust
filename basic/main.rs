fn main() {
    let x:() = ();
    let y:() = println!("wow");
    assert_eq!(x, y);
    println!("All units are the same")
}
