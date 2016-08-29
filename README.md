# MonstrIO - set of high level IO methods.

# Usage

    extern crate monstrio;
    use std::env;
    use std::io;
    
    fn main() {
        let stdin = io::stdin();
        let input_stdin = monstrio::Input::stdin(&stdin);

        let args: Vec<_> = env::args().collect();
        let glob_input = monstrio::Input::glob(args[1..].into_iter());

        // ...
    }
    
