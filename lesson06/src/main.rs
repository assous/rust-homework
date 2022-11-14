use std::{
    collections::{HashMap, BTreeMap, HashSet},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cache<'a, 'b> {
    map: HashMap<&'a str, (&'b usize, Instant)>,
    ttl: BTreeMap<Instant, HashSet<&'a str>>,
}

impl Cache<'static, 'static> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
            ttl: Default::default(),
        }
    }
}

impl Cache<'_, '_> {
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<'key, 'value> Cache<'key, 'value> {
    pub fn insert(&mut self, key: &'key str, value: &'value usize, ttl: Duration) -> Option<(&usize, Instant)> {
        let now = Instant::now();
        let expire = now + ttl;

        self.ttl.entry(expire)
            .and_modify(|x| {
                x.insert(key);
            })
            .or_insert(HashSet::from([key]));

        self.map.insert(key, (value, now))
    }

    pub fn remove(&mut self, key: &'key str) -> Option<(&usize, Instant)> {
        self.ttl.retain(|_, v| {
            v.remove(key);
            !v.is_empty()
        });
        self.map.remove(key)
    }

    pub fn get(&self, key: &'key str) -> Option<(&usize, Instant)> {
        let val = self.map.get(key);
        if val.is_some() {
            Some(*val.unwrap())
        } else {
            None
        }
    }

    pub fn expire(&mut self) {
        let now = &Instant::now();
        self.ttl.retain(|k, v| {
            let expired = k <= now;
            if expired {
                v.iter().for_each(|kk| {
                    self.map.remove(*kk);
                });
            }
            !expired
        });
    }
}

fn main() {
    let data = vec![
        (String::from("one"), Box::new(1), Duration::from_secs(5)),
        (String::from("two"), Box::new(2), Duration::from_millis(1)),
        (String::from("three"), Box::new(3), Duration::from_secs(5)),
    ];

    let mut cache = Cache::new();
    data.iter().for_each(|item| {
        cache.insert(&item.0, &item.1, item.2);
    });

    std::thread::sleep(Duration::from_secs(1));
    cache.expire();

    println!("{:#?}", cache);
}
