use url::Url;

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run(msg: &String) -> Option<String>
{
    println!("<received command request: {}>", msg);
    let args = msg.split_whitespace().collect::<Vec<&str>>();
    if args.len() == 2
    {
        if !is_valid_url(args[1]) {
            return Some("URL is not valid".to_string())
        }
        // aqui
    }
    Some("invalid arguments".to_string())
}