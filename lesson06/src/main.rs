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
        let ts = Instant::now() + ttl;
        let old_pair = self.map.insert(key, (value, ts));

        if old_pair.is_some() {
            self.clean_ttl(old_pair.unwrap().1, key);
        }
        self.ttl.entry(ts)
            .and_modify(|keys| {
                keys.insert(key);
            })
            .or_insert(HashSet::from([key]));

        old_pair
    }

    pub fn remove(&mut self, key: &'key str) -> Option<(&usize, Instant)> {
        let removed_pair = self.map.remove(key);
        if removed_pair.is_some() {
            self.clean_ttl(removed_pair.unwrap().1, key);
        }
        removed_pair
    }

    fn clean_ttl(&mut self, ts: Instant, key: &str) {
        self.ttl.entry(ts).and_modify(|keys| {
            keys.remove(key);
        });
        if self.ttl.get(&ts).is_some() {
            self.ttl.remove(&ts);
        }
    }

    pub fn get(&self, key: &'key str) -> Option<(&usize, Instant)> {
        let val_ts = self.map.get(key);
        if val_ts.is_some() {
            Some(*val_ts.unwrap())
        } else {
            None
        }
    }

    pub fn expire(&mut self) {
        let now = &Instant::now();
        self.ttl.retain(|ts, keys| {
            let expired = ts <= now;
            if expired {
                keys.iter().for_each(|key| {
                    self.map.remove(*key);
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
