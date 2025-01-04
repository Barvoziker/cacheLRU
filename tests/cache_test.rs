use cachelru::cache::{Cache, LRUCache};

#[test]
fn test_lru_cache_integration() {
    let mut cache = Cache::new(2); // Taille de 2
    cache.put("key1", "value1");
    cache.put("key2", "value2");
    assert_eq!(cache.get(&"key1"), Some(&"value1"));
    cache.put("key3", "value3"); // Ceci doit évincer "key2"
    assert_eq!(cache.get(&"key2"), None);
    assert_eq!(cache.get(&"key3"), Some(&"value3"));
}

#[test]
fn test_persistent_cache_integration() {
    let filename = "test_cache_integration.txt";

    {
        let mut cache: Cache<String, String> = Cache::new_persistent(2, filename);
        cache.put("key1".to_string(), "value1".to_string());
        cache.save_to_file(filename).unwrap();
    }

    {
        let mut cache: Cache<String, String> = Cache::new_persistent(2, filename);
        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
    }

    std::fs::remove_file(filename).unwrap();
}
