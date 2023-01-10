use std::{
    env,
    process::{Command, Output},
};
use std::collections::HashSet;
use text_io::read;

 const INPUT_PROMPT: &'static str = "Enter file name prefix for which you wish to perform 'git add' (you can enter multiple option with space separated delimiter) : ";
fn main() {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];

    let output: Output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .expect("Failed to run git command");

    let dirty_out = String::from_utf8_lossy(&output.stdout);

    let files = dirty_out.lines().collect::<Vec<_>>();
    let files = files.iter().map(|x| &x[3..]).collect::<Vec<_>>();

    match args.len() {
        0 => {
            //  when no args provided
            match files.len() {
                0 => {
                    print!("There is no files to add !!");
                }
                _ => {
                    println!("{}", INPUT_PROMPT);
                    println!();
                    prompt_user_options(&files);
                    println!();
                    read_input(&files)
                }
            }
        }
        _ => {
            //  when file names provided in args
            for input in args {
                let mut eligible: Vec<&str> = vec![];
                for git_file in files.iter() {
                    if is_match(git_file, input) {
                        eligible.push(git_file);
                    }
                }
                match eligible.len() {
                    1 => git_add(eligible.first().unwrap()),
                    len if len > 1 => {
                        println!("duh, there are multiple matching files,");
                        println!("{}", INPUT_PROMPT);
                        prompt_user_options(&eligible);
                        read_input(&eligible)
                    }
                    _ => println!("no matching file found")
                }
            }
        }
    }
}

fn prompt_user_options(eligible: &[&str]) {
    println!("[a] add all files");
    for (index, file_name) in eligible.iter().enumerate() {
        println!("[{}] {}", index, file_name);
    }
}

/// read input from terminal and execute 'git add' command based on the input
fn read_input(files: &[&str]) {
    let line: String = read!("{}\n");
    let option: HashSet<&str> = HashSet::from_iter(line.split(' '));
    for (index, file_name) in files.iter().enumerate() {
        if option.contains(index.to_string().as_str()) || option.contains("a") {
            git_add(file_name);
        }
    }
}

/// perform git add command
/// input : file path
fn git_add(file_name: &str) {
    Command::new("git").arg("add").arg(file_name).output().expect("failed to run git add command");
    println!("Executed : git add {}", file_name);
}

/// match user input with un staged files, matches if un staged file path contains user input file name
/// or check user input matches only capital letters from un staged file name
fn is_match(git_file: &str, user_input: &str) -> bool {
    let short_hand = git_file.split('/').last().unwrap_or_default().chars().filter(|x| x.is_uppercase()).collect::<String>();
    git_file.contains(user_input) || short_hand == user_input
}