use cachelru::cache::{Cache, LRUCache};
use std::env;

fn main() {
    // Afficher le répertoire courant
    let current_dir = env::current_dir().unwrap();
    println!("Répertoire courant : {}", current_dir.display());

    let filename = "mon_cache.txt";

    // Créer un cache persistant avec une capacité de 3
    let mut cache: Cache<String, String> = Cache::new_persistent(3, filename);

    // Ajouter des données au cache
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());
    cache.put("D".to_string(), "value_d".to_string());

    // Sauvegarder le cache dans le fichier
    cache.save_to_file(filename).unwrap();
    println!("Cache sauvegardé dans le fichier {}", filename);
}
