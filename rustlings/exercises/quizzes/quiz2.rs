// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.
#[derive(Debug)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut new_vec = Vec::new();

        for data in input.into_iter(){ 

            let mut your_string = data.0;
            let command = data.1;
            println!("String {}", your_string);
            // depending on the command, the operation will be undertaken

            match command {
                Command::Uppercase => new_vec.push(your_string.to_uppercase()),
                Command::Trim => new_vec.push(your_string.trim().to_string()),
                Command::Append(val) => {
                    println!("Entering here...");
                    let mut mx: usize= 0;
                    loop {
                        your_string.push_str("bar"); 
                        mx += 1;
                        if mx > val - 1 {
                            break
                        }
                    }
                    // update the string into the vector
                    new_vec.push(your_string);
                }
            }
        }
        new_vec
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer; 
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
