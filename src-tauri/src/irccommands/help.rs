#[derive(Clone)]
pub struct Help {
    pub command: String,
    pub description: String,
    pub usage: String,
}

pub fn create(helps: Vec<Help>) -> String {
    "WIP".to_string()
}

pub fn run(msg: &String, help: &String) -> Option<String> {
    let args = msg.split_whitespace().collect::<Vec<&str>>();
    if args.len() == 1 {
        return Some("full help message".to_string());
    }
    if args.len() == 2 {
        return Some("especific help message".to_string());
    }
    Some("full help message com aviso de erro de argumento".to_string())
}
