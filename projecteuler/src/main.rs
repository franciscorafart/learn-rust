fn main() {
    println!("Result euler 1 - Multiples 3 and 5: {}", multiples_three_five());
    println!("Result eluer 2 - Even Fibonacci numbers: {}", even_fib());
    println!("Result euler 3 - Largest Prime Factor: {}", largest_prime_factor());
}

fn multiples_three_five() -> i32 {
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

fn even_fib() -> i32 {
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


// largest prime factor
fn largest_prime_factor() -> i64 {

    let x = 600851475143;
    let y = (x / 2) as i64;
    let mut v = vec![];

    for n in 2..y {
        if is_prime(n) && x%n == 0 {
            v.push(n);

            let multiplication = multiply_primes(&v);
            if multiplication == x {
                return v[v.len() - 1]
            } else if multiplication > x {
                break;
            }
        }
    }

    x
}

fn is_prime(x: i64) -> bool {
    if x%2 == 0 {
        return false;
    }

    let mut y = x/2;

    while y > 2 {
        if x%y == 0 {
            return false;
        }
        y-=1;
    }

    return true;
}

fn multiply_primes(v: &Vec<i64>) -> i64 {
    let mut result = 1;
    for p in v {
        result = result * p;
    }

    result
}

#[test]
fn test_euler() {
    assert_eq!(multiples_three_five(), 233168);
    assert_eq!(even_fib(), 4613732);
    assert_eq!(largest_prime_factor(), 6857);
}