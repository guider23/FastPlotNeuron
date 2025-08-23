use std::env;
use std::fs;
use std::process;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    #[serde(rename = "type")]
    pub layer_type: String,
    pub name: Option<String>,
    pub filters: Option<u32>,
    pub kernel_size: Option<u32>,
    pub stride: Option<u32>,
    pub units: Option<u32>,
    pub activation: Option<String>,
    pub position: Option<String>,
    pub pool_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkArchitecture {
    pub layers: Vec<Layer>,
    pub name: Option<String>,
    pub description: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        eprintln!("Commands:");
        eprintln!("  validate <json_file>  - Validate neural network JSON");
        eprintln!("  benchmark <json_file> - Benchmark JSON parsing performance");
        eprintln!("  process <json_file>   - Process layers and show optimized parameters");
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "validate" => {
            if args.len() < 3 {
                eprintln!("Usage: {} validate <json_file>", args[0]);
                process::exit(1);
            }
            validate_json(&args[2]);
        }
        "benchmark" => {
            if args.len() < 3 {
                eprintln!("Usage: {} benchmark <json_file>", args[0]);
                process::exit(1);
            }
            benchmark_parsing(&args[2]);
        }
        "process" => {
            if args.len() < 3 {
                eprintln!("Usage: {} process <json_file>", args[0]);
                process::exit(1);
            }
            process_layers(&args[2]);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

fn validate_json(file_path: &str) {
    println!("🔍 Validating neural network JSON: {}", file_path);
    
    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let start = Instant::now();
    
    match serde_json::from_str::<NetworkArchitecture>(&json_content) {
        Ok(architecture) => {
            let duration = start.elapsed();
            println!("✅ JSON is valid!");
            println!("📊 Parsed in: {:.2}ms", duration.as_secs_f64() * 1000.0);
            println!("📋 Network info:");
            println!("   - Name: {}", architecture.name.unwrap_or_else(|| "Unnamed".to_string()));
            println!("   - Layers: {}", architecture.layers.len());
            
            for (i, layer) in architecture.layers.iter().enumerate() {
                println!("   - Layer {}: {} ({})", 
                    i + 1, 
                    layer.name.as_ref().unwrap_or(&format!("layer{}", i)),
                    layer.layer_type
                );
            }
        }
        Err(e) => {
            eprintln!("❌ JSON validation failed: {}", e);
            process::exit(1);
        }
    }
}

fn benchmark_parsing(file_path: &str) {
    println!("⚡ Benchmarking JSON parsing performance: {}", file_path);
    
    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        match serde_json::from_str::<NetworkArchitecture>(&json_content) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("❌ Parsing error during benchmark: {}", e);
                process::exit(1);
            }
        }
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / iterations as f64;
    
    println!("📊 Benchmark Results:");
    println!("   - Iterations: {}", iterations);
    println!("   - Total time: {:.2}s", duration.as_secs_f64());
    println!("   - Average per parse: {:.2}μs", avg_time * 1_000_000.0);
    println!("   - Parses per second: {:.0}", 1.0 / avg_time);
}

fn process_layers(file_path: &str) {
    println!("⚙️  Processing neural network layers: {}", file_path);
    
    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    let architecture: NetworkArchitecture = match serde_json::from_str(&json_content) {
        Ok(arch) => arch,
        Err(e) => {
            eprintln!("❌ JSON parsing failed: {}", e);
            process::exit(1);
        }
    };
    
    println!("📋 Processing {} layers:", architecture.layers.len());
    println!();
    
    for (i, layer) in architecture.layers.iter().enumerate() {
        let default_name = format!("layer{}", i);
        let name = layer.name.as_ref().unwrap_or(&default_name);
        println!("🔹 Layer {}: {} ({})", i + 1, name, layer.layer_type);
        
        match layer.layer_type.to_lowercase().as_str() {
            "conv" | "convolution" => {
                println!("   - Filters: {}", layer.filters.unwrap_or(64));
                println!("   - Kernel size: {}", layer.kernel_size.unwrap_or(3));
                println!("   - Stride: {}", layer.stride.unwrap_or(1));
            }
            "dense" | "linear" | "fc" => {
                println!("   - Units: {}", layer.units.unwrap_or(128));
                if let Some(activation) = &layer.activation {
                    println!("   - Activation: {}", activation);
                }
            }
            "pool" | "maxpool" | "avgpool" => {
                println!("   - Pool size: {}", layer.pool_size.unwrap_or(2));
            }
            _ => {
                println!("   - Custom layer type");
            }
        }
        
        if let Some(position) = &layer.position {
            println!("   - Position: {}", position);
        }
        
        println!();
    }
}
