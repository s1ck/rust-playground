mod hexdump;

use bincode::serialize as to_binc;
use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::to_string as to_json;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn serialize() {
    let leipzig = City {
        name: String::from("Leipzig"),
        population: 605_407,
        latitude: 42.42,
        longitude: 13.37,
    };

    let as_json = to_json(&leipzig).unwrap();
    let as_cbor = to_cbor(&leipzig).unwrap();
    let as_binc = to_binc(&leipzig).unwrap();

    println!("json:\n{:?}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("binc:\n{:?}\n", &as_binc);

    println!(
        "json (as UTF-8):\n{}\n",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!("cbor (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_cbor));
    println!("binc (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_binc));
}

#[cfg(test)]
mod tests {
    use crate::serialize;

    #[test]
    fn it_works() {
        serialize();
    }
}
