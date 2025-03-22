# Obadh Engine (অবাধ ইঞ্জিন)

Obadh Engine is a Roman-to-Bengali transliteration engine built with Rust, focusing on linguistic accuracy and performance.

## Core Capabilities

- Built in Rust for native performance across platforms
- Implements a phonological approach for accurate Bengali transliteration
- Uses algorithmic syllable formation instead of extensive lookup tables
- Compiles to WebAssembly for browser-based usage

## Supported Bengali Writing Features

- Consonants and vowels with proper rendering
- Conjuncts (যুক্তাক্ষর) with correct orthography
- Reph (র্) and other special forms
- Ya-phala (্য), Ra-phala (্র), and other phonological modifications
- Bengali numerals and punctuation

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.56 or newer
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly compilation

### Building and Running

1. Clone the repository:

```bash
git clone https://github.com/yourusername/obadh_engine.git
cd obadh_engine
```

2. Build the library for native use:

```bash
cargo build --release
```

3. Run tests:

```bash
cargo test
```

### Building for Web

To build the WebAssembly version:

```bash
# Use the build script
./build.sh

# Or run the commands manually
wasm-pack build --target web --out-dir www/pkg
```

### Running the Web Demo

Using the build script:

```bash
./build.sh serve
```

Or manually:

```bash
cd www
python -m http.server
```

Then open `http://localhost:8000` in your browser.

## Implementation Details

Obadh Engine processes text in several stages:

1. **Tokenization**: Input is broken into linguistic units
2. **Phoneme Analysis**: Tokens are converted to Bengali phonological units
3. **Syllable Formation**: Phonemes are organized following Bengali orthographic rules
4. **Rendering**: Final output is generated with proper conjuncts and modifiers

This phonological approach provides:
- More accurate handling of complex Bengali script features
- Better understanding of language-specific rules
- Efficient memory usage compared to extensive lookup tables

## Code Organization

```
obadh_engine/
├── src/               # Source code
│   ├── engine/        # Core processing engine
│   ├── linguistic/    # Phonological models
│   └── wasm/          # WebAssembly bindings
├── tests/             # Test suites
└── www/               # Web interface
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.