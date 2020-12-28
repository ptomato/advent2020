use std::collections::HashMap;

fn pow_m(base: u64, exponent: usize, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut value = 1;
    let mut mod_base = base % modulus;
    let mut mod_exponent = exponent;
    while mod_exponent > 0 {
        if mod_exponent % 2 == 1 {
            value *= mod_base;
            value %= modulus;
        }
        mod_exponent >>= 1;
        mod_base *= mod_base;
        mod_base %= modulus;
    }
    value
}

fn bsgs(base: u64, modulus: u64, result: u64) -> Option<usize> {
    let m = (modulus as f64).sqrt().ceil() as u64;
    let mut table = HashMap::new();
    let mut e = 1;
    for j in 0..m {
        table.insert(e, j);
        e *= base;
        e %= modulus;
    }
    let factor = pow_m(base, (modulus - m - 1) as usize, modulus);
    let mut gamma = result;
    for i in 0..m {
        if let Some(j) = table.get(&gamma) {
            return Some((i * m + j) as usize);
        }
        gamma *= factor;
        gamma %= modulus;
    }
    None
}

fn transform_subject_number(subject_number: u64, loop_size: usize) -> u64 {
    pow_m(subject_number, loop_size, 20201227)
}

fn guess_loop_size(public_key: u64) -> usize {
    bsgs(7, 20201227, public_key).unwrap()
}

#[derive(Debug)]
struct Party {
    loop_size: usize,
}

impl Party {
    fn public_key(&self) -> u64 {
        transform_subject_number(7, self.loop_size)
    }

    fn encryption_key(&self, other_public_key: u64) -> u64 {
        transform_subject_number(other_public_key, self.loop_size)
    }
}

fn main() {
    let card_public_key = 2084668;
    let door_public_key = 3704642;
    let card = Party {
        loop_size: guess_loop_size(card_public_key),
    };
    let door = Party {
        loop_size: guess_loop_size(door_public_key),
    };
    println!("{}", card.encryption_key(door.public_key()));
}

#[test]
fn test_transform_subject_number() {
    assert_eq!(transform_subject_number(7, 8), 5764801);
    assert_eq!(transform_subject_number(7, 11), 17807724);
    assert_eq!(transform_subject_number(5764801, 11), 14897079);
    assert_eq!(transform_subject_number(17807724, 8), 14897079);
}

#[test]
fn test_loop_size() {
    assert_eq!(guess_loop_size(5764801), 8);
    assert_eq!(guess_loop_size(17807724), 11);
}

#[test]
fn test_public_key() {
    assert_eq!((Party { loop_size: 8 }).public_key(), 5764801);
    assert_eq!((Party { loop_size: 11 }).public_key(), 17807724);
}

#[test]
fn test_encryption_key() {
    let card = Party { loop_size: 8 };
    let door = Party { loop_size: 11 };
    assert_eq!(card.encryption_key(door.public_key()), 14897079);
    assert_eq!(door.encryption_key(card.public_key()), 14897079);
}
