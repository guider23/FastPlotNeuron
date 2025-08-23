import os
import sys
import json
import shutil
import subprocess
import time


try:
    import plotneuron_rust
    RUST_AVAILABLE = True
    print("🦀 Rust acceleration enabled!")
except ImportError:
    RUST_AVAILABLE = False
    print("⚠️  Rust module not available, using Python fallback")


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PYCORE_DIR = os.path.abspath(os.path.join(SCRIPT_DIR, '..', 'plotter', 'PlotNeuralNet', 'pycore'))
sys.path.insert(0, PYCORE_DIR)
from tikzeng import *

def parse_json_fast(json_path):
    """Fast JSON parsing using Rust if available, fallback to Python"""
    with open(json_path, "r") as f:
        content = f.read()
    
    if RUST_AVAILABLE:
        try:

            validated_json = plotneuron_rust.parse_network_json(content)
            return json.loads(validated_json)
        except Exception as e:
            print(f"⚠️  Rust parsing failed: {e}, falling back to Python")
    

    return json.loads(content)

def process_layers_optimized(data):
    """Process layers with Rust optimization if available"""
    if RUST_AVAILABLE:
        try:

            json_str = json.dumps(data)
            processed = plotneuron_rust.process_layers(json_str)
            return processed
        except Exception as e:
            print(f"⚠️  Rust layer processing failed: {e}, using Python fallback")
    

    layers = []
    for i, layer in enumerate(data.get("layers", [])):
        processed = {
            "name": layer.get("name", f"layer{i}"),
            "type": layer["type"],
            "width": "2.0",
            "height": "15.0",
            "depth": "15.0",
            "position": f"({i * 5.0},0,0)"
        }
        layers.append(processed)
    return layers

def calculate_positions_fast(layer_count, spacing=5.0):
    """Fast position calculation using Rust"""
    if RUST_AVAILABLE:
        try:
            return plotneuron_rust.calculate_layer_positions(layer_count, spacing)
        except Exception as e:
            print(f"⚠️  Rust position calculation failed: {e}")
    

    return [f"({i * spacing},0,0)" for i in range(layer_count)]

def benchmark_performance(json_path):
    """Benchmark Rust vs Python performance"""
    if not RUST_AVAILABLE:
        print("📊 Rust not available for benchmarking")
        return
    
    with open(json_path, "r") as f:
        content = f.read()
    
    iterations = 1000
    

    start_time = time.time()
    rust_time = plotneuron_rust.benchmark_json_parsing(content, iterations)
    total_rust = time.time() - start_time
    

    start_time = time.time()
    for _ in range(iterations):
        json.loads(content)
    python_time = time.time() - start_time
    
    print(f"Performance Comparison ({iterations} iterations):")
    print(f"   🦀 Rust: {rust_time:.4f}s")
    print(f"   🐍 Python: {python_time:.4f}s")
    print(f"   ⚡ Speedup: {python_time/rust_time:.1f}x faster with Rust")

def generate_arch_from_json(data):
    """Generate architecture with optimized processing"""
    print("🔄 Processing neural network architecture...")
    

    processed_layers = process_layers_optimized(data)
    
    arch = [
        to_head('..'),
        to_cor(),
        to_begin()
    ]
    
    prev_name = None
    for i, layer_info in enumerate(processed_layers):
        layer = data["layers"][i]
        ltype = layer["type"].lower()
        name = layer_info["name"]
        
        if ltype == "conv":
            arch.append(to_Conv(
                name=name,
                s_filer=layer.get("filters", 64),
                n_filer=layer.get("filters", 64),
                to=layer_info["position"],
                width=float(layer_info["width"]),
                height=float(layer_info["height"]),
                depth=float(layer_info["depth"])
            ))
        elif ltype == "pool":
            arch.append(to_Pool(
                name=name,
                to=layer_info["position"],
                width=float(layer_info["width"]),
                height=float(layer_info["height"]),
                depth=float(layer_info["depth"])
            ))
        elif ltype == "dense":
            arch.append(to_SoftMax(
                name=name,
                s_filer=layer.get("units", 10),
                to=layer_info["position"],
                width=float(layer_info["width"]),
                height=float(layer_info["height"]),
                depth=float(layer_info["depth"])
            ))
        
        if prev_name:
            arch.append(to_connection(prev_name, name))
        prev_name = name
    
    arch.append(to_end())
    return arch

def main():
    start_total = time.time()
    

    script_dir = os.path.dirname(os.path.abspath(__file__))
    server_dir = os.path.dirname(script_dir)
    project_root = os.path.dirname(server_dir)
    

    base_dir = os.path.join(server_dir, "plotter", "PlotNeuralNet", "pyexamples")
    output_dir = os.path.join(server_dir, "plotter", "output")
    os.makedirs(output_dir, exist_ok=True)
    
    if not os.path.exists(base_dir):
        print(f"❌ PlotNeuralNet directory not found at: {base_dir}")
        print("Please run: git clone https://github.com/HarisIqbal88/PlotNeuralNet.git")
        sys.exit(1)
    
    os.chdir(base_dir)


    if len(sys.argv) > 1:
        json_path = sys.argv[1]
        

        if not os.path.isabs(json_path):
            json_path = os.path.join(project_root, json_path)
        

        if "--benchmark" in sys.argv:
            benchmark_performance(json_path)
        
        print(f"📖 Reading architecture from: {json_path}")
        data = parse_json_fast(json_path)
        
        print(f"Generating architecture with {len(data.get('layers', []))} layers...")
        arch = generate_arch_from_json(data)
        tex_name = "main.tex"
        to_generate(arch, tex_name)
    else:
        print("💀 No JSON input provided.")
        sys.exit(1)

    print("Compiling LaTeX...")
    compile_start = time.time()
    subprocess.run(["pdflatex", "main.tex"], check=True, capture_output=True)
    compile_time = time.time() - compile_start

    print("Converting to PNG...")
    convert_start = time.time()
    subprocess.run([
        "magick", "-density", "300", "main.pdf",
        "-quality", "100", "main.png"
    ], check=True, capture_output=True)
    convert_time = time.time() - convert_start


    output_path = os.path.join(output_dir, "main.png")
    shutil.copy("main.png", output_path)
    
    total_time = time.time() - start_total
    
    print(f"⚡🪙 Generation complete!")
    print(f"👾 Performance Summary:")
    print(f"   📝 LaTeX compilation: {compile_time:.2f}s")
    print(f"   🖼️  PNG conversion: {convert_time:.2f}s")
    print(f"   ⏱️  Total time: {total_time:.2f}s")
    print(f"   📁 Output: {output_path}")

if __name__ == "__main__":
    main()

