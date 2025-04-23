use super::*;

/// POWER TEST SYSTEM

// Resource (Electrical Joule)
struct EJoule {}
impl Resource for EJoule {}

// Source
struct Solar {
    rate: usize,
}
impl Source<EJoule> for Solar {
    fn get_rate(&self) -> usize {
        self.rate
    }
}

// Sink
struct Hydroponics {
    rate: usize,
}
impl Sink<EJoule> for Hydroponics {
    fn get_rate(&self) -> usize {
        self.rate
    }
}

// Storage
struct Battery {
    capacity: usize,
}
impl Storage<EJoule> for Battery {
    fn get_capacity(&self) -> usize {
        self.capacity
    }
}

struct PowerSystem {
    // didn't feel like dealing with + Sized for now
    // sources: Vec<dyn Source<EJoule>>,
    sources: Vec<Solar>,
    sinks: Vec<Hydroponics>,
    storages: Vec<Battery>,
}

impl System for PowerSystem {
    fn compile(&self) {
        let mut total_source_rate = 0;
        for source in &self.sources {
            total_source_rate += source.get_rate();
        }

        let mut total_sink_rate = 0;
        for sink in &self.sinks {
            total_sink_rate += sink.get_rate();
        }

        let mut total_capacity = 0;
        for storage in &self.storages {
            total_capacity += storage.get_capacity();
        }
        // TODO: some math here to see if it checks out
    }
}

#[test]
fn it_works() {
    assert!(true);
}
