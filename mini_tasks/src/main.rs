// one
// fn main() {
//     let mut data = vec![1, 2, 3];
//     data.push(4);
//     let first = &data[0];
//     println!("First: {}", first);
// }

// two
// fn main() {
//     let s = String::from("hello");
//     process(s);
//     // process(s);
// }

// three
// fn process(_s: String) { /* consumes string */
// }

// four
// fn main() {
//     let mut x = 5;
//     let r1 = &mut x;
//     println!("{}", r1);
//     let r2 = &mut x;
//     println!("{}", r2);
// }

// five
// fn main() {
//     dangle();
// }

// fn dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// six
// fn main() {
//     let mut v = vec![1, 2, 3];
//     v.push(4);
//     let first = &v[0];
//     println!("First: {}", first);
// }

// Seven
// fn main() {
//     struct Processor {
//         data: String,
//     }

//     impl Processor {
//         fn process(&mut self) -> &str {
//             self.data = self.data.trim().to_string(); // ERROR
//             &self.data
//         }
//     }
// }

// Eight
// fn main() {
//     get_slice("ola", 0, 2);
// }

// fn get_slice(s: &str, start: usize, end: usize) -> &str {
//     &s[start..end]
// }

fn main() {
    let names = vec!["Alice".to_string(), "Bob".to_string()];
    for name in &names {
        println!("{}", name);
    }
    println!("Count: {}", names.len());
}
