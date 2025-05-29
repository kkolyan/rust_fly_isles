use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use macroquad::logging::{info, trace};

struct Perf {
    prev: Instant,
    aggregated: HashMap<&'static str, Vec<f32>>,
}

impl Default for Perf {
    fn default() -> Self {
        Perf {
            prev: Instant::now(),
            aggregated: Default::default(),
        }
    }
}

thread_local! {
    static PERF: RefCell<Perf> = {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap();
        assert_eq!(thread_name, "main");
        Default::default()
    }
}

pub fn perf_task(name: &'static str) {
    PERF.with(|perf| {
        let mut perf = perf.borrow_mut();
        let now = std::time::Instant::now();
        let passed = now - perf.prev;
        trace!("[PERF] {}: {:.05}s", name, passed.as_secs_f32());
        perf.prev = now;

        perf.aggregated.entry(name)
            .or_default()
            .push(passed.as_secs_f32());
    });
}

#[derive(Debug)]
struct OrdF32(f32);

impl Eq for OrdF32 {}

impl PartialEq<Self> for OrdF32 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for OrdF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Greater;
        }
        if self.0 < other.0 {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

pub fn perf_report() {
    PERF.with(|perf| {
        let perf = perf.borrow();
        let mut agg: Vec<(&&str, &Vec<f32>)> = perf.aggregated.iter().collect();
        agg.sort_by_key(|(k, v)| {
            let x = v.iter().copied().reduce(|a,b| a + b).unwrap();
            OrdF32(x)
        });

        for (name, metrics) in &mut agg {
            // metrics.sort_by_key(|it| (*it * 1000.0) as i32);
            let metrics: Vec<OrdF32> = metrics.iter().map(|it| OrdF32(*it)).collect();
            let min = metrics.iter().min().unwrap();
            let max = metrics.iter().max().unwrap();
            let sum = metrics.iter().map(|it| it.0).reduce(|a, b| a + b).unwrap();
            let avg: f32 = sum / metrics.len() as f32;
            info!("{:.05}s: {:.05}-{:.05} ~{:.05} x{:05} ({})", sum, min.0, max.0, avg, metrics.len(), name);
        }
    });
}
