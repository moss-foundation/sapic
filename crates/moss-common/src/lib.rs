/// A macro for handling errors in loops by automatically continuing.
///
/// This macro simplifies error handling in loops by automatically calling `continue`
/// when encountering an `Err` in a `Result`.
///
/// # Variants
///
/// - `continue_if_err!(expr)` - Continue on error, ignore the error
/// - `continue_if_err!(expr, |e| { /* handle error */ })` - Continue on error with handler
/// - `continue_if_err!('label: expr)` - Continue to labeled loop on error
/// - `continue_if_err!('label: expr, |e| { /* handle error */ })` - Continue to labeled loop with handler
///
/// # Examples
///
/// ```rust
/// use moss_common::continue_if_err;
///
/// let numbers = vec!["1", "not_a_number", "3", "4"];
/// let mut parsed = Vec::new();
/// for s in numbers {
///     let num = continue_if_err!(s.parse::<i32>());
///     parsed.push(num);
/// }
/// assert_eq!(parsed, vec![1, 3, 4]);
/// ```
#[macro_export]
macro_rules! continue_if_err {
    // Convenience: closure returning async block → delegate to async block variants
    (|| async { $($body:tt)* }, $on_err:expr) => {
        $crate::continue_if_err!(async { $($body)* }, $on_err)
    };

    (|| async { $($body:tt)* }) => {
        $crate::continue_if_err!(async { $($body)* })
    };

    ($label:lifetime: || async { $($body:tt)* }, $on_err:expr) => {
        $crate::continue_if_err!($label: async { $($body)* }, $on_err)
    };

    ($label:lifetime: || async { $($body:tt)* }) => {
        $crate::continue_if_err!($label: async { $($body)* })
    };
    // Async block variants (unlabeled)
    (async { $($body:tt)* }, $on_err:expr) => {
        match (async { $($body)* }).await {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue;
            }
        }
    };

    (async { $($body:tt)* }) => {
        match (async { $($body)* }).await {
            Ok(val) => val,
            Err(_) => continue,
        }
    };

    // Synchronous block variants (unlabeled)
    ({ $($body:tt)* }, $on_err:expr) => {
        match (|| { $($body)* })() {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue;
            }
        }
    };

    ({ $($body:tt)* }) => {
        match (|| { $($body)* })() {
            Ok(val) => val,
            Err(_) => continue,
        }
    };

    // Async block variants (labeled)
    ($label:lifetime: async { $($body:tt)* }, $on_err:expr) => {
        match (async { $($body)* }).await {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue $label;
            }
        }
    };

    ($label:lifetime: async { $($body:tt)* }) => {
        match (async { $($body)* }).await {
            Ok(val) => val,
            Err(_) => continue $label,
        }
    };

    // Synchronous block variants (labeled)
    ($label:lifetime: { $($body:tt)* }, $on_err:expr) => {
        match (|| { $($body)* })() {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue $label;
            }
        }
    };

    ($label:lifetime: { $($body:tt)* }) => {
        match (|| { $($body)* })() {
            Ok(val) => val,
            Err(_) => continue $label,
        }
    };

    ($label:lifetime: $expr:expr, $on_err:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue $label;
            }
        }
    };

    ($label:lifetime: $expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => continue $label,
        }
    };

    ($expr:expr, $on_err:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue;
            }
        }
    };

    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => continue,
        }
    };
}

