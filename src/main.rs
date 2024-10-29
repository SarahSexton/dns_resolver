use std::net::IpAddr;
use std::time::{Duration, Instant};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn main() {
    let start_time = Instant::now();

    // Configure the resolver once
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .expect("Failed to create resolver");

    for i in 0..100 {
        let iteration_start_time = Instant::now();

        // Perform the DNS lookup
        let dns_start_time = Instant::now();
        let response = resolver.lookup_ip("www.example.com").expect("Failed to lookup IP");
        let dns_elapsed_time = dns_start_time.elapsed();

        // Iterate over the IPs in the response
        for ip in response.iter() {
            println!("Iteration {}: IP Address: {}", i + 1, ip);

            // Perform the reverse DNS lookup for a known IP with a PTR record
            let known_ip: IpAddr = "8.8.8.8".parse().expect("Invalid IP address");
            let reverse_start_time = Instant::now();
            match resolver.reverse_lookup(known_ip) {
                Ok(reverse_response) => {
                    let reverse_elapsed_time = reverse_start_time.elapsed();
                    for name in reverse_response.iter() {
                        println!("Iteration {}: Hostname for 8.8.8.8: {}", i + 1, name);
                    }
                    println!(
                        "Iteration {}: Reverse DNS lookup time: {:?}",
                        i + 1, reverse_elapsed_time
                    );
                }
                Err(e) => {
                    println!("Iteration {}: Failed to perform reverse lookup for 8.8.8.8: {}", i + 1, e);
                }
            }
        }

        let iteration_elapsed_time = iteration_start_time.elapsed();
        println!(
            "Iteration {}: DNS lookup time: {:?}, Total iteration time: {:?}",
            i + 1, dns_elapsed_time, iteration_elapsed_time
        );
    }

    let total_elapsed_time = start_time.elapsed();
    println!("Total elapsed time for 100 iterations: {:?}", total_elapsed_time);
}

