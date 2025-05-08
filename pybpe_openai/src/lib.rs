use pyo3::prelude::*;
use std::sync::OnceLock;

static CL100K: OnceLock<bpe_openai::Bpe> = OnceLock::new();
static O200K: OnceLock<bpe_openai::Bpe> = OnceLock::new();

#[pyclass]
struct BPETokenizer {
    tokenizer: &'static bpe_openai::Bpe,
}

#[pymethods]
impl BPETokenizer {
    fn count(&self, text: &str) -> usize {
        self.tokenizer.count(text)
    }
    fn encode(&self, text: &str) -> Vec<usize> {
        self.tokenizer.encode(text)
    }
}

/// Get the CL100K tokenizer.
#[pyfunction]
fn cl100k() -> BPETokenizer {
    BPETokenizer {
        tokenizer: CL100K.get_or_init(|| bpe_openai::cl100k()),
    }
}

/// Get the O200K tokenizer.
#[pyfunction]
fn o200k() -> BPETokenizer {
    BPETokenizer {
        tokenizer: O200K.get_or_init(|| bpe_openai::o200k()),
    }
}

#[pymodule]
fn pybpe_openai(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BPETokenizer>()?;
    m.add_function(wrap_pyfunction!(cl100k, m)?)?;
    m.add_function(wrap_pyfunction!(o200k, m)?)?;
    Ok(())
}