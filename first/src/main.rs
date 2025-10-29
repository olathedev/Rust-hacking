fn main() {
    // variable declaration
    let age = 10;
    println!("age: {age}");

    // mutability
    let mut score = 30.8;
    println!("before mutate: {score}");

    score = 10.1;
    println!("after mutate: {score}");

    // Shadowing
    let name = "john";
    println!("name first: {name}");
    let name = 1;
    println!("name after shadow: {name}");

    // scope

    let babe = "Tessa";

    {
        let babe = "Senen";
        println!("my babe innerscope: {babe}");
    }

    println!("my babe outerscope: {babe}");

    // constants
}
