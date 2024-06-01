pub fn warning(msg: &String) -> () {
    eprintln!("warning: {}", msg)
}

pub fn failure(msg: &String) -> ! {
    eprintln!("failure: {}", msg);
    std::process::exit(1)
}
