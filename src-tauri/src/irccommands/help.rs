
pub fn run(msg: &String) -> Option<String>
{
    println!("<received command help: {}>", msg);
    let args = msg.split_whitespace().collect::<Vec<&str>>();
    if args.len() != 1
    {
        return Some("invalid arguments".to_string())
    }
    Some("mensagem de help foda".to_string())
}