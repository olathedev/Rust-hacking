// note references are immutable and we cant borrow the value as mutable in here
// fn main() {
//     let s1 = String::from("Heyy");
//     let len = calculate_length(&s1);
//     println!("the length of {} is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     let length = s.len();
//     length
// }

// situation where you want to borrow value as mutable
// fix: use a mutabele borrow
// note: you can only have one mutable reference for a particular data in a scope
// fn main() {
//     let mut s1 = String::from("Heyy");
//     change_string(&mut s1);
// }

// fn change_string(param: &mut String) {
//     param.push_str(", yoo");
// }

fn main() {
    let mut s = String::from("Heyy");

    let s1 = &s;
    let s2 = &s;
    
    println!("{s}");

    let s3 = &mut s;
}

// Dangling reference: a varibale that points to invalid data
