struct Player {
    username: String,
    streak: u64,
    points: u64,
}


// fn main() {
//     let player1 = Player {
//         username: String::from("Oganla"),
//         streak: 0,
//         points: 0
//     };

//     let player1_username = player1.username;
//     println!("{}", player1_username)

//     let player2 = create_player(String::from("john doe"))
//     let player3 = create_player(String::from("salama"))


// }


// fn create_player(username: String) -> Player {
//     Player {
//         username,
//         points: 0,
//         streak: 0,
//     }
// }

// Tupple like structs
fn main() {
    struct Color(i32, i32, i32)
    struct Location(i64, i64, i64)
}