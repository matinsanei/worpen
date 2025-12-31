use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Global telemetry counters for performance monitoring
static TOTAL_REQUESTS: AtomicU64 = AtomicU64::new(0);
static TOTAL_TIME_NANOS: AtomicU64 = AtomicU64::new(0);
static TIME_CACHE_ACCESS_NANOS: AtomicU64 = AtomicU64::new(0);
static TIME_INPUT_INJECTION_NANOS: AtomicU64 = AtomicU64::new(0);
static TIME_VM_EXECUTION_NANOS: AtomicU64 = AtomicU64::new(0);

/// Record performance metrics for a single request execution
pub fn record_metrics(cache_access_ns: u64, input_injection_ns: u64, vm_execution_ns: u64) {
    // Increment request count
    TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed);

    // Add to total time (sum of all phases)
    let total_ns = cache_access_ns + input_injection_ns + vm_execution_ns;
    TOTAL_TIME_NANOS.fetch_add(total_ns, Ordering::Relaxed);

    // Add to individual phase counters
    TIME_CACHE_ACCESS_NANOS.fetch_add(cache_access_ns, Ordering::Relaxed);
    TIME_INPUT_INJECTION_NANOS.fetch_add(input_injection_ns, Ordering::Relaxed);
    TIME_VM_EXECUTION_NANOS.fetch_add(vm_execution_ns, Ordering::Relaxed);
}

/// Print current performance statistics and reset counters
pub fn print_stats() {
    let requests = TOTAL_REQUESTS.load(Ordering::Relaxed);

    if requests == 0 {
        println!("🔍 Telemetry: No requests processed yet");
        return;
    }

    let total_time_ns = TOTAL_TIME_NANOS.load(Ordering::Relaxed);
    let cache_time_ns = TIME_CACHE_ACCESS_NANOS.load(Ordering::Relaxed);
    let inject_time_ns = TIME_INPUT_INJECTION_NANOS.load(Ordering::Relaxed);
    let exec_time_ns = TIME_VM_EXECUTION_NANOS.load(Ordering::Relaxed);

    // Convert to microseconds for readability
    let avg_total_us = Duration::from_nanos(total_time_ns / requests).as_micros() as f64;
    let avg_cache_us = Duration::from_nanos(cache_time_ns / requests).as_micros() as f64;
    let avg_inject_us = Duration::from_nanos(inject_time_ns / requests).as_micros() as f64;
    let avg_exec_us = Duration::from_nanos(exec_time_ns / requests).as_micros() as f64;

    println!("🔍 Performance Telemetry ({} requests):", requests);
    println!("  📊 Avg Total Time: {:.2} μs", avg_total_us);
    println!("  🗄️  Avg Cache Access: {:.2} μs", avg_cache_us);
    println!("  📥 Avg Input Injection: {:.2} μs", avg_inject_us);
    println!("  ⚡ Avg VM Execution: {:.2} μs", avg_exec_us);
    println!("  📈 Breakdown: Cache={:.1}%, Inject={:.1}%, Exec={:.1}%",
        (avg_cache_us / avg_total_us) * 100.0,
        (avg_inject_us / avg_total_us) * 100.0,
        (avg_exec_us / avg_total_us) * 100.0
    );

    // Reset counters for next measurement period
    TOTAL_REQUESTS.store(0, Ordering::Relaxed);
    TOTAL_TIME_NANOS.store(0, Ordering::Relaxed);
    TIME_CACHE_ACCESS_NANOS.store(0, Ordering::Relaxed);
    TIME_INPUT_INJECTION_NANOS.store(0, Ordering::Relaxed);
    TIME_VM_EXECUTION_NANOS.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_recording() {
        // Reset counters
        TOTAL_REQUESTS.store(0, Ordering::Relaxed);
        TOTAL_TIME_NANOS.store(0, Ordering::Relaxed);
        TIME_CACHE_ACCESS_NANOS.store(0, Ordering::Relaxed);
        TIME_INPUT_INJECTION_NANOS.store(0, Ordering::Relaxed);
        TIME_VM_EXECUTION_NANOS.store(0, Ordering::Relaxed);

        // Record some test metrics
        record_metrics(1000, 2000, 3000); // 6μs total
        record_metrics(1500, 2500, 3500); // 7.5μs total

        // Check counters
        assert_eq!(TOTAL_REQUESTS.load(Ordering::Relaxed), 2);
        assert_eq!(TOTAL_TIME_NANOS.load(Ordering::Relaxed), 13500); // 1000+2000+3000 + 1500+2500+3500
        assert_eq!(TIME_CACHE_ACCESS_NANOS.load(Ordering::Relaxed), 2500); // 1000+1500
        assert_eq!(TIME_INPUT_INJECTION_NANOS.load(Ordering::Relaxed), 4500); // 2000+2500
        assert_eq!(TIME_VM_EXECUTION_NANOS.load(Ordering::Relaxed), 6500); // 3000+3500
    }

    #[test]
    fn test_telemetry_print_stats() {
        // Reset counters
        TOTAL_REQUESTS.store(0, Ordering::Relaxed);
        TOTAL_TIME_NANOS.store(0, Ordering::Relaxed);
        TIME_CACHE_ACCESS_NANOS.store(0, Ordering::Relaxed);
        TIME_INPUT_INJECTION_NANOS.store(0, Ordering::Relaxed);
        TIME_VM_EXECUTION_NANOS.store(0, Ordering::Relaxed);

        // Record test metrics (simulate 1μs = 1000ns per phase)
        record_metrics(1000, 2000, 3000);

        // This should print stats and reset counters
        print_stats();

        // Verify counters were reset
        assert_eq!(TOTAL_REQUESTS.load(Ordering::Relaxed), 0);
        assert_eq!(TOTAL_TIME_NANOS.load(Ordering::Relaxed), 0);
        assert_eq!(TIME_CACHE_ACCESS_NANOS.load(Ordering::Relaxed), 0);
        assert_eq!(TIME_INPUT_INJECTION_NANOS.load(Ordering::Relaxed), 0);
        assert_eq!(TIME_VM_EXECUTION_NANOS.load(Ordering::Relaxed), 0);
    }
}