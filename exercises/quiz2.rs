// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        let bar = "bar";
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                },
                Command::Trim => {
                    output.push(string.trim().to_string());
                },
                Command::Append(u) => {
                    output.push(format!("{}{}", string, bar.repeat(*u)));
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use my_module::transformer;
    use super::Command;
    use string;
    use string_slice;

    #[test]
    fn it_works() {
        // &str to_owned => String
        let vec: Vec<(String,Command)> = vec![
            // into return &str
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ];
        string(vec[0].0.clone());
        // &str == &String
        string_slice(&vec[0].0);
        let output = transformer(vec);
        string_slice("bar".into());
        string(output[0].clone());
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}
