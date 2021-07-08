fn main() {
    println!("What is your name?");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let s = s.trim();
    println!("Hello, {}!", s);

    println!("Rolling the dice...");

    let (a1, a2) = (rand::random::<u8>() % 6 + 1, rand::random::<u8>() % 6 + 1);
    println!("Die 1: {}", a1);
    println!("Die 2: {}", a2);

    println!("Total value: {}", a1 + a2);

    if a1 + a2 > 7 {
        println!("{} won!", s);
    }
    else {
        println!("{} lost", s);
    }
}
