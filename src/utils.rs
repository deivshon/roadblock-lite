fn roadblock_lite_log(msg: &String) {
    eprintln!("roadblock-lite: {}", msg)
}

pub fn warning(msg: &String) {
    roadblock_lite_log(&format!("warning: {}", msg))
}

pub fn failure(msg: &String) -> ! {
    roadblock_lite_log(&format!("failure: {}", msg));
    std::process::exit(1)
}
