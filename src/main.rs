fn main() {
    let body = reqwest::get("https://lpw6001tws03.unix.fidelidademundial.com:31116/twsd").text()

    println!("body = {:?}", body);
}
