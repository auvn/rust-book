use std::io::stdout;

use log::info;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    let mut var = 4;
    info!("var is {var}");
    var = 5;
    info!("var is {var}");

    let var = 10;

    {
        let var = 11;
        info!("shadowed var in a block is {var}");
    }
    info!("shadowed var is {var}");

    let str_var: &str = "test";
    let string_var: String = String::from("test");
    info!("string vars: {str_var} {string_var}");

    let num_var = "10".parse::<i32>().expect("should be a number");
    info!("num var {num_var}");

    let num_var = 10u8;
    info!("num var {num_var}");

    let bits = 0b1111_1110;

    info!("bits var {bits}");

    let tuple_var = (1, 0x34, 3.0);
    {
        let (a, b, c) = tuple_var;
        info!("{a} {b} {c}");
    }

    let _vec: [i32; 3] = [1, 2, 3];
    let vec2: [&str; 5] = ["a", "b", "c", "d", "f"];

    for l in vec2 {
        info!("vec2 item is {l}");
    }

    let expr_result = {
        let my_var = 4;
        my_var + 2
    };

    info!("expr result {expr_result}");

    let cond_var = if expr_result > 5 { 10 } else { 125 };

    info!("cond var {cond_var}");

    let loop_var = loop {
        let mut a: i32 = 0;
        a += 1;

        if a > 0 {
            break a;
        }
    };

    info!("loop var is {loop_var}");

    let fib10 = fib(10);

    info!("fib 10 num is {fib10}");
}

fn fib(n: i32) -> i64 {
    let mut n = n - 2;
    let mut f1: i64 = 0;
    let mut f2: i64 = 1;

    while n > 0 {
        let v = f1 + f2;
        f1 = f2;
        f2 = v;
        n -= 1;
    }

    f2
}
