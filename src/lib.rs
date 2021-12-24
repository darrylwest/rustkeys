use rand::prelude::*;

pub fn bytes(size: usize) -> Vec<u8> {
    const COUNT: usize = 64;
    let mut data = [0u8; COUNT];
    rand::thread_rng().fill_bytes(&mut data);

    let sz: usize = if size > COUNT {
        COUNT
    } else {
        size
    };

    let mut v = vec![];
    for i in 0..sz {
        v.push(data[i]);
    }

    v
}

#[cfg(test)]
mod tests {
    use crate::bytes;

    #[test]
    fn bytes_min_test() {
        let v = bytes(2);
        assert_eq!(v.len(), 2);
        println!("{:?}", v);
    }

    #[test]
    fn bytes_max_test() {
        let v = bytes(64);
        assert_eq!(v.len(), 64);
        println!("{:?}", v);
    }

    #[test]
    fn bytes_over_max_test() {
        let v = bytes(128);
        assert_eq!(v.len(), 64);
        println!("{:?}", v);
    }
}
