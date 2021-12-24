use rand::prelude::*;

type Bytes = Vec<u8>;

pub fn bytes(size: usize) -> Bytes {
    let sz: usize = if size < 2 {
        2
    } else {
        size
    };

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

#[cfg(test)]
mod tests {
    use crate::{bytes, bytes_to_string};

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
        let v = vec![ 0, 1, 2, 3, 255 ];
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
