fn main() {
    let x = do_stuff(2.0, 12.5);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz sarsaparillas(s)!", qty, oz);
    qty * oz // Can use return or omit it
}
