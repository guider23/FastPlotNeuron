
<img width="1536" height="1024" alt="FastPlotNeuron Logo Design" src="https://github.com/user-attachments/assets/8ab9206d-38ad-4bf8-aeb0-5de9dba4a962" />

# PlotNeuron — Neural Network Diagram Generator 🦀

  

High-performance neural network visualization with **Rust acceleration**! This tool takes your neural network JSON, processes it with blazing-fast Rust code, runs it through a Node.js backend, cooks it with a Python-LaTeX script, and spits out a clean PNG diagram.

  

## New Features with Rust Integration

  

-  **5-10x faster JSON parsing** using Rust's serde

-  **Optimized layer processing** with efficient algorithms

-  **Performance benchmarking** tools

-  **Enhanced validation** with detailed error reporting

-  **CLI tools** for batch processing and testing

  

---

  

## 1. What This Is

  

- You send a JSON with your model layers

-  **Rust acceleration** handles parsing and optimization

- Backend runs a Python script using PlotNeuralNet (built on LaTeX/TikZ)

- It gives you back a crispy PNG diagram with performance metrics

  

---

  

## 2. You'll Need This Gear

  

- Windows 10/11 (Mac/Linux work too, but commands vary)

-  **Rust 1.70+** (install from [rustup.rs](https://rustup.rs/))

- Python 3.8+ (works well with 3.9+)

- Node.js 14+

- LaTeX (MiKTeX or TeX Live — must include `pdflatex`)

- ImageMagick (with `magick` command and PDF support)

- Git (optional, if you're cloning)

  

---

  

## 3. Quick Setup with Rust

  

### A. Grab the Project

  

```sh

git  clone  https://github.com/guider23/PlotNeuron

cd  PlotNeuron

```

  

### B. One-Command Setup

```sh

python  setup_rust.py

```

  

This will:

- Check all prerequisites

- Build Rust extensions

- Install dependencies

- Test integration

- Create example files

  

### C. Manual Setup (Alternative)

  

```sh

# Install Rust dependencies

pip  install  maturin

maturin  build  --release

pip  install  target/wheels/plotneuron_rust-0.1.0-*.whl

  

# Build CLI tools

cargo  build  --release  --bin  neuron-cli

  

# Install Node.js dependencies

cd  Server

npm  install

cd  ..

```

  

---

  

## 4. Usage Examples

  

### A. Production Server

```sh

cd  Server

npm  start

```

Access at `http://localhost:3000`

  

### B. CLI Tools

```sh

# Validate a neural network JSON

cargo  run  --bin  neuron-cli  validate  examples/cnn_example.json

  

# Benchmark performance

cargo  run  --bin  neuron-cli  benchmark  examples/complex_network.json

  

# Process layers and show parameters

cargo  run  --bin  neuron-cli  process  examples/cnn_example.json

```

  

### C. Direct Script Usage

```sh

# Generate with performance benchmarking

python  Server/scripts/generate.py  examples/cnn_example.json  --benchmark

  

# Standard generation

python  Server/scripts/generate.py  examples/complex_network.json

```

  

### D. API Endpoints

  

**Generate Diagram:**

```bash

curl  -X  POST  http://localhost:3000/generate  \

-H "Content-Type: application/json" \

-d  @examples/cnn_example.json

```

  

**Benchmark Performance:**

```bash

curl  -X  POST  http://localhost:3000/generate/benchmark  \

-H "Content-Type: application/json" \

-d  @examples/complex_network.json

```

  

**Validate JSON:**

```bash

curl  -X  POST  http://localhost:3000/generate/validate  \

-H "Content-Type: application/json" \

-d  @examples/cnn_example.json

```

  

---

  

## 5. Performance Improvements

  

With Rust integration, you get:

  

-  **JSON Parsing**: 5-10x faster than pure Python

-  **Layer Processing**: Optimized algorithms for complex architectures

-  **Mathematical Operations**: Vectorized computations

-  **Memory Efficiency**: Lower memory usage for large networks

-  **Error Handling**: Detailed validation with helpful error messages

  

Example benchmark output:

```

Performance Comparison (1000 iterations):

🦀 Rust: 0.0234s

Python: 0.1891s

⚡ Speedup: 8.1x faster with Rust

```

  

---

  

## 6. JSON Format

  

```json

{

"name": "My Neural Network",

"description": "Optional description",

"layers": [

{

"type": "conv",

"name": "conv1",

"filters": 32,

"kernel_size": 3,

"stride": 1

},

{

"type": "pool",

"name": "pool1",

"pool_size": 2

},

{

"type": "dense",

"name": "fc1",

"units": 128,

"activation": "relu"

}

]

}

```

  

**Supported Layer Types:**

-  `conv` - Convolutional layers

-  `pool` - Pooling layers (max/avg)

-  `dense` - Fully connected layers

-  `batchnorm` - Batch normalization

-  `dropout` - Dropout layers

  

---

  

## 7. Development

  

### Build from Source

```sh

# Build Rust components

cargo  build  --release

  

# Develop Python extension

maturin  develop  --release

  

# Run tests

cargo  test

python  -m  pytest  tests/

```

  

### Project Structure

```

PlotNeuron/

├── Cargo.toml # Rust dependencies

├── src/ # Rust source code

│ ├── lib.rs # Python bindings

│ ├── layer_processor.rs # Layer optimization

│ ├── neuron_math.rs # Mathematical operations

│ └── bin/neuron_cli.rs # CLI tools

├── Server/ # Node.js backend

├── examples/ # Example JSON files

└── setup_rust.py # Setup script

```

  

---

  

## 8. Troubleshooting

  

**Rust module not found:**

```sh

pip  install  maturin

maturin  develop  --release

```

  

**Performance not improved:**

- Ensure Rust module is properly installed

- Check for import errors in Python script

- Verify `RUST_AVAILABLE = True` in logs

  

**CLI tools not working:**

```sh

cargo  build  --release  --bin  neuron-cli

./target/release/neuron-cli  --help

```

  

---

  

## 9. Contributing

  

We welcome contributions! Particularly interested in:

- Additional Rust optimizations

- New neural network layer types

- Visualization improvements

- Documentation and examples

  

---

  

## 10. License

  

ISC License - see the original project for details.

  


**Rust acceleration additions** by the community. Built with performance in mind.

