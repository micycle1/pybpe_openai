# pybpe_openai

**Fast OpenAI-Compatible Byte Pair Encoding (BPE) Tokenizer for Python, powered by Rust**

This package provides extremely fast, native Python bindings for Github's [OpenAI's BPE tokenizers](https://github.com/github/rust-gems/tree/main/crates/bpe-openai), including support for CL100K and O200K token sets.

Counts tokens about 5x faster in my tests vs `tiktoken`!

Supported tokenizers:
* cl100k
* o200k

---

## Installation

See [PyPI](https://test.pypi.org/project/pybpe-openai/0.3.0/) (note test for now).

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
