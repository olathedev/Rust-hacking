fn main() {
    println!("Hello, world!");
    let numbers = vec![34, 50, 25, 100, 65];
    let result = get_largest(numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(chars);
    println!("The largest char is {}", result);
}
 

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
} 