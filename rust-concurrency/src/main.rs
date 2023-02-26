use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello from main thread! Id = {:?}", thread::current().id());

    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();

    let a = Cell::new(3i32);
    f_cell(&a, &a);

    f_rcell();

    mutex();

    // thread_parking();

    condvar();
}

fn f() {
    println!(
        "Hello from another thread! Id = {:?}",
        thread::current().id()
    );
}

fn f_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("Interior mutabiltiy via Cell")
    }

    // Cell can't let you borrow the value it holds, we need to move a value out (leaving something in its place),
    // modify it, then put it back, to mutate its contents
    let v = &Cell::new(vec![1, 2, 3]);
    let mut v2 = v.take(); // Replaces the contents of the Cell with an empty Vec
    v2.push(4);
    v.set(v2);
}

fn f_rcell() {
    // Unlike a regular Cell, a std::cell::RefCell does allow you to borrow its contents, at a small runtime cost
    let v = &RefCell::new(vec![1, 2, 3]);
    v.borrow_mut().push(4);
    v.borrow_mut().push(4);

    for i in v.borrow().iter() {
        println!("{i}");
    }
}

fn mutex() {
    let m = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = m.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });

    assert_eq!(m.into_inner().unwrap(), 1000);
}

fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(i) = item {
                dbg!(i);
            } else {
                thread::park();
            }
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn condvar() {
    let queue = Mutex::new(VecDeque::new());
    let non_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut g = queue.lock().unwrap();

            let i = loop {
                if let Some(item) = g.pop_front() {
                    break item;
                } else {
                    g = non_empty.wait(g).unwrap();
                }
            };

            drop(g);
            dbg!(i);
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            non_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
