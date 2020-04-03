#![allow(dead_code)]

mod to_bench;

use crate::to_bench::{bitshift_byte, max_f32_multiplication, max_u64_multiplications};
use benchlib::benching::Bencher;
use rayon::prelude::*;

pub fn noop() {}

pub fn main() {
    let mut bencher = Bencher::new();
    bencher
        .set_iterations(10000)
        .print_settings()
        .bench("Empty closure", || {})
        .bench("Empty fn", noop)
        .bench("Empty loop to 1000", || for _ in 0..1000 {})
        .bench("u64::MAX x u64::MAX 1000 times", || {
            max_u64_multiplications(1000)
        })
        .bench("f32::MAX x f32::MAX 1000 times", || {
            max_f32_multiplication(1000)
        })
        .bench("Bitshift u16 1 byte 1000 times", || bitshift_byte(1000))
        .bench("Multiply to 100", || to_bench::multiply_to(100))
        .bench("Summation from 0u32 to 10000", || {
            to_bench::summation_to::<u32>(10000)
        })
        .bench("Summation from 0u64 to 10000", || {
            to_bench::summation_to::<u64>(10000)
        })
        .compare()
        .bench("Summation from 0u128 to 10000", || {
            to_bench::summation_to::<u128>(10000)
        })
        .compare()
        .bench("Parallel summation using rayon from 0u128 to 10000", || {
            (0u128..10000).into_par_iter().sum::<u128>()
        })
        .compare()
        .bench(
            "Parallel summation with arc mutex from 0u128 to 10000",
            || to_bench::summation_using_mutex(10000),
        )
        .compare()
        .set_iterations(1000)
        .print_settings()
        .bench("Spawn and stop thread", || to_bench::start_stop_thread())
        .bench("Start and stop threads==cpus", || {
            to_bench::start_and_wait_for_num_cpu_threads()
        })
        .compare()
        .bench("MPSC channel transmit 1000x u128", || {
            to_bench::send_mpsc_channel()
        })
        .bench("MPMC channel transmit 1000x u128", || {
            to_bench::send_mpmc_channel()
        })
        .compare()
        .bench("Largest prime until 1000000", || {
            to_bench::largest_prime(1000000)
        })
        .bench("Largest prime parallel until 1000000", || {
            to_bench::largest_prime_par(1000000)
        })
        .compare();
}

#[cfg(test)]
pub mod test {
    use crate::to_bench::{largest_prime, largest_prime_par, summation_to, summation_using_mutex};

    #[test]
    pub fn summation_works() {
        assert_eq!(summation_to(1000), summation_using_mutex(1000))
    }

    #[test]
    pub fn primes_work() {
        assert_eq!(largest_prime(10000), largest_prime_par(10000))
    }
}
