use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc,  Mutex};
use adjacency_list::AdjacencyNodeList;

pub struct EGraph {
    head_nodes: BTreeMap<usize, usize>,
    last_edge: usize,
    nodes: BTreeMap<usize, usize>,
}

pub enum Expr {
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Atom {
        value: f32
    },
}

impl Expr {
    pub fn into_expression(self) {
         match self {
             Expr::Add { left, right } => {
                    left.into_expression();
                    right.into_expression();
             }
             Expr::Sub { left, right } => {
                    left.into_expression();
                    right.into_expression();
             }
             Expr::Atom { value } => {

             }
         }
    }
}

pub struct EVariable {

}

pub static mut SYMBOL_TABLE: LazyLock<Mutex<Vec<u8>>> = None;

use std::sync::{Once};
use std::time::Duration;
use std::{mem::MaybeUninit, thread};

struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Mutex<u8>,
}

fn singleton() -> &'static SingletonReader {
    // Create an uninitialized static
    static mut SINGLETON: MaybeUninit<SingletonReader> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Mutex::new(0),
            };
            // Store it to the static var, i.e. initialize it
            SINGLETON.write(singleton);
        });

        // Now we give out a shared reference to the data, which is safe to use
        // concurrently.
        SINGLETON.assume_init_ref()
    }
}

fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let s = singleton();
                let mut data = s.inner.lock().unwrap();
                *data = i as u8;
            })
        })
        .collect();

    // And let's check the singleton every so often
    for _ in 0u8..20 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("It is: {}", *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

pub struct SymbolTable {
    symbols: BTreeSet<Arc<String>>,
}

pub struct Symbol {
    intern: Arc<String>,
}

impl Symbol {
    pub fn new(intern: &str) -> Self {
        Self {
            intern: Arc::new(intern.to_string()),
        }
    }
}



pub struct Expression<H> {
    head: EHead<H>,
    rest: Vec<ENode<Self>>,
}

pub struct EHead<H> {
    id: usize,
    data: H,
}

pub struct ENode<N> {
    id: usize,
    data: N,
}

pub struct ENeighbors {
    end_nodes: BTreeMap<u32, u32>,
    links: BTreeMap<u32, u32>,
}
