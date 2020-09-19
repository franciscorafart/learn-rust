fn main(){
    let a: u64 = gcd(4,12);
    println!("{}", a); 
}

fn gcd(mut n: u64, mut m:u64) -> u64 {
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


#[test]
fn test_gcd(){
    assert_eq!(gcd(3,12), 3);
    assert_eq!(gcd(4,12), 4);
    assert_eq!(gcd(12,18), 6);
}