//! Scanner Performance Benchmarks
//!
//! This benchmark suite measures the performance of the DevShield scanner
//! with pattern caching and parallel file scanning.
//!
//! # Benchmarks
//!
//! - `scan_100_files`: Scan 100 files with mixed content
//! - `scan_regex_only`: Regex matching performance (no I/O)
//! - `scan_parallel_vs_serial`: Compare parallel vs serial scanning
//! - `pattern_cache_overhead`: Measure pattern cache initialization cost
//!
//! # Running Benchmarks
//!
//! ```bash
//! # Run all benchmarks
//! cargo bench -p coax-scanner
//!
//! # Run specific benchmark
//! cargo bench -p coax-scanner --bench scanner_benchmarks scan_100_files
//!
//! # Run with more samples for accuracy
//! cargo bench -p coax-scanner -- --sample-size 50
//! ```
//!
//! # Target Performance
//!
//! - scan_100_files: <100ms (baseline: ~300ms)
//! - scan_regex_only: <100ms (baseline: ~270ms)
//! - Speedup: 3-5x expected from optimizations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use coax_scanner::{PatternCache, Scanner, ScannerConfig};
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

/// Create test files for benchmarking
fn create_test_files(count: usize, secret_ratio: f64) -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    for i in 0..count {
        let file = temp_dir.path().join(format!("file_{}.txt", i));
        let content = if (i as f64 / count as f64) < secret_ratio {
            // File with secrets
            format!(
                "Some config here\nAWS_KEY=AKIAIOSFODNN7EXAMPLE{i:03}\nGITHUB_TOKEN=ghp_1234567890abcdefghij1234567890abcdefghij{i:03}\nMore content\n"
            )
        } else {
            // Clean file
            format!("Clean content line 1\nClean content line 2\nClean content line 3\n")
        };
        fs::write(&file, content).unwrap();
    }

    temp_dir
}

/// Create test content strings for regex-only benchmarks
fn create_test_content(count: usize, has_secrets: bool) -> Vec<String> {
    (0..count)
        .map(|i| {
            if has_secrets {
                format!(
                    "Config line {}\nAWS_KEY=AKIAIOSFODNN7EXAMPLE{}\nGitHub token: ghp_1234567890abcdefghij1234567890abcdefghij{}\nEnd\n",
                    i, i, i
                )
            } else {
                format!("Clean content line {}\nAnother clean line {}\nFinal line {}\n", i, i, i)
            }
        })
        .collect()
}

/// Benchmark scanning 100 files
fn bench_scan_100_files(c: &mut Criterion) {
    let temp_dir = create_test_files(100, 0.1); // 10% with secrets
    let scanner = Scanner::new();

    let mut group = c.benchmark_group("scan_100_files");
    group.throughput(Throughput::Elements(100));

    group.bench_function("parallel", |b| {
        b.iter(|| {
            let results = scanner.scan_directory(black_box(temp_dir.path()));
            black_box(results);
        })
    });

    group.finish();
}

/// Benchmark scanning 1000 files
fn bench_scan_1000_files(c: &mut Criterion) {
    let temp_dir = create_test_files(1000, 0.05); // 5% with secrets
    let scanner = Scanner::new();

    let mut group = c.benchmark_group("scan_1000_files");
    group.throughput(Throughput::Elements(1000));

    group.bench_function("parallel", |b| {
        b.iter(|| {
            let results = scanner.scan_directory(black_box(temp_dir.path()));
            black_box(results);
        })
    });

    group.finish();
}

/// Benchmark regex-only matching (no I/O)
fn bench_scan_regex_only(c: &mut Criterion) {
    let content_with_secrets = create_test_content(100, true);
    let content_clean = create_test_content(100, false);
    let scanner = Scanner::new();
    let cache = Arc::clone(scanner.pattern_cache());

    let mut group = c.benchmark_group("scan_regex_only");

    group.bench_function("with_secrets", |b| {
        b.iter(|| {
            for content in &content_with_secrets {
                let results = scanner.scan_content(black_box(content), "test.txt");
                black_box(results);
            }
        })
    });

    group.bench_function("clean_content", |b| {
        b.iter(|| {
            for content in &content_clean {
                let results = scanner.scan_content(black_box(content), "test.txt");
                black_box(results);
            }
        })
    });

    group.bench_function("pattern_cache_lookup", |b| {
        b.iter(|| {
            for content in &content_with_secrets {
                let matches = cache.matches_any(black_box(content));
                black_box(matches);
            }
        })
    });

    group.finish();
}

/// Benchmark pattern cache initialization
fn bench_pattern_cache_creation(c: &mut Criterion) {
    // Create patterns using scanner's default config
    let config = ScannerConfig::default();
    let patterns = &config.patterns;

    let mut group = c.benchmark_group("pattern_cache_creation");
    group.throughput(Throughput::Elements(patterns.len() as u64));

    group.bench_function("compile_all_patterns", |b| {
        b.iter(|| {
            let cache = PatternCache::new(black_box(patterns));
            black_box(cache);
        })
    });

    group.finish();
}

