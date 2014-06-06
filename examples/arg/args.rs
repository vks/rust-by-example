use std::os;

fn main() {
    let args = os::args();
    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args.get(0));
    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    // $ ./args arg1 arg2
    println!("I got {} arguments: {}.", args.len() - 1, args.tail());
}
