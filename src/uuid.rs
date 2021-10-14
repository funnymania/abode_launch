use rand;
use rand::Rng;

pub fn create() -> [u8; 16] {
    let mut uuid = [0; 16];

    for num in 0..15 {
        uuid[num] = rand::thread_rng().gen::<u8>();
    }

    uuid
}
