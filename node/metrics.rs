use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct MetricsData {
    pub queries: u64,
    pub errors: u64,
    pub dhcp_leases: u64,
}

#[derive(Clone)]
pub struct Metrics {
    inner: Arc<Mutex<MetricsData>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MetricsData::default())),
        }
    }

    pub fn inc_queries(&self) {
        let mut m = self.inner.lock().unwrap();
        m.queries += 1;
    }

    pub fn inc_errors(&self) {
        let mut m = self.inner.lock().unwrap();
        m.errors += 1;
    }

    pub fn inc_dhcp_leases(&self) {
        let mut m = self.inner.lock().unwrap();
        m.dhcp_leases += 1;
    }

    pub fn snapshot(&self) -> MetricsData {
        self.inner.lock().unwrap().clone()
    }
}
