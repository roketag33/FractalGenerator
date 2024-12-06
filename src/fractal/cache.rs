use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ViewRegion {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    resolution: u32,  // Niveau de détail : 1 = pleine résolution, 2 = demi-résolution, etc.
}

pub struct CacheEntry {
    image: egui::ColorImage,
    timestamp: std::time::Instant,
}

pub struct FractalCache {
    cache: HashMap<ViewRegion, CacheEntry>,
    max_entries: usize,
    max_age: std::time::Duration,
}

impl FractalCache {
    pub fn new(max_entries: usize, max_age_secs: u64) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            max_age: std::time::Duration::from_secs(max_age_secs),
        }
    }

    pub fn get(&mut self, region: &ViewRegion) -> Option<&egui::ColorImage> {
        if let Some(entry) = self.cache.get(region) {
            if entry.timestamp.elapsed() < self.max_age {
                return Some(&entry.image);
            }
        }
        None
    }

    pub fn insert(&mut self, region: ViewRegion, image: egui::ColorImage) {
        // Nettoyer le cache si nécessaire
        if self.cache.len() >= self.max_entries {
            // D'abord, supprimer les entrées expirées
            self.remove_expired_entries();

            // Si toujours trop plein, supprimer les plus anciennes entrées
            if self.cache.len() >= self.max_entries {
                self.remove_oldest_entries();
            }
        }

        self.cache.insert(region, CacheEntry {
            image,
            timestamp: std::time::Instant::now(),
        });
    }

    fn remove_expired_entries(&mut self) {
        let now = std::time::Instant::now();
        self.cache.retain(|_, entry| entry.timestamp.elapsed() < self.max_age);
    }

    fn remove_oldest_entries(&mut self) {
        // Collecter les clés à supprimer dans un vecteur séparé
        let keys_to_remove: Vec<ViewRegion> = {
            let mut entries: Vec<_> = self.cache.iter()
                .map(|(key, entry)| (key.clone(), entry.timestamp))
                .collect();

            entries.sort_by_key(|(_key, timestamp)| *timestamp);

            let to_remove = entries.len() - self.max_entries + 1;
            entries.into_iter()
                .take(to_remove)
                .map(|(key, _)| key)
                .collect()
        };

        // Supprimer les entrées dans une étape séparée
        for key in keys_to_remove {
            self.cache.remove(&key);
        }
    }
}