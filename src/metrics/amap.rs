use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug)]
pub struct AmapMertrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMertrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let map = metrics_names
            .iter()
            .map(|name| (*name, AtomicI64::new(0)))
            .collect();
        AmapMertrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let conuter = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!("key {} not found", key))?;
        conuter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMertrics {
    fn clone(&self) -> Self {
        AmapMertrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl std::fmt::Display for AmapMertrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
