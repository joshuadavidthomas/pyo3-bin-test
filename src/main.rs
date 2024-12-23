use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        println!("Python version: {}", version);
        Ok(())
    })
}