/// A macro for handling None values in loops by automatically continuing.
///
/// This macro simplifies None handling in loops by automatically calling `continue`
/// when encountering a `None` in an `Option`.
///
/// # Variants
///
/// - `continue_if_none!(expr)` - Continue on None
/// - `continue_if_none!(expr, || { /* handle none */ })` - Continue on None with handler
/// - `continue_if_none!('label: expr)` - Continue to labeled loop on None
/// - `continue_if_none!('label: expr, || { /* handle none */ })` - Continue to labeled loop with handler
///
/// # Examples
///
/// ```rust
/// use moss_common::continue_if_none;
///
/// let items = vec![Some(1), None, Some(3), None, Some(5)];
/// let mut valid = Vec::new();
/// for item in items {
///     let value = continue_if_none!(item);
///     valid.push(value);
/// }
/// assert_eq!(valid, vec![1, 3, 5]);
/// ```
#[macro_export]
macro_rules! continue_if_none {
    ($label:lifetime: $expr:expr, $on_none:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                $on_none();
                continue $label;
            }
        }
    };

    ($label:lifetime: $expr:expr) => {
        match $expr {
            Some(val) => val,
            None => continue $label,
        }
    };

    ($expr:expr, $on_none:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                $on_none();
                continue;
            }
        }
    };

    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_continue_if_err_with_handler() {
        let error_log = Arc::new(Mutex::new(Vec::new()));
        let error_log_clone = Arc::clone(&error_log);

        let mut successful_values = Vec::new();

        for i in 0..5 {
            let result: Result<i32, &str> = if i % 2 == 0 { Ok(i) } else { Err("error") };

            let value = continue_if_err!(result, |e| {
                error_log_clone
                    .lock()
                    .unwrap()
                    .push(format!("Error: {}", e));
            });

            successful_values.push(value);
        }

        assert_eq!(successful_values, vec![0, 2, 4]);
        assert_eq!(error_log.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_continue_if_err_without_handler() {
        let mut successful_values = Vec::new();

        for i in 0..5 {
            let result: Result<i32, &str> = if i % 2 == 0 { Ok(i) } else { Err("error") };

            let value = continue_if_err!(result);
            successful_values.push(value);
        }

        assert_eq!(successful_values, vec![0, 2, 4]);
    }

    #[test]
    fn test_continue_if_err_with_label() {
        let mut successful_values = Vec::new();
        let mut outer_iterations = 0;

        'outer: for i in 0..3 {
            outer_iterations += 1;

            for j in 0..3 {
                let result: Result<(i32, i32), &str> = if j == 1 {
                    Err("skip to next outer iteration")
                } else {
                    Ok((i, j))
                };

                let value = continue_if_err!('outer: result);
                successful_values.push(value);
            }
        }

        assert_eq!(outer_iterations, 3);
        assert_eq!(successful_values, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn test_continue_if_none_with_handler() {
        let none_log = Arc::new(Mutex::new(Vec::new()));
        let none_log_clone = Arc::clone(&none_log);

        let mut successful_values = Vec::new();

        for i in 0..5 {
            let option = if i % 2 == 0 { Some(i) } else { None };

            let value = continue_if_none!(option, || {
                none_log_clone
                    .lock()
                    .unwrap()
                    .push(format!("None at index {}", i));
            });

            successful_values.push(value);
        }

        assert_eq!(successful_values, vec![0, 2, 4]);
        assert_eq!(none_log.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_continue_if_none_without_handler() {
        let mut successful_values = Vec::new();

        for i in 0..5 {
            let option = if i % 2 == 0 { Some(i) } else { None };

            let value = continue_if_none!(option);
            successful_values.push(value);
        }

        assert_eq!(successful_values, vec![0, 2, 4]);
    }

    #[test]
    fn test_continue_if_none_with_label() {
        let mut successful_values = Vec::new();
        let mut outer_iterations = 0;

        'outer: for i in 0..3 {
            outer_iterations += 1;

            for j in 0..3 {
                let option = if j == 1 { None } else { Some((i, j)) };

                let value = continue_if_none!('outer: option);
                successful_values.push(value);
            }
        }

        assert_eq!(outer_iterations, 3);
        assert_eq!(successful_values, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn test_mixed_scenarios() {
        let mut results = Vec::new();

        for i in 0..10 {
            // Test string parsing
            let str_val = if i < 5 {
                i.to_string()
            } else {
                "not_a_number".to_string()
            };
            let parsed = continue_if_err!(str_val.parse::<i32>(), |_| {
                // Ignore parsing errors
            });

            // Test Option
            let maybe_doubled = if parsed % 2 == 0 {
                Some(parsed * 2)
            } else {
                None
            };

            let doubled = continue_if_none!(maybe_doubled, || {
                // Ignore None values
            });

            results.push(doubled);
        }

        assert_eq!(results, vec![0, 4, 8]); // 0*2=0, 2*2=4, 4*2=8
    }

    #[test]
    fn test_nested_loops() {
        let mut results = Vec::new();

        'outer: for i in 0..3 {
            'inner: for j in 0..3 {
                // Test with Result and inner label
                let result1: Result<i32, &str> = if j == 1 { Err("skip inner") } else { Ok(j) };

                let val1 = continue_if_err!('inner: result1);

                // Test with Option and outer label
                let option: Option<(i32, i32)> = if i == 1 && j == 2 {
                    None // Skip to next i
                } else {
                    Some((i, val1))
                };

                let val2 = continue_if_none!('outer: option);
                results.push(val2);
            }
        }

        assert_eq!(results, vec![(0, 0), (0, 2), (1, 0), (2, 0), (2, 2)]);
    }

    #[test]
    fn test_continue_if_err_sync_block_grouped() {
        let mut results = Vec::new();

        for i in 0..5 {
            let value = continue_if_err!(
                {
                    if i == 1 { Err("err1") } else { Ok(i) }?;
                    if i == 3 { Err("err2") } else { Ok(i * 10) }
                },
                |_| {
                    // ignore errors
                }
            );

            results.push(value);
        }

        assert_eq!(results, vec![0, 20, 40]);
    }

    #[tokio::test]
    async fn test_continue_if_err_async_block_grouped_with_handler() {
        let mut results = Vec::new();

        for i in 0..5 {
            let value = continue_if_err!(
                async {
                    let _a = if i == 1 {
                        return Err("err1");
                    } else {
                        i
                    };
                    let b = if i == 3 {
                        return Err("err2");
                    } else {
                        _a * 2
                    };
                    Ok::<_, &str>(b)
                },
                |_| {
                    // ignore errors
                }
            );

            results.push(value);
        }

        assert_eq!(results, vec![0, 4, 8]);
    }

    #[test]
    fn test_continue_if_err_sync_block_with_label() {
        let mut results = Vec::new();
        let mut outer_iterations = 0;

        'outer: for i in 0..3 {
            outer_iterations += 1;
            for j in 0..3 {
                let val =
                    continue_if_err!('outer: { if j == 1 { Err("skip") } else { Ok((i, j)) } });
                results.push(val);
            }
        }

        assert_eq!(outer_iterations, 3);
        assert_eq!(results, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[tokio::test]
    async fn test_continue_if_err_async_block_with_label() {
        let mut results = Vec::new();
        let mut outer_iterations = 0;

        'outer: for i in 0..3 {
            outer_iterations += 1;
            for j in 0..3 {
                let val = continue_if_err!('outer: async {
                    if j == 1 { return Err("skip"); }
                    Ok::<_, &str>((i, j))
                });
                results.push(val);
            }
        }

        assert_eq!(outer_iterations, 3);
        assert_eq!(results, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[tokio::test]
    async fn test_continue_if_err_closure_async_proxy() {
        let mut results = Vec::new();

        for i in 0..4 {
            let val = continue_if_err!(
                || async {
                    if i % 2 == 1 {
                        return Err("odd");
                    }
                    Ok::<_, &str>(i + 10)
                },
                |_| {}
            );
            results.push(val);
        }

        assert_eq!(results, vec![10, 12]);
    }

    #[test]
    fn test_continue_if_err_with_label_and_handler() {
        let error_log = Arc::new(Mutex::new(Vec::new()));
        let error_log_clone = Arc::clone(&error_log);
        let mut results = Vec::new();

        'outer: for i in 0..3 {
            for j in 0..3 {
                let result: Result<i32, String> = if i == 1 && j == 1 {
                    Err(format!("Error at ({}, {})", i, j))
                } else {
                    Ok(i * 10 + j)
                };

                let value = continue_if_err!('outer: result, |e| {
                    error_log_clone.lock().unwrap().push(e);
                });

                results.push(value);
            }
        }

        assert_eq!(results, vec![0, 1, 2, 10, 20, 21, 22]);
        assert_eq!(error_log.lock().unwrap().len(), 1);
        assert_eq!(error_log.lock().unwrap()[0], "Error at (1, 1)");
    }

    #[test]
    fn test_continue_if_none_with_label_and_handler() {
        let none_log = Arc::new(Mutex::new(Vec::new()));
        let none_log_clone = Arc::clone(&none_log);
        let mut results = Vec::new();

        'outer: for i in 0..3 {
            for j in 0..3 {
                let option: Option<i32> = if i == 1 && j == 1 {
                    None
                } else {
                    Some(i * 10 + j)
                };

                let value = continue_if_none!('outer: option, || {
                    none_log_clone.lock().unwrap().push(format!("None at ({}, {})", i, j));
                });

                results.push(value);
            }
        }

        assert_eq!(results, vec![0, 1, 2, 10, 20, 21, 22]);
        assert_eq!(none_log.lock().unwrap().len(), 1);
        assert_eq!(none_log.lock().unwrap()[0], "None at (1, 1)");
    }

    #[test]
    fn test_continue_if_err_edge_cases() {
        let mut results = Vec::new();

        // Test with different error types
        for i in 0..5 {
            let result: Result<i32, Box<dyn std::error::Error>> = if i == 2 {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "test error",
                )))
            } else {
                Ok(i)
            };

            let value = continue_if_err!(result, |_| {
                // Handle any error type
            });

            results.push(value);
        }

        assert_eq!(results, vec![0, 1, 3, 4]);
    }

    #[test]
    fn test_continue_if_none_edge_cases() {
        let mut results = Vec::new();

        // Test with complex Option types
        for i in 0..5 {
            let option: Option<(i32, String)> = if i == 2 {
                None
            } else {
                Some((i, format!("value_{}", i)))
            };

            let value = continue_if_none!(option, || {
                // Handle None case
            });

            results.push(value);
        }

        assert_eq!(
            results,
            vec![
                (0, "value_0".to_string()),
                (1, "value_1".to_string()),
                (3, "value_3".to_string()),
                (4, "value_4".to_string())
            ]
        );
    }
}
