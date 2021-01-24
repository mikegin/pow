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
    let u8_max: u8 = 255;
    let mut target: [u8; 32] = [u8_max; 32];
    let u8_half: u8 = 15;

    //1 -> 0F loop 0 times, target[loop_times] = 0F
    //2 -> 00 loop 1 times
    //3 -> 000F loop 1 times, target[loop_times] = 0F
    //4 -> 0000 loop 2 times
    //5 -> 00000F loop 2 times, target[loop_times] = 0F

    let is_even = difficulty % 2 == 0;
    let loop_count = difficulty / 2;

    for i in 0..loop_count {
        target[i] = 0;
    }

    if !is_even {
        target[loop_count] = u8_half;
    }

    // println!("{:?}", target);
    // let target_hex = hex::encode(target);
    // println!("target_hex {:?}", target_hex);

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
    let immu_copy = data_hex.clone();
    let mut target_hex = hex::encode(target);

    while data_hex > target_hex {
        println!("data_hex {:?}, nonce {}", data_hex, data.nonce);
        data.nonce += 1;
        data_hex = hex::encode(hash_data(&data));
        target_hex = hex::encode(target);
    }


    println!("initial data_hex {:?}", immu_copy);
    println!("target_hex {:?}", target_hex);
    println!("POW hex: {:?}", data_hex);
    println!("POW: {:?}", data);

    data
}
