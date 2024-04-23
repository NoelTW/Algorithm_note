use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;

#[derive(Debug)]
struct HashTable<K, V> {
    data: Vec<Option<LinkedList<(K, V)>>>,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(size: usize) -> Self {
        HashTable {
            data: vec![None; size],
        }
    }

    fn _hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.data.len()
    }

    fn set(&mut self, key: K, value: V) {
        let key_hash = self._hash(&key);
        match self.data.get_mut(key_hash) {
            Some(bucket) => {
                if let Some(bucket) = bucket {
                    for item in bucket.iter_mut() {
                        if item.0 == key {
                            item.1 = value;
                            return;
                        }
                    }
                    bucket.push_back((key, value));
                } else {
                    self.data[key_hash] = Some(LinkedList::new());
                    self.data[key_hash].as_mut().unwrap().push_back((key, value));
                }
            }
            None => panic!("Index out of bounds"),
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let key_hash = self._hash(&key);
        if let Some(bucket) = self.data.get(key_hash).and_then(|b| b.as_ref()) {
            for item in bucket.iter() {
                if item.0 == *key {
                    return Some(&item.1);
                }
            }
        }
        None
    }

    // fn keys(&self) -> Vec<&K> {
    //     let mut result = Vec::new();
    //     for bucket in self.data.iter().filter_map(|b| b.as_ref()) {
    //         for item in bucket.iter() {
    //             result.push(&item.0);
    //         }
    //     }
    //     result
    // }
}

fn main() {
    let mut my_hash_table: HashTable<&str, i32> = HashTable::new(2);
    my_hash_table.set("grapes", 10000);
    my_hash_table.set("apples", 9);
    println!("{:?}", my_hash_table.get(&"apples"));
    println!("{:?}", my_hash_table.get(&"grapes"));
}
