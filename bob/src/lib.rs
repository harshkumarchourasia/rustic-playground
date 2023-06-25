pub fn reply(message: &str) -> &str {
    let mut _message = &message.chars()
    .filter(|x| !x.is_whitespace())
    .filter(|x| x.is_alphabetic() || x==&'?')
    .collect::<String>();

    println!("{:?}", _message);
    match _message.len(){
        0 => "Fine. Be that way!",
        _ => {
            let is_question: bool = _message.chars().nth(_message.len()-1).unwrap()=='?';
            let mut is_yell: bool = false;
            if _message == &_message.to_uppercase() 
            {
                if _message.len()==1 && _message.chars().nth(_message.len()-1).unwrap()=='?'{
                    is_yell = false;
                }
                is_yell = true;
            }

            if is_question && is_yell {
                "Calm down, I know what I'm doing!"
            }
            else if is_question{
                "Sure."
            }
            else if is_yell {
                "Whoa, chill out!"
            }
            else {
                "Whatever."
            }

        }
    }
}
