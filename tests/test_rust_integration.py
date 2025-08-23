"""
Test Rust integration for PlotNeuron
"""
import json
import os
import sys
import time


sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def test_rust_import():
    """Test if Rust module can be imported"""
    print("🧪 Testing Rust module import...")
    try:
        import plotneuron_rust
        print("✅ Rust module imported successfully")
        return True
    except ImportError as e:
        print(f"❌ Rust module import failed: {e}")
        return False

def test_json_parsing():
    """Test JSON parsing functionality"""
    print("🧪 Testing JSON parsing...")
    
    try:
        import plotneuron_rust
        
        test_json = {
            "name": "Test Network",
            "layers": [
                {"type": "conv", "filters": 32, "kernel_size": 3},
                {"type": "pool", "pool_size": 2},
                {"type": "dense", "units": 10, "activation": "softmax"}
            ]
        }
        
        json_str = json.dumps(test_json)
        result = plotneuron_rust.parse_network_json(json_str)
        parsed_back = json.loads(result)
        
        assert parsed_back["name"] == "Test Network"
        assert len(parsed_back["layers"]) == 3
        print("✅ JSON parsing test passed")
        return True
        
    except Exception as e:
        print(f"❌ JSON parsing test failed: {e}")
        return False

def test_layer_processing():
    """Test layer processing functionality"""
    print("🧪 Testing layer processing...")
    
    try:
        import plotneuron_rust
        
        test_json = {
            "layers": [
                {"type": "conv", "filters": 64, "kernel_size": 3},
                {"type": "dense", "units": 128}
            ]
        }
        
        json_str = json.dumps(test_json)
        processed = plotneuron_rust.process_layers(json_str)
        
        assert len(processed) == 2
        assert processed[0]["type"] == "conv"
        assert processed[1]["type"] == "dense"
        print("✅ Layer processing test passed")
        return True
        
    except Exception as e:
        print(f"❌ Layer processing test failed: {e}")
        return False

def test_activations():
    """Test activation functions"""
    print("🧪 Testing activation functions...")
    
    try:
        import plotneuron_rust
        
        test_input = [1.0, 2.0, 3.0, -1.0, -2.0]
        

        relu_result = plotneuron_rust.compute_activations(test_input, "relu")
        expected_relu = [1.0, 2.0, 3.0, 0.0, 0.0]
        assert relu_result == expected_relu
        

        sigmoid_result = plotneuron_rust.compute_activations(test_input, "sigmoid")
        assert len(sigmoid_result) == len(test_input)
        assert all(0.0 < x < 1.0 for x in sigmoid_result)
        
        print("✅ Activation functions test passed")
        return True
        
    except Exception as e:
        print(f"❌ Activation functions test failed: {e}")
        return False

def test_performance_benchmark():
    """Test performance benchmarking"""
    print("🧪 Testing performance benchmark...")
    
    try:
        import plotneuron_rust
        
        test_json = json.dumps({
            "layers": [{"type": "conv", "filters": 32}] * 10
        })
        

        duration = plotneuron_rust.benchmark_json_parsing(test_json, 100)
        assert duration > 0.0
        print(f"✅ Benchmark test passed (duration: {duration:.4f}s)")
        return True
        
    except Exception as e:
        print(f"❌ Benchmark test failed: {e}")
        return False

def test_position_calculation():
    """Test position calculation"""
    print("🧪 Testing position calculation...")
    
    try:
        import plotneuron_rust
        
        positions = plotneuron_rust.calculate_layer_positions(5, 4.0)
        
        assert len(positions) == 5
        assert positions[0] == "(0.0,0,0)"
        assert positions[1] == "(4.0,0,0)"
        assert positions[4] == "(16.0,0,0)"
        
        print("✅ Position calculation test passed")
        return True
        
    except Exception as e:
        print(f"❌ Position calculation test failed: {e}")
        return False

def run_all_tests():
    """Run all tests"""
    print("🚀 Running PlotNeuron Rust Integration Tests")
    print("=" * 50)
    
    tests = [
        ("Rust Import", test_rust_import),
        ("JSON Parsing", test_json_parsing),
        ("Layer Processing", test_layer_processing),
        ("Activation Functions", test_activations),
        ("Performance Benchmark", test_performance_benchmark),
        ("Position Calculation", test_position_calculation),
    ]
    
    passed = 0
    failed = 0
    
    for test_name, test_func in tests:
        print(f"\n🔍 Running {test_name} test...")
        if test_func():
            passed += 1
        else:
            failed += 1
    
    print("\n" + "=" * 50)
    print("📊 Test Results Summary")
    print(f"✅ Passed: {passed}")
    print(f"❌ Failed: {failed}")
    print(f"📈 Success Rate: {passed/(passed+failed)*100:.1f}%")
    
    if failed == 0:
        print("🎉 All tests passed! Rust integration is working correctly.")
    else:
        print("⚠️  Some tests failed. Check the error messages above.")
    
    return failed == 0

if __name__ == "__main__":
    success = run_all_tests()
    sys.exit(0 if success else 1)
