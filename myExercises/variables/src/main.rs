fn main() {
    let (mut missles, ready): (i32, i32) = (8, 2);
    println!("Firing {} of my {} missles...", ready, missles);
    missles = missles - ready;
    println!("{} missles left.", missles);

    const STARTING_MISSLES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    {
        let missles = STARTING_MISSLES;
        let ready = READY_AMOUNT;
    }
}