/// Benchmark parallel vs serial scanning
fn bench_parallel_vs_serial(c: &mut Criterion) {
    let temp_dir = create_test_files(500, 0.05);
    let scanner = Scanner::new();

    // Create a serial scanner (1 thread)
    let serial_config = ScannerConfig::default().with_threads(1);
    let serial_scanner = Scanner::with_config(serial_config);

    let mut group = c.benchmark_group("parallel_vs_serial");
    group.throughput(Throughput::Elements(500));

    group.bench_function("parallel_auto", |b| {
        b.iter(|| {
            let results = scanner.scan_directory(black_box(temp_dir.path()));
            black_box(results);
        })
    });

    group.bench_function("serial_1_thread", |b| {
        b.iter(|| {
            let results = serial_scanner.scan_directory(black_box(temp_dir.path()));
            black_box(results);
        })
    });

    group.finish();
}

/// Benchmark with different thread counts
fn bench_thread_count_scaling(c: &mut Criterion) {
    let temp_dir = create_test_files(500, 0.05);
    let thread_counts = vec![1, 2, 4, 8];

    let mut group = c.benchmark_group("thread_count_scaling");
    group.throughput(Throughput::Elements(500));

    for num_threads in thread_counts {
        let config = ScannerConfig::default().with_threads(num_threads);
        let scanner = Scanner::with_config(config);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{} threads", num_threads)),
            &num_threads,
            |b, _| {
                b.iter(|| {
                    let results = scanner.scan_directory(black_box(temp_dir.path()));
                    black_box(results);
                })
            },
        );
    }

    group.finish();
}

/// Benchmark file collection (I/O bound)
fn bench_file_collection(c: &mut Criterion) {
    let temp_dir = create_test_files(500, 0.05);
    let scanner = Scanner::new();

    let mut group = c.benchmark_group("file_collection");
    group.throughput(Throughput::Elements(500));

    group.bench_function("collect_files", |b| {
        b.iter(|| {
            // Use reflection or internal access to test file collection
            // For now, we'll just measure full scan as proxy
            let _files = scanner.scan_directory(black_box(temp_dir.path()));
        })
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let config = ScannerConfig::default();
    let cache = Arc::new(PatternCache::new(&config.patterns));

    let mut group = c.benchmark_group("memory_patterns");

    group.bench_function("clone_cache", |b| {
        b.iter(|| {
            let cloned = black_box(cache.clone());
            black_box(cloned);
        })
    });

    group.finish();
}

/// Benchmark large file scanning
fn bench_large_file(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large.txt");

    // Create a 1MB file with some secrets
    let mut content = String::with_capacity(1024 * 1024);
    for i in 0..10000 {
        if i % 1000 == 0 {
            content.push_str(&format!("AWS_KEY=AKIAIOSFODNN7EXAMPLE{}\n", i));
        } else {
            content.push_str(&format!("Normal content line {}\n", i));
        }
    }
    fs::write(&large_file, &content).unwrap();

    let scanner = Scanner::new();

    let mut group = c.benchmark_group("large_file");
    group.throughput(Throughput::Bytes(content.len() as u64));

    group.bench_function("scan_1mb_file", |b| {
        b.iter(|| {
            let results = scanner.scan_file(black_box(&large_file));
            black_box(results);
        })
    });

    group.finish();
}

/// Benchmark with various file types
fn bench_mixed_file_types(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    // Create various file types
    let extensions = ["rs", "py", "js", "ts", "json", "yaml", "toml", "md", "txt", "sh"];
    for (i, ext) in extensions.iter().enumerate() {
        for j in 0..10 {
            let file = temp_dir.path().join(format!("file_{}_{}.{}", i * 10 + j, i, ext));
            let content = if j % 5 == 0 {
                format!("AWS_KEY=AKIAIOSFODNN7EXAMPLE{}\n", j)
            } else {
                format!("Content for file {}.{}\n", i, j)
            };
            fs::write(&file, content).unwrap();
        }
    }

    let scanner = Scanner::new();

    let mut group = c.benchmark_group("mixed_file_types");
    group.throughput(Throughput::Elements(extensions.len() as u64 * 10));

    group.bench_function("scan_mixed_types", |b| {
        b.iter(|| {
            let results = scanner.scan_directory(black_box(temp_dir.path()));
            black_box(results);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_scan_100_files,
    bench_scan_1000_files,
    bench_scan_regex_only,
    bench_pattern_cache_creation,
    bench_parallel_vs_serial,
    bench_thread_count_scaling,
    bench_file_collection,
    bench_memory_patterns,
    bench_large_file,
    bench_mixed_file_types,
);

criterion_main!(benches);
