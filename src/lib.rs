use std::time::Duration;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn test_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(call_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(return_value, m)?)?;
    Ok(())
}

async fn rust_sleep(sec: u64) {
    tokio::time::sleep(Duration::from_secs(sec)).await;
}

#[pyfunction]
fn call_sleep(py: Python, sec: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(
        py,
        async move {
            rust_sleep(sec).await;
            Ok(())
        }
    )
}

#[pyfunction]
fn return_value(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(
        py,
        async move {
            rust_sleep(1).await;
            Ok(1)
        }
    )
}

