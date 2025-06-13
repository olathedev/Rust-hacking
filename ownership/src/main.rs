fn main() {
    let s1 = String::from("Heyy");
    let len = calculate_length(&s1);
    println!("the length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}