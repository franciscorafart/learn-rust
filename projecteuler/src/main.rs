fn main() {
    println!("Result euler 1: {}", euler1());
    println!("Result eluer 2: {}", euler2());
}

fn euler1() -> i32 {
    let mut m5: i32 = 0;
    let mut m3: i32 = 0;
    let max: i32 = 1000;

    let mut multiples = Vec::new();

    while m5 < max || m3 < max {
        let mut holder = 0;

        if m3 <= m5 {
            m3+=3;
            holder = m3;
        } else if m5 < m3 {
            m5+=5;
            holder = m5;
        }

        if holder < max && m3 != m5 {
            multiples.push(holder);
        }
    }

    sum(multiples)
}

fn sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for element in vec.iter() {
        sum+=element;
    }

    sum
}

fn euler2() -> i32 {
    let mut fib = vec![1];
    let mut fib_even = vec![];

    let mut prev_fib = 1;
    let mut last_fib = 2;

    while last_fib < 4000000 {
        fib.push(last_fib);
        if last_fib % 2 == 0 {
            fib_even.push(last_fib);
        }

        let holder = prev_fib;
        prev_fib = last_fib;
        last_fib = holder + last_fib;
    }

    sum(fib_even)
}

#[test]
fn test_euler() {
    assert_eq!(euler1(), 233168);
    assert_eq!(euler2(), 4613732);
}