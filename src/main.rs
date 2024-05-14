mod arg;
mod numbers;

fn main() {
    // 인자를 받아서 처리
    let matches = arg::build_cli().get_matches();

    let arg_num = matches.get_many::<String>("numbers").unwrap();
    let (num1, num2) = numbers::parse_numbers(Some(arg_num.map(|s| s.to_string())));

    if matches.get_flag("add") {
        let sum = numbers::sum_numbers(num1, num2);
        println!("{}", sum);
    } else if matches.get_flag("subtract") {
        let difference = numbers::subtract_numbers(num1, num2);
        println!("{}", difference);
    } else {
        eprintln!("Please specify an operation: -a to add or -s to subtract");
    }
}
