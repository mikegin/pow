use std::hash::{Hash};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use generic_array::{GenericArray};
use hex;


#[derive(Debug)]
#[derive(Hash)]
#[derive(Serialize, Deserialize)]
struct Data {
    id: u32,
    name: String,
    phone: u64,
    nonce: u32
}

fn main() {
    let person = Data {
        id: 1,
        name: "Bob1".to_string(),
        phone: 1,
        nonce: 0
    };

    let difficulty = 3;
    let mut target: [u8; 32] = [0; 32];
    let u8_max: u8 = 255;

    for (i, item) in target.iter_mut().enumerate() {
        if i >= difficulty {
            *item = u8_max;
        }
    }

    pow(person, &target);
}

fn hash_data<T: Hash + Serialize>(data: T) -> GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>> {
    let mut hasher = Sha256::new();
    let bytes = bincode::serialize(&data).unwrap();
    hasher.update(bytes);
    let result = hasher.finalize();
    result
}

fn pow(mut data: Data, target: &[u8]) -> Data {
    let mut data_hex = hex::encode(hash_data(&data));
    let mut target_hex = hex::encode(target);
    while data_hex > target_hex {
        println!("data_hex {:?}", data_hex);
        println!("target_hex {:?}", target_hex);
        data.nonce += 1;
        data_hex = hex::encode(hash_data(&data));
        target_hex = hex::encode(target);
    }

    println!("POW: {:?}", data);
    println!("POW hex: {:?}", data_hex);

    data
}
