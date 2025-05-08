use pyo3::prelude::*;
use std::sync::OnceLock;

// The actual `Tokenizer` struct is exported at the top level of bpe-openai
static CL100K: OnceLock<&'static bpe_openai::Tokenizer> = OnceLock::new();
static O200K: OnceLock<&'static bpe_openai::Tokenizer> = OnceLock::new();

#[pyclass]
struct BPETokenizer {
    tokenizer: &'static bpe_openai::Tokenizer,
}

#[pymethods]
impl BPETokenizer {
    fn count(&self, text: &str) -> usize {
        self.tokenizer.count(text)
    }
    fn encode(&self, text: &str) -> Vec<u32> {
        self.tokenizer.encode(text)
    }
}

#[pyfunction]
fn cl100k() -> BPETokenizer {
    BPETokenizer {
        tokenizer: CL100K.get_or_init(|| bpe_openai::cl100k_base()),
    }
}

#[pyfunction]
fn o200k() -> BPETokenizer {
    BPETokenizer {
        tokenizer: O200K.get_or_init(|| bpe_openai::o200k_base()),
    }
}

#[pymodule]
fn pybpe_openai(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BPETokenizer>()?;
    m.add_function(wrap_pyfunction!(cl100k, m)?)?;
    m.add_function(wrap_pyfunction!(o200k, m)?)?;
    Ok(())
}
