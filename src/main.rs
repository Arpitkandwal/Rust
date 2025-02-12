fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let concatenated_string = concatenate_string(&s1, &s2);
    println!("{}", concatenated_string);
}

fn concatenate_string(s1: &str, s2: &str) -> String {
    let mut result = String::new();

    result.push_str(s1);
    result.push_str(" ");
    result.push_str(s2);

    return result;
}
