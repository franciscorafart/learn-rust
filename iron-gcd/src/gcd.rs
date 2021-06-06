extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

pub fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("value for 'n' parameter not a number: {:?}\n", unparsed));
                return Ok(response);
            }

            Ok(n) => { numbers.push(n); }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n", numbers, d));

    Ok(response)
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