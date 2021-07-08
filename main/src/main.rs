fn main() {
    println!("Rolling the dice...");

    let (a1, a2) = (rand::random::<u8>() % 6 + 1, rand::random::<u8>() % 6 + 1);
    println!("Die 1: {}", a1);
    println!("Die 2: {}", a2);
}
