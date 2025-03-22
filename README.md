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
./build.sh               # Build only
./build.sh clean         # Clean up build artifacts
```

OR build manually:

```bash
wasm-pack build --target web --out-dir www/pkg
```

### Running the Web Demo

Using the build script:

```bash
./build.sh serve           # Uses default port 8000
./build.sh serve 8080      # Uses custom port 8080
```

Or manually:

```bash
cd www
python3 -m http.server 8000   # or 'python -m http.server 8000' on some systems
```

Then open `http://localhost:PORT` in your browser.

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

## Usage Examples

### Command Line Interface

The CLI offers several ways to input and output text:

```bash
# Basic usage - directly on command line
obadh "amar sonar bangla"

# Read from a file
obadh -i input.txt

# Write to a file
obadh "amar sonar bangla" -o output.txt

# Read from stdin and write to stdout
echo "amar sonar bangla" | obadh

# Use different output formats
obadh "amar sonar bangla" -f json
obadh "amar sonar bangla" -f xml
obadh "amar sonar bangla" -f html
obadh "amar sonar bangla" -f markdown

# Use different verbosity levels
obadh "amar sonar bangla" -v quiet
obadh "amar sonar bangla" -v normal
obadh "amar sonar bangla" -v detailed
obadh "amar sonar bangla" -v debug

# Enable debug mode with token information and performance metrics (outputs JSON)
obadh "amar sonar bangla" -d

# Process a batch of text from a file
obadh -i input.txt -o output.txt
```

## Debug Mode and Performance Metrics

Obadh Engine provides built-in diagnostics and performance analysis capabilities for developers and technical users.

### Debug Flag (-d)

When the debug flag is enabled, the engine outputs detailed JSON with full processing information:

```json
{
  "input": "amar sonar bangla",
  "output": "আমার সোনার বাংলা",
  "tokens": [
    {"type": "Consonant", "value": "a", "position": 0},
    {"type": "Consonant", "value": "m", "position": 1},
    // ... more tokens
  ],
  "phonemes": [
    {"type": "Vowel", "value": "আ"},
    {"type": "Consonant", "value": "ম"},
    // ... more phonemes
  ],
  "syllables": [
    ["আ", "মা", "র"],
    ["সো", "না", "র"],
    ["বাং", "লা"]
  ],
  "performance": {
    "total_ms": 0.215,
    "analysis_ms": 0.142,
    "token_count": 17,
    "estimated_chars_per_second": 79069
  }
}
```

This output provides:
- Original input and transliterated output
- Complete token breakdown with position information
- Phoneme analysis showing the phonological units
- Syllable organization showing Bengali orthographic grouping
- Performance metrics with timing details

### Performance Metrics API

For programmatic access to performance data, the library provides these methods:

```rust
// Get detailed performance metrics for a single transliteration
let metrics_json = engine.transliterate_with_performance("amar sonar bangla");

// Process a batch with performance tracking
let batch_metrics = engine.batch_transliterate_with_performance(&["amar", "sonar", "bangla"]);

// Use efficient batch processing for larger datasets
let results = engine.batch_transliterate_efficient(&large_input_array);
```

Batch processing metrics include:
- Total processing time in milliseconds
- Average time per text item
- Individual processing time for each item
- Estimated characters per second

These features are particularly useful for:
- Debugging transliteration issues
- Performance optimization and bottleneck identification
- Integration testing
- Analysis of complex Bengali linguistic patterns