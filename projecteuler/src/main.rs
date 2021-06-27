

fn main() {
    euler1();
}

fn euler1() {
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

    let result = sum(multiples);

    println!("Result {}", result);
}

fn sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for element in vec.iter() {
        sum+=element;
    }

    sum
}