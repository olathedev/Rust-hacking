struct Player {
    username: String,
    streak: u64,
    points: u64,
}


fn main() {
    let player1 = Player {
        username: String::from("Oganla"),
        streak: 0,
        points: 0
    };

    let player1_username = player1.username;
    println!("{}", player1)
}
