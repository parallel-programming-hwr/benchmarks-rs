use crossbeam_channel::unbounded;
use crossbeam_utils::sync::WaitGroup;
use num_cpus;
use num_traits::{PrimInt, Unsigned};
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_stop_thread() {
    let handle = thread::spawn(|| {
        return;
    });
    handle.join().unwrap();
}

pub fn summation_to<T: PrimInt + Unsigned>(end: T) -> T {
    let mut res: T = T::zero();
    let mut i = T::zero();
    while i < end {
        res = res + i;
        i = i + T::one();
    }
    res
}

pub fn multiply_to(end: usize) -> f64 {
    let mut result = 0f64;
    for i in 2..end {
        result = (result * i as f64) / (i - 1) as f64;
    }

    result
}

pub fn largest_prime(end: u128) -> u128 {
    let mut last_prime = 2;
    for i in (2u128..end).step_by(2) {
        let mut is_prime = true;
        for j in 2..(i as f64).sqrt().ceil() as u128 {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            last_prime = i;
        }
    }

    last_prime
}

pub fn largest_prime_par(end: u128) -> u128 {
    (2u128..((end as f64) / 2f64).ceil() as u128)
        .into_par_iter()
        .filter(|number| {
            let num = number * 2;
            for i in 2..(num as f64).sqrt().ceil() as u128 {
                if num % i == 0 {
                    return false;
                }
            }

            true
        })
        .max()
        .unwrap()
        * 2
}

pub fn send_mpsc_channel() {
    let (rx, tx) = channel::<u128>();
    let handle = thread::spawn(move || for _ in tx {});
    for i in 0..1000 {
        rx.send(i).unwrap();
    }
    std::mem::drop(rx);
    handle.join().unwrap();
}

pub fn send_mpmc_channel() {
    let (rx, tx) = unbounded::<u128>();
    let handle = thread::spawn(move || for _ in tx {});
    for i in 0..1000 {
        rx.send(i).unwrap();
    }
    std::mem::drop(rx);
    handle.join().unwrap();
}

pub fn summation_using_mutex(end: usize) -> u128 {
    let wg = WaitGroup::new();
    let num = Arc::new(Mutex::new(0u128));

    for thread_number in 0..num_cpus::get() {
        let wg = wg.clone();
        let num = Arc::clone(&num);
        thread::spawn(move || {
            for i in (thread_number..end).step_by(num_cpus::get()) {
                *num.lock().unwrap() += i as u128;
            }
            std::mem::drop(wg);
        });
    }
    wg.wait();
    let result = *num.lock().unwrap();

    result
}

pub fn start_and_wait_for_num_cpu_threads() {
    let wg = WaitGroup::new();

    for thread_number in 0..num_cpus::get() {
        let wg = wg.clone();
        thread::spawn(move || {
            std::mem::drop(wg);
            thread_number
        });
    }
    wg.wait();
}
