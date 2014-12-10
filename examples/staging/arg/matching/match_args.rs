use std::os;

fn increase(number: int) {
    println!("{}", number + 1);
}

fn decrease(number: int) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args = os::args();

    match args.as_slice() {
        // no arguments passed
        [ref name] => {
            println!("My name is '{}'. Try passing some arguments!", name);
        },
        // one argument passed
        [_, ref string] => {
            if string.as_slice() == "42" {
                println!("This is the answer!");
            } else {
                println!("This is not the answer.");
            }
        },
        // one command and one argument passed
        [_, ref cmd, ref num] => {
            // parse the number
            let number: int = match from_str(num.as_slice()) {
                Some(n) => {
                    n
                },
                None => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match cmd.as_slice() {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
