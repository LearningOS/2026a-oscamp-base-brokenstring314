//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    // TODO: Spawn n_threads threads
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: Join all threads, return final value
    let cnt = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for i in 1..=n_threads {
        let cnt = Arc::clone(&cnt);
        handles.push(thread::spawn(move || {
            *cnt.lock().unwrap() += count_per_thread;
        }));
    }
    for hd in handles {
        hd.join().unwrap();
    }
    let res = *cnt.lock().unwrap();
    // 直接写 *cnt.lock().unwrap() 的话
    // Rust 2021 的 drop 顺序：
    // 计算尾表达式：lock() 产生临时 MutexGuard，它借用着 cnt
    // 拷贝出 i32 作为返回值
    // drop 函数的局部变量：cnt（主线程那个 Arc）、handles……
    // 最后才 drop 尾表达式里的临时值（guard）
    // 第 3 步 cnt 没了，第 4 步 guard 还在借用它，借用比主人活得久，所以报 E0597: cnt does not live long enough。
    res
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return
    let cnt = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 1..=n_threads {
        let cnt = Arc::clone(&cnt);
        handles.push(thread::spawn(move || {
            *cnt.lock().unwrap().push_mut(i-1);
        }));
    }
    for hd in handles {
        hd.join().unwrap();
    }
    let mut res = cnt.lock().unwrap().clone();
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
