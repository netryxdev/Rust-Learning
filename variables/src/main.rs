const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;
fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    let _berserk = 5;
    println!("{missiles} missiles left", missiles = missiles - ready);
}

//const WARP_FACTOR: f64 = 9.9; // By Convention Default, const is scream-snake-case in rust and type annotation is required!