use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Trait définissant les opérations d'un cache LRU.
pub trait LRUCache<K, V> {
    /// Insère une paire clé-valeur dans le cache.
    fn put(&mut self, key: K, value: V);

    /// Récupère une valeur du cache par sa clé.
    fn get(&mut self, key: &K) -> Option<&V>;
}

/// Un nœud dans la liste doublement chaînée pour suivre l'ordre d'utilisation.
#[derive(Debug)]
struct Node<K> {
    prev: Option<K>,
    next: Option<K>,
}

/// Un cache LRU générique.
#[derive(Debug)]
pub struct Cache<K: Eq + Hash + Clone, V> {
    capacity: usize,
    map: HashMap<K, (V, Node<K>)>,
    head: Option<K>, // Le plus récemment utilisé
    tail: Option<K>, // Le moins récemment utilisé
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    /// Crée un nouveau `Cache` avec une capacité donnée.
    ///
    /// # Exemple
    ///
    /// ```
    /// use cachelru::cache::Cache;
    ///
    /// let mut cache: Cache<&str, i32> = Cache::new(3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    /// Crée un nouveau `Cache` persistant avec une capacité donnée et un fichier de stockage.
    ///
    /// # Exemple
    ///
    /// ```
    /// use cachelru::cache::Cache;
    ///
    /// let mut cache: Cache<String, String> = Cache::new_persistent(3, "mon_cache.txt");
    /// ```
    pub fn new_persistent(capacity: usize, filename: &str) -> Self
    where
        K: std::fmt::Display + std::str::FromStr,
        V: std::fmt::Display + std::str::FromStr,
    {
        let mut cache = Cache::new(capacity);
        cache.load_from_file(filename).unwrap_or_else(|_| ());
        cache
    }

    /// Sauvegarde le cache dans un fichier.
    pub fn save_to_file(&self, filename: &str) -> io::Result<()>
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        let mut file = File::create(filename)?;
        for (key, (value, _)) in &self.map {
            writeln!(file, "{}\t{}", key, value)?;
        }
        Ok(())
    }

    /// Charge le cache depuis un fichier.
    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()>
    where
        K: std::fmt::Display + std::str::FromStr,
        V: std::fmt::Display + std::str::FromStr,
    {
        if !Path::new(filename).exists() {
            return Ok(());
        }
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split('\t');
            if let (Some(k_str), Some(v_str)) = (parts.next(), parts.next()) {
                if let (Ok(key), Ok(value)) = (k_str.parse::<K>(), v_str.parse::<V>()) {
                    self.put(key, value);
                }
            }
        }
        Ok(())
    }

    /// Supprime un nœud de sa position actuelle dans la liste.
    fn remove_node(&mut self, key: &K) {
        let node = self.map.get(key).unwrap();
        let prev_key_opt = node.1.prev.clone();
        let next_key_opt = node.1.next.clone();

        if let Some(ref prev_key) = prev_key_opt {
            let prev_node = self.map.get_mut(prev_key).unwrap();
            prev_node.1.next = next_key_opt.clone();
        } else {
            self.head = next_key_opt.clone();
        }

        if let Some(ref next_key) = next_key_opt {
            let next_node = self.map.get_mut(next_key).unwrap();
            next_node.1.prev = prev_key_opt.clone();
        } else {
            self.tail = prev_key_opt.clone();
        }
    }

    /// Ajoute un nœud en tête de la liste (le plus récemment utilisé).
    fn add_to_head(&mut self, key: K) {
        let node = self.map.get_mut(&key).unwrap();
        node.1.prev = None;
        node.1.next = self.head.clone();

        if let Some(ref old_head_key) = self.head {
            let old_head_node = self.map.get_mut(old_head_key).unwrap();
            old_head_node.1.prev = Some(key.clone());
        }

        self.head = Some(key.clone());

        if self.tail.is_none() {
            self.tail = Some(key);
        }
    }

    /// Déplace un nœud en tête de la liste (le marque comme le plus récemment utilisé).
    fn move_to_head(&mut self, key: &K) {
        self.remove_node(key);
        self.add_to_head(key.clone());
    }

    /// Supprime le nœud le moins récemment utilisé (en queue de liste).
    fn remove_tail(&mut self) {
        if let Some(tail_key) = self.tail.clone() {
            self.remove_node(&tail_key);
            self.map.remove(&tail_key);
        }
    }

}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> for Cache<K, V> {
    /// Insère une paire clé-valeur dans le cache.
    ///
    /// # Exemple
    ///
    /// ```
    /// use cachelru::cache::{Cache, LRUCache};
    ///
    /// let mut cache: Cache<&str, i32> = Cache::new(3);
    /// cache.put("A", 1);
    /// ```
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.remove_node(&key);
        } else {
            if self.map.len() == self.capacity {
                self.remove_tail();
            }
        }

        self.map.insert(
            key.clone(),
            (
                value,
                Node {
                    prev: None,
                    next: None,
                },
            ),
        );
        self.add_to_head(key);
    }

    /// Récupère une valeur du cache par sa clé.
    ///
    /// # Exemple
    ///
    /// ```
    /// use cachelru::cache::{Cache, LRUCache};
    ///
    /// let mut cache: Cache<&str, i32> = Cache::new(3);
    /// cache.put("A", 1);
    /// assert_eq!(cache.get(&"A"), Some(&1));
    /// ```
    fn get(&mut self, key: &K) -> Option<&V> {
        if !self.map.contains_key(key) {
            return None;
        }
        self.move_to_head(key);
        Some(&self.map.get(key).unwrap().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3); // Taille de 3
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Cache == [B, C, D]

        let my_value = cache.get(&"A");
        assert_eq!(my_value, None);

        let my_value = cache.get(&"D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get(&"C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get(&"X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get(&"D");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]
    }

    #[test]
    fn test_persistent_cache() {
        let filename = "test_cache.txt";

        {
            let mut cache: Cache<String, String> = Cache::new_persistent(3, filename);
            cache.put("A".to_string(), "value_a".to_string());
            cache.put("B".to_string(), "value_b".to_string());
            cache.save_to_file(filename).unwrap();
        }

        {
            let mut cache: Cache<String, String> = Cache::new_persistent(3, filename);
            assert_eq!(cache.get(&"A".to_string()), Some(&"value_a".to_string()));
            assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string()));
        }

        // Nettoyage du fichier de test
        std::fs::remove_file(filename).unwrap();
    }
}
