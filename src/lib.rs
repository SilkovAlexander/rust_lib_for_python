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
    m.add_function(wrap_pyfunction!(init, m)?)?;
    Ok(())
}

async fn rust_sleep() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    // async_std::task::sleep(Duration::from_secs(1)).await;
}

#[pyfunction]
fn call_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(
        py,
        async move {
            rust_sleep().await;
            Ok(())
        }
    )
}

use tokio::runtime::Builder;

#[pyfunction]
fn init() -> PyResult<()> {
    let mut runtime = Builder::new_multi_thread();
    runtime.enable_time();
    pyo3_asyncio::tokio::init(runtime);
    Ok(())
}