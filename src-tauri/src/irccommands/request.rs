use url::Url;

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run(msg: &String) -> Option<String> {
    // precisa dar um jeito de saber quem mandou o commando

    if true {
        let args = msg.split_whitespace().collect::<Vec<&str>>();
        if args.len() == 2 {
            if !is_valid_url(args[1]) {
                return Some("Invalid argument, this is not a valid URL  + help".to_string());
            }

            // aqui adicionar link e quem mandou em um vector struct

            return Some("Done!".to_string());
        }
        return Some("Invalid number of arguments + help".to_string());
    }
    Some("Requests are closed!".to_string())
}
