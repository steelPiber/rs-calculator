use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("rsadd")
        .version("0.0.1")
        .author("piber")
        .about("Adds or subtracts two numbers")
        .arg(
            Arg::new("numbers")
                .help("The two numbers to add or subtract | 2개의 정수를 더하거나 뺌")
                .required(true) // 필수 인자
                .num_args(2) // 정확히 2개의 인자를 받아야 함
                .value_name("NUMBER"), // 값의 이름
        )
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .help("Add the two numbers | 두 숫자를 더함")
                .action(clap::ArgAction::SetTrue), // 값 설정 액션
        )
        .arg(
            Arg::new("subtract")
                .short('s')
                .long("sub")
                .help("Subtract the two numbers | 두 숫자를 뺌")
                .action(clap::ArgAction::SetTrue), // 값 설정 액션
        )
}
