pub fn red(text: String)-> String {
    format!("\x1b[31;4m{}\x1b[0m", text)
}

pub fn blue(text: &String)-> String {
    format!("\x1b[34;4m{}\x1b[0m", text)
}

pub fn green(text: &String)-> String {
    format!("\x1b[32;4m{}\x1b[0m", text)
}

pub fn yellow(text: String)-> String {
    format!("\x1b[33;4m{}\x1b[0m", text)
}