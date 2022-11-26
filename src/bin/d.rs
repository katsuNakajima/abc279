#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn fall_time(a: f64, b: f64, t: f64) -> f64 {
    let g = t + 1_f64;
    b * t + a / g.sqrt()
}

fn main() {
    let (a, b) = parse_line!(f64, f64);
    let mut before = fall_time(a, b, 0.0);
    let mut after = fall_time(a, b, 1.0);
    let mut cnt = 2.0;
    let mut ans = 0.0;
    loop {
        if before < after {
            ans = before;
            break;
        } else {
            before = after;
            after = fall_time(a, b, cnt);
            cnt += 1.0;
        }
    }
    println!("{}", ans);
}
