use cachelru::cache::{Cache, LRUCache};
use std::time::Instant;

fn main() {
    let mut cache = Cache::new(1000);

    let start = Instant::now();
    for i in 0..1000 {
        cache.put(i, i);
    }
    let duration = start.elapsed();
    println!("Temps pour insérer 1000 éléments : {:?}", duration);

    let start = Instant::now();
    for i in 0..1000 {
        cache.get(&i);
    }
    let duration = start.elapsed();
    println!("Temps pour accéder à 1000 éléments : {:?}", duration);
}
