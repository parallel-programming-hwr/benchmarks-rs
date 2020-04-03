#![allow(dead_code)]

mod to_bench;

use benchlib::benching::Bencher;

pub fn main() {
    let mut bencher = Bencher::new();
    bencher
        .set_iterations(100000000)
        .print_settings()
        .bench("Multiply to 100", || to_bench::multiply_to(100))
        .bench("Summation from 0u128 to 1000000", || {
            to_bench::summation_to_1000000()
        })
        .set_iterations(1000)
        .print_settings()
        .bench("Spawn and Stop thread", || to_bench::start_stop_thread())
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
