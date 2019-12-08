fn main() {
    // output
    // code
    // iterator

    let result = run(&mut code);
    if let Err(e) = result {
        println!("{}", e);
    }
}
