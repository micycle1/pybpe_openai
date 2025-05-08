# pybpe_openai

**Fast OpenAI-Compatible Byte Pair Encoding (BPE) Tokenizer for Python, powered by Rust**

This package provides extremely fast, native Python bindings for Github's [OpenAI's BPE tokenizers](https://github.com/github/rust-gems/tree/main/crates/bpe-openai), including support for CL100K and O200K token sets.

Counts tokens about 5x faster in my tests vs `tiktoken`!

Supported tokenizers:
* cl100k
* o200k

---

## Installation

Install directly from GitHub (using pip's [subdirectory](https://pip.pypa.io/en/stable/topics/vcs-support/#installables-containing-their-own-projects) feature):

```bash
pip install "git+https://github.com/micycle1/pybpe_openai.git#subdirectory=pybpe_openai"
```

## Usage Example

```python
import pybpe_openai

# Get a tokenizer (O200K = GPT-4o standard)
bpe = pybpe_openai.o200k()

# Count tokens
print("Tokens:", bpe.count("Hello, world!"))

# Encode string to IDs
ids = bpe.encode("Hello, world!")
print("Token IDs:", ids)
```
