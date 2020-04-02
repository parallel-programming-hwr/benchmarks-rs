#![feature(test)]

extern crate test;

use rayon::prelude::*;
use std::ops::{Add, Div};
use std::time::{Duration, Instant};
use termion::{color, style};

const BENCHMARK_ITERATIONS: usize = 1000;

pub fn main() {
    bench_function("Spawn and Stop thread", || {
        to_test::start_stop_thread();
    });
    bench_function("MPSC channel", || {
        to_test::send_mpsc_channel();
    });
    bench_function("MPMC channel", || {
        to_test::send_mpmc_channel();
    });
    bench_function("Multiply to 100", || {
        to_test::multiply_to(100);
    });
    bench_function("Largest prime", || {
        to_test::largest_prime(1000000);
    });
    bench_function("Largest prime parallel", || {
        to_test::largest_prime_par(1000000);
    })
}

fn bench_function<F: Fn()>(name: &str, func: F) {
    println!(
        "\n{}{}{}{}",
        color::Fg(color::LightBlue),
        style::Bold,
        name,
        style::Reset
    );
    let (avg_duration, durations) = bench_n_times(BENCHMARK_ITERATIONS, func);
    if durations.len() > 10 {
        println!("Durations(10):\t {:?}...", &durations[0..10]);
    } else {
        println!("Durations:\t {:?}", durations);
    }
    let standard_deviation = (durations.par_iter().sum::<Duration>().as_nanos() as f64
        / (BENCHMARK_ITERATIONS as f64 - 1f64))
        .sqrt();
    println!(
        "Average Duration: {:?} (±{:.2}ns ~ {:.1}%)",
        avg_duration,
        standard_deviation,
        (standard_deviation / avg_duration.as_nanos() as f64) * 100f64
    );
}

fn bench_n_times<F: Fn()>(n: usize, func: F) -> (Duration, Vec<Duration>) {
    let mut durations: Vec<Duration> = Vec::new();
    for _ in 0..n {
        let start = Instant::now();
        func();
        durations.push(start.elapsed());
    }

    (
        durations
            .par_iter()
            .cloned()
            .reduce_with(|a, b| a.add(b).div(2))
            .unwrap(),
        durations,
    )
}

mod to_test {

    use crossbeam_channel::unbounded;
    use rayon::prelude::*;
    use std::sync::mpsc::channel;
    use std::thread;

    pub fn start_stop_thread() {
        let handle = thread::spawn(|| {
            return;
        });
        handle.join().unwrap();
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
        let (rx, tx) = channel::<usize>();
        let handle = thread::spawn(move || for _ in tx {});
        for i in 0..1000 {
            rx.send(i).unwrap();
        }
        std::mem::drop(rx);
        handle.join().unwrap();
    }

    pub fn send_mpmc_channel() {
        let (rx, tx) = unbounded::<usize>();
        let handle = thread::spawn(move || for _ in tx {});
        for i in 0..1000 {
            rx.send(i).unwrap();
        }
        std::mem::drop(rx);
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::to_test::*;
    use test::Bencher;

    #[bench]
    fn bench_start_thread(b: &mut Bencher) {
        b.iter(|| start_stop_thread());
    }

    #[bench]
    fn bench_mpsc_channel(b: &mut Bencher) {
        b.iter(|| send_mpsc_channel())
    }

    #[bench]
    fn bench_mpmc_channel(b: &mut Bencher) {
        b.iter(|| send_mpmc_channel())
    }

    #[bench]
    fn bench_multiply_to_100(b: &mut Bencher) {
        b.iter(|| multiply_to(100));
    }

    #[test]
    fn test_largest_prime_functions() {
        assert_eq!(largest_prime_par(1000), largest_prime(1000))
    }

    #[bench]
    fn bench_largest_prime_1000000(b: &mut Bencher) {
        b.iter(|| largest_prime(1000000));
    }

    #[bench]
    fn bench_largest_prime_1000000_par(b: &mut Bencher) {
        b.iter(|| largest_prime_par(1000000));
    }
}
