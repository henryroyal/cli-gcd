fn gcd(a: i64, b: i64) {
    if a == 1 || b == 1 {
        println!("{}", 1);
        return;
    }

    if a == 0 {
        println!("{}", b);
        return;
    }

    if b == 0 {
        println!("{}", a);
        return;
    }

    if a == b {
        println!("{}", a);
        return;
    }

    if a > b {
        let difference = a - b;
        gcd(difference, b);
    } else {
        let difference = b - a;
        gcd(difference, a);
    }
}

fn read_input() -> Vec<i64> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer);

    buffer = buffer.replace("\n", "");
    let mut split = buffer.split_whitespace();

    let mut input: Vec<i64> = Vec::new();
    for item in split {
        let int: i64 = item.parse().unwrap();
        input.push(int);
    }

    if input.len() != 2 {
        println!("input should be a pair of integers!");
        std::process::exit(1);
    }

    return input;
}


fn main() {
    let input = read_input();
    gcd(input[0], input[1]);
}
