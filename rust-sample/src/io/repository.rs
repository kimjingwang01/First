use crate::domain::models::DataRecord;

#[derive(Default)]
pub struct InMemoryRepository {
    records: Vec<DataRecord>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn store(&mut self, batch: Vec<DataRecord>) -> Result<(), String> {
        if batch.is_empty() {
            return Err("empty batch".to_string());
        }

        for record in batch {
            if record.id == 0 {
                return Err("invalid id".to_string());
            }
            self.records.push(record);
        }
        Ok(())
    }

    pub fn all(&self) -> &[DataRecord] {
        &self.records
    }

    pub fn find_by_category<'a>(&'a self, category: &str) -> Vec<&'a DataRecord> {
        self.records
            .iter()
            .filter(|r| r.category == category)
            .collect::<Vec<_>>()
    }

    pub fn active_count(&self) -> usize {
        self.records.iter().filter(|r| r.active).count()
    }

    pub fn sum_value(&self) -> f64 {
        self.records.iter().map(|r| r.value).sum()
    }

    pub fn average_value(&self) -> f64 {
        if self.records.is_empty() {
            0.0
        } else {
            self.sum_value() / self.records.len() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(id: u64, category: &str, value: f64, active: bool) -> DataRecord {
        DataRecord {
            id,
            category: category.to_string(),
            value,
            tags: Vec::new(),
            active,
        }
    }

    #[test]
    fn store_and_retrieve() {
        let mut repo = InMemoryRepository::new();
        repo.store(vec![
            make(1, "alpha", 10.0, true),
            make(2, "beta", 20.0, false),
            make(3, "alpha", 30.0, true),
        ])
        .unwrap();

        assert_eq!(repo.all().len(), 3);
        assert_eq!(repo.find_by_category("alpha").len(), 2);
        assert_eq!(repo.active_count(), 2);
    }
}
