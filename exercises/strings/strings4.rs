// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    
    // .to_string() 创建 String
    string("red".to_string());
    
    // String::from 创建 String
    string(String::from("hi"));
    
    // .to_owned() 创建 String
    string("rust is fun!".to_owned());
    
    // .into() 根据上下文推断类型，这里推断为 String
    string("nice weather".into());
    
    // format! 宏返回 String
    string(format!("Interpolation {}", "Station"));
    
    // 字符串切片操作返回 &str
    string_slice(&String::from("abc")[0..1]);
    
    // .trim() 返回 &str
    string_slice("  hello there ".trim());
    
    // .replace() 返回 String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    
    // .to_lowercase() 返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
