pub fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s = {s}");
}
