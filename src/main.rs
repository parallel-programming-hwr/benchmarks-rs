#![allow(dead_code)]

mod to_bench;

use benchlib::benching::Bencher;

pub fn main() {
    let mut bencher = Bencher::new();
    bencher
        .set_iterations(10000)
        .print_settings()
        .bench("Dry Run", || {})
        .bench("Multiply to 100", || to_bench::multiply_to(100))
        .bench("Summation from 0u128 to 1000", || {
            to_bench::summation_to(1000)
        })
        .bench(
            "Parallel summation with arc mutex from 0u128 to 1000",
            || to_bench::summation_using_mutex(1000),
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
