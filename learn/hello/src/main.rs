
fn main() {
    let my_name = String::from("Israel");
    hello::greet_name(&my_name);
    println!("After, {}", my_name);
}
