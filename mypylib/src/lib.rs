use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "mypylib")]
pub fn mypylib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello))?;
    Ok(())
}

#[pyfunction]
pub fn say_hello(_py: Python) -> PyResult<String> {
    Ok("Hello, world!".to_string())
}
