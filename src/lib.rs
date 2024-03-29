use num_bigint::BigUint;
use rand::prelude::*;

type Bytes = Vec<u8>;

pub fn bytes(size: usize) -> Bytes {
    let sz: usize = if size < 2 { 2 } else { size };

    let mut results = vec![0u8; sz];

    rand::thread_rng().fill_bytes(&mut results);

    results
}

pub fn bytes_to_string(bytes: Bytes) -> String {
    let mut results = vec![];

    for byte in bytes {
        results.push(format!("{:02x}", byte));
    }

    results.into_iter().rev().collect()
}

pub fn base36_bytes(size: usize) -> Vec<u8> {
    let sz: usize = if size < 2 { 2 } else { size };

    let mut rng = rand::thread_rng();
    let mut results = vec![0u8; 0];

    for _ in 0..sz {
        results.push(rng.gen_range(0..36));
    }

    results
}

pub fn base36_key(size: usize) -> String {
    let bytes = base36_bytes(size);
    let mut results = vec![];

    for byte in bytes {
        results.push(std::char::from_digit(byte as u32, 36).unwrap())
    }

    results.into_iter().rev().collect()
}

pub fn big_random(size: usize, base: u32) -> String {
    let mut rng = rand::thread_rng();

    let mut bytes: Vec<u32> = vec![];
    for _ in 0..size {
        bytes.push(rng.gen());
    }

    let r = BigUint::new(bytes);

    r.to_str_radix(base)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn big_random_test() {
        let sz: usize = 8;
        let s = big_random(sz, 36);
        assert!(s.len() > 48);
    }

    #[test]
    fn base36_bytes_test() {
        let sz: usize = 32;
        let v = base36_bytes(sz);
        println!("{:?}", v);
        assert_eq!(v.len(), sz);
        for b36 in v {
            assert!(b36 < 36);
        }
    }

    #[test]
    fn base36_key_test() {
        let sz: usize = 20;
        let key = base36_key(sz);
        println!("{}", key);
        assert_eq!(key.len(), sz);
    }

    #[test]
    fn bytes_to_random_string_test() {
        let sz = 32;
        let v = bytes(sz);
        println!("{:?}", v);
        let str: String = bytes_to_string(v);
        assert_eq!(str.len(), sz * 2);
        println!("{}", str);
    }

    #[test]
    fn bytes_to_string_test() {
        let v = vec![0, 1, 2, 3, 255];
        println!("{:?}", v);
        let str: String = bytes_to_string(v);
        println!("{}", str);
        assert_eq!(str, "ff03020100");
    }

    #[test]
    fn bytes_min_test() {
        let v = bytes(2);
        println!("{:?}", v);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn bytes_under_min_test() {
        let v = bytes(0);
        println!("{:?}", v);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn bytes_test() {
        let v = bytes(64);
        println!("{:?}", v);
        assert_eq!(v.len(), 64);
    }

    #[test]
    fn bytes_big_test() {
        let v = bytes(128);
        println!("{:?}", v);
        assert_eq!(v.len(), 128);
    }
}
