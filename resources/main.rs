fn main() {
    // output
    // code
    // iterator

    let result = run(&mut code, i);
    if let Err(e) = result {
        println!("{}", e);
    }
}
