
"""
Setup script for PlotNeuron with Rust integration
"""

import os
import sys
import subprocess
import platform

def run_command(cmd, description):
    """Run a command and handle errors"""
    print(f" {description}...")
    try:
        if platform.system() == "Windows":
            result = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
        else:
            result = subprocess.run(cmd.split(), check=True, capture_output=True, text=True)
        print(f" {description} completed successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"{description} failed:")
        print(f"   Error: {e}")
        if e.stdout:
            print(f"   Stdout: {e.stdout}")
        if e.stderr:
            print(f"   Stderr: {e.stderr}")
        return False

def check_rust_installation():
    """Check if Rust is installed"""
    try:
        result = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
        if result.returncode == 0:
            print(f" Rust found: {result.stdout.strip()}")
            return True
    except FileNotFoundError:
        pass
    
    print("❌ Rust not found. Please install Rust from: https://rustup.rs/")
    return False

def check_python_version():
    """Check Python version"""
    version = sys.version_info
    if version.major >= 3 and version.minor >= 8:
        print(f"Python {version.major}.{version.minor}.{version.micro} is compatible")
        return True
    else:
        print(f" Python {version.major}.{version.minor}.{version.micro} is not supported. Need Python 3.8+")
        return False

def install_rust_dependencies():
    """Install Rust dependencies and build the Python extension"""
    

    if not run_command("pip install maturin", "Installing maturin"):
        return False
    

    if not run_command("maturin build --release", "Building Rust extension"):
        print(" Falling back to debug build...")
        if not run_command("maturin build", "Building Rust extension (debug)"):
            return False
    

    import glob
    wheel_files = glob.glob("target/wheels/plotneuron_rust-*.whl")
    if wheel_files:
        wheel_file = wheel_files[0]
        if not run_command(f"pip install {wheel_file}", "Installing Rust extension"):
            return False
    else:
        print("❌ No wheel file found to install")
        return False
    
    return True

def build_rust_cli():
    """Build the Rust CLI tool"""
    return run_command("cargo build --release --bin neuron-cli", "Building Rust CLI tool")

def test_rust_integration():
    """Test if the Rust module can be imported"""
    try:
        import plotneuron_rust
        print("Rust module imported successfully")
        

        test_json = '{"layers": [{"type": "conv", "filters": 32}]}'
        result = plotneuron_rust.parse_network_json(test_json)
        print("Rust functions working correctly")
        return True
    except ImportError as e:
        print(f"Failed to import Rust module: {e}")
        return False
    except Exception as e:
        print(f"Rust module test failed: {e}")
        return False

def setup_npm_dependencies():
    """Install Node.js dependencies"""
    os.chdir("Server")
    success = run_command("npm install", "Installing Node.js dependencies")
    os.chdir("..")
    return success

def create_example_json():
    """Create an example neural network JSON for testing"""
    example = {
        "name": "Example CNN",
        "description": "A simple convolutional neural network",
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
                "type": "conv",
                "name": "conv2",
                "filters": 64,
                "kernel_size": 3,
                "stride": 1
            },
            {
                "type": "dense",
                "name": "fc1",
                "units": 128,
                "activation": "relu"
            },
            {
                "type": "dense",
                "name": "output",
                "units": 10,
                "activation": "softmax"
            }
        ]
    }
    
    os.makedirs("examples", exist_ok=True)
    with open("examples/cnn_example.json", "w") as f:
        import json
        json.dump(example, f, indent=2)
    
    print("Created example JSON at examples/cnn_example.json")

def main():
    print("⭐ Setting up PlotNeuron with Rust integration")
    print("=" * 50)
    

    if not check_python_version():
        sys.exit(1)
    
    if not check_rust_installation():
        sys.exit(1)
    

    steps = [
        ("Installing Rust dependencies and building extension", install_rust_dependencies),
        ("Building Rust CLI tool", build_rust_cli),
        ("Testing Rust integration", test_rust_integration),
        ("Installing Node.js dependencies", setup_npm_dependencies),
        ("Creating example files", create_example_json),
    ]
    
    failed_steps = []
    for description, step_func in steps:
        print(f"\n📦 {description}")
        if not step_func():
            failed_steps.append(description)
    

    print("\n" + "=" * 50)
    print("🎯 Setup Summary")
    
    if failed_steps:
        print(f" {len(failed_steps)} step(s) failed:")
        for step in failed_steps:
            print(f"   - {step}")
        print("\nPlotNeuron may work with reduced functionality")
    else:
        print(" All setup steps completed successfully!")
        print("\n PlotNeuron with Rust acceleration is ready!")
        
    print("\n📖 Next steps:")
    print("   1. Test with: cargo run --bin neuron-cli validate examples/cnn_example.json")
    print("   2. Start server: cd Server && npm start")
    print("   3. Check performance: python Server/scripts/generate.py examples/cnn_example.json --benchmark")

if __name__ == "__main__":
    main()

