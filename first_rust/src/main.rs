use std::io::Write; 
use std::str::FromStr;

mod my_mod {
    pub fn gcd(mut n: u64, mut m:u64) -> u64 {
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                let t =m;
                m = n;
                n = t;
            }
            m = m % n;
        }
        n
    }
}

fn main(){
    let a: u64 = my_mod::gcd(4,12);
    println!("Greater common denominator: {}", a); 

    println!("Interactive!");
    let mut numbers = Vec::new();
    
        for arg in std::env::args().skip(1) {
            numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
        }
    
        if numbers.len() == 0 {
            writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
            std::process::exit(1);
        }
    
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = my_mod::gcd(d, *m);
        }
    
        println!("The greates common divisor of {:?} is {}", numbers, d);
}

#[test]
fn test_gcd(){
    assert_eq!(my_mod::gcd(3,12), 3);
    assert_eq!(my_mod::gcd(4,12), 4);
    assert_eq!(my_mod::gcd(12,18), 6);
}