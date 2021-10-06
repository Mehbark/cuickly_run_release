use std::process::Command;

fn main() {
    //println!("{}", run_program());
    run_program_with_output();
}

fn get_working_directory() -> String {
    let mut wd = String::from_utf8(Command::new("pwd")
        .output()
        .expect("pwd didn't work for some reason lol")
        .stdout
    ).expect("failed to parse tring getting working directory");
    wd.pop();

    wd
}

fn lowest_directory(directory: String) -> String {
    directory
        .split('/')
        .last()
        .expect("pwd gave us invalid output ig or ew windows")
        .to_string()
}

fn guess_program_filename() -> String {
    lowest_directory(get_working_directory())
}

fn run_program() -> String {
    // println!(
    //         "{base}/target/release/{name}",
    //         base = get_working_directory(),
    //         name = guess_program_filename()
    //     );
    String::from_utf8(
        Command::new(format!(
            "{base}/target/release/{name}",
            base = get_working_directory(),
            name = guess_program_filename()
        ))
        .output()
        .expect("something went wrong execcing")
        .stdout
    )
    .expect("failed to parse string running program")
}

fn run_program_with_output() {
    println!("{}", run_program());
}