fn main() {
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    // println!("{}, {}", x, y); // Error on Y Scope! 
}

fn main2() {
    let x = 5;
    {
        let yx = 99;
        println!("{}", x); // Prints "99"
    }
    println!("{}", x); // Prints "5"
} // This diffenrence between var value scopes is called var shadowed

fn main3() {
    let meme = "More cowbell!"; // Starts as a string
    //let meme = make_image(meme); // And becomes here a img.
}
