pub fn inspect(arg: &String) -> bool {
    if arg.ends_with("s") {
        println!("plural");
        return true;
    } else {
        println!("singular");
        return false;
    }
}

pub fn change(arg: &mut String) {
    if !inspect(arg) {
        arg.push_str("s");
    }
}

pub fn eat(consumes: String) -> bool {
    return if consumes.starts_with("b") && consumes.contains("a") {
        true
    } else {
        false
    };
}

pub fn bedazzle(arg: &mut String){
    *arg = String::from("sparkly"); 
}

pub fn add(number_a: i32, number_b: i32)->i32{
    return number_a + number_b;
} 
