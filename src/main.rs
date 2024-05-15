mod arg;
mod numbers;

use std::collections::HashMap;

fn main() {
    // 인자를 받아서 처리
    let matches = arg::build_cli().get_matches();

    let arg_num = matches.get_many::<String>("numbers").unwrap();
    let (num1, num2) = numbers::parse_numbers(Some(arg_num.map(|s| s.to_string())));

    //HashMap을 사용하여 플래그와 연산 함수를 매핑
    let arg_map: HashMap<&str, fn(i32, i32) -> i32> = HashMap::from([
        ("add", numbers::sum_numbers as fn(i32, i32) -> i32),
        ("subtract", numbers::subtract_numbers as fn(i32, i32) -> i32),
        (
            "multiplication",
            numbers::multiplication_numbers as fn(i32, i32) -> i32,
        ),
    ]);

    let mut oper_dec = false;

    for (flag, operation) in &arg_map {
        if matches.get_flag(flag) {
            let result = operation(num1, num2);
            println!("{}", result);
            oper_dec = true;
            break;
        }
    }
    if !oper_dec {
        eprintln!(
            "Error: Please specify an operation: -a to add, -s to subtract, or -m to multiply"
        );
    }
}
