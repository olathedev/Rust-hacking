fn main() {
    // mutability
    // println!("Variables and mutability");

    // let mut x = 5;
    // println!("the value of x is: {x}");

    // x = 2;
    // println!("the value of x is: {x}");

    // shadowing
    let x = 2;

    let x = 2 + x;
    {
        let x = 6
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");


}
