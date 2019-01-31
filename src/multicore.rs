//! This is an interface for dealing with the kinds of
//! parallel computations involved in bellman. It's
//! currently just a thin wrapper around CpuPool and
//! crossbeam but may be extended in the future to
//! allow for various parallelism strategies.

#[derive(Clone)]
pub struct Worker {}

impl Worker {
    // We don't expose this outside the library so that
    // all `Worker` instances have the same number of
    // CPUs configured.
    pub(crate) fn new_with_cpus() -> Worker {
        Worker {}
    }

    pub fn new() -> Worker {
        // Self::new_with_cpus(num_cpus::get())
        Self::new_with_cpus()
    }

    pub fn log_num_cpus(&self) -> u32 {
        0
    }

    pub fn scope<F, R>(&self, elements: usize, f: F) -> R
    where
        F: FnOnce(&Scope, usize) -> R,
    {
        let scope: Scope = Scope{};
        f(&scope, elements)
    }
}

pub struct Scope {}

impl Scope {
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        f()
    }
}

fn log2_floor(num: usize) -> u32 {
    assert!(num > 0);

    let mut pow = 0;

    while (1 << (pow + 1)) <= num {
        pow += 1;
    }

    pow
}

#[test]
fn test_log2_floor() {
    assert_eq!(log2_floor(1), 0);
    assert_eq!(log2_floor(2), 1);
    assert_eq!(log2_floor(3), 1);
    assert_eq!(log2_floor(4), 2);
    assert_eq!(log2_floor(5), 2);
    assert_eq!(log2_floor(6), 2);
    assert_eq!(log2_floor(7), 2);
    assert_eq!(log2_floor(8), 3);
}

#[test]
fn test_worker_square() {
    let worker = self::Worker::new();
    let mut input = [1, 2, 3, 4];

    worker.scope(input.len(), |scope, chunk| {
        for a in input.chunks_mut(chunk) {
            scope.spawn(move || {
                for b in a {
                    *b = *b + *b;
                }
            });
        }
    });
    assert_eq!(input[0], 2);
    assert_eq!(input[1], 4);
    assert_eq!(input[2], 6);
    assert_eq!(input[3], 8);
}

#[test]
fn test_worker_mult() {
    let worker = self::Worker::new();
    
    let mut input = [1, 2, 3, 4, -5, 123];
    let saved = input.clone();

    worker.scope(input.len(), |scope, chunk| {
        for a in input.chunks_mut(chunk) {
            scope.spawn(move || {
                for b in a {
                    *b = *b * *b;
                }
            });
        }
    });
    for i in 0..input.len() {
        assert_eq!(input[i], saved[i]* saved[i]);
    }
}