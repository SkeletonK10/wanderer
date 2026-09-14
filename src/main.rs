fn main() {
    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [] => println!("사용법: wd <명령어>"),
        [cmd] => println!("명령: {}", cmd),
        [cmd, rest @ ..] => println!("명령: {}, 인자: {:?}", cmd, rest),
    }
}
