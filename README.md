# Obadh Engine (অবাধ ইঞ্জিন)

A high-performance, linguistically accurate Roman-to-Bengali transliteration engine built with Rust.

## Features

- ⚡ **High Performance**: Built in Rust for speed and efficiency
- 🔍 **Linguistic Accuracy**: Based on phonological principles rather than exhaustive mappings
- 🧩 **Algorithm-Driven**: Handles Bengali writing complexities algorithmically
- 💻 **Cross-Platform**: Works on desktop, web, and can be integrated into other systems
- 🌐 **WASM Support**: Compiles to WebAssembly for web usage

## Bengali Writing Features Supported

- ✓ Consonants and vowels
- ✓ Conjuncts (juktakkhor)
- ✓ Reph (র্)
- ✓ Hasanta/Hosonto (্)
- ✓ Vowel diacritics (kar)
- ✓ Ya-phala (্য), Ra-phala (্র), etc.
- ✓ Special symbols (Chandrabindu, Visarga, etc.)
- ✓ Bengali numerals and punctuation

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.56+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for web functionality)

### Building the Project

1. Clone the repository:

```bash
git clone https://github.com/yourusername/obadh_engine.git
cd obadh_engine
```

2. Build the library:

```bash
cargo build --release
```

3. Run tests:

```bash
cargo test
```

4. Build WebAssembly module (for web usage):

```bash
wasm-pack build --target web --out-dir www/pkg
```

5. Run the web interface:

```bash
cd www
python -m http.server  # Or any local server
```

Then visit `http://localhost:8000` in your browser.

## Usage Examples

### As a Rust Library

```rust
use obadh_engine::ObadhEngine;

fn main() {
    // Create a new engine instance
    let engine = ObadhEngine::new();
    
    // Transliterate text
    let bengali = engine.transliterate("amar sonar bangla");
    println!("{}", bengali);  // Output: আমার সোনার বাংলা
}
```

### On the Web

```javascript
// Import the WebAssembly module
const wasm = await import('./pkg/obadh_engine.js');
await wasm.default();

// Create an instance of the WasmEngine
const engine = new wasm.WasmEngine();

// Transliterate text
const result = engine.transliterate("ami bangla likhte pari");
console.log(result);  // Output: আমি বাংলা লিখতে পারি
```

## How It Works

Unlike traditional transliteration engines that use extensive lookup tables, Obadh Engine uses a phonological approach:

1. **Tokenization**: Input text is broken into meaningful linguistic units
2. **Phoneme Analysis**: Tokens are converted to Bengali phonemes
3. **Syllable Formation**: Phonemes are organized into syllables following Bengali rules
4. **Orthographic Rendering**: Syllables are rendered with proper conjuncts, pholas, etc.

This approach is more memory-efficient and linguistically accurate than exhaustive mappings.

## Project Structure

```
obadh_engine/
├── src/               # Source code
│   ├── engine/        # Core engine components
│   ├── linguistic/    # Linguistic models
│   └── wasm/          # WebAssembly bindings
├── tests/             # Test suite
└── www/               # Web interface
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.