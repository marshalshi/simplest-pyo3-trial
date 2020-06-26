use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pymodule]
fn rust2py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(minus))?;
    m.add_wrapped(wrap_pyfunction!(loop_rust))?;
    Ok(())
}

#[pyfunction]
fn minus(a: i64, b: i64) -> i64 {
    a - b
}

/// loop_rust(n)
/// --
///
/// O(N^2) complex sample code
#[pyfunction]
fn loop_rust(n: i64) {
    for i in 0..n {
        for j in 0..n {
            let _ = j - i;
        }
    }
}
