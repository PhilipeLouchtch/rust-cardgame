use rand::Rng;
use rand::OsRng;

pub trait Shufflable {
    fn shuffle(&mut self);
}

impl <T> Shufflable for Vec<T> {
    fn shuffle(&mut self) {
        let mut rng = match OsRng::new() {
            Ok(rng) => rng,
            Err(_) => panic!("Failed to get OS Rng, cannot continue"),
        };

        let len = self.len();
        for i in 0..len {
            let random_number: usize = rng.gen();
            self.swap(i, i + (random_number % (len - i)));
        }
    }
}