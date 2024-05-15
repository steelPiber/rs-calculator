use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("rsadd")
        .version("0.0.2")
        .author("piber")
        .about("Adds or subtracts two numbers")
        .arg(
            Arg::new("numbers") // 숫자를 받아들이는 인자
                .help("The two numbers to add or subtract | 2개의 정수를 더하거나 뺌") // 도움말
                .required(true) // 필수 인자 사용 요청
                .num_args(2) // 정확히 2개의 인자를 받아냄 ex) rsadd 1 2
                .value_name("NUMBER"), // 인자의 이름 설정
        )
        .arg(
            Arg::new("add")
                .short('a') // 덧셈을 수행하는 옵션
                .long("add") // 덧셈을 수행하는 긴 옵션
                .help("Add the two numbers | 두 숫자를 더함") // 도움말
                .action(clap::ArgAction::SetTrue), // 값 설정 액션
        )
        .arg(
            Arg::new("subtract")
                .short('s') // 뺄셈을 수행하는 옵션
                .long("sub") // 뺄셈을 수행하는 긴 옵션
                .help("Subtract the two numbers | 두 숫자를 뺌") // 도움말
                .action(clap::ArgAction::SetTrue), // 값 설정 액션
        )
        .arg(
            Arg::new("multiplication")
                .short('m') // 곱셈을 수행하는 옵션
                .long("mul") // 곱셈을 수행하는 긴 옵션
                .help("Multiply the two numbers | 두 숫자를 곱함") // 도움말
                .action(clap::ArgAction::SetTrue), // 값 설정 액션
        )
}
