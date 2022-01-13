use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let mut hash1: Vec<u8> = Vec::new();
    let mut hash2: Vec<u8> = Vec::new();

    //+2
    hash1.push(0b00110011);
    hash2.push(0b00010001);
    //+5
    hash1.push(0b00110011);
    hash2.push(0b01010100);
    //+4
    hash1.push(0b11110101);
    hash2.push(0b01011001);
    //+6
    hash1.push(0b00110011);
    hash2.push(0b11000000);

    //total = 17

    let res = hamming(hash1, hash2).unwrap();

    assert_eq!(res, 17);

    println!("{}", res);
    Ok(())
}

fn hamming (hash1 : Vec<u8> , hash2 : Vec<u8>) -> Result<i16, anyhow::Error> {

    if hash1.len() != hash2.len() {
        return Err(anyhow!("length mismatch"));
    }

    let mut ret: i16 = 0;
    for itr in hash1.iter().zip(hash2.iter()) {
 
        let (byte1, byte2) = itr;
        let xor = byte1 ^ byte2;

        ret += xor.count_ones() as i16;
    };

    Ok(ret)
}
