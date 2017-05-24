use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(1000));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating with {} and {}.", self.name, self.left, self.right);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    println!("Beginning...");
    let p1 = Philosopher::new("AA", 0, 1);
    let p2 = Philosopher::new("BB", 1, 2);
    let p3 = Philosopher::new("CC", 2, 3);
    let p4 = Philosopher::new("DD", 3, 4);
    let p5 = Philosopher::new("EE", 4, 5);


    let pp = Philosopher { name: "PP".to_string(), left: 0, right: 5 };

    let philosophers: Vec<_> = vec![p1, p2, p3, p4, p5, pp];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    //    for p in philosophers {
    //        p.eat();
    //    }

    loop {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_millis(10000));
            println!("thread {}!", thread::current().name().unwrap_or_default());
        });
        handle.join().unwrap();
    }
}
