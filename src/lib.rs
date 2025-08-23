use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod layer_processor;
mod neuron_math;

use layer_processor::{LayerProcessor, LayerConfig};
use neuron_math::NeuronMath;

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


#[pyfunction]
fn parse_network_json(json_str: &str) -> PyResult<String> {
    let architecture: NetworkArchitecture = serde_json::from_str(json_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parsing error: {}", e)))?;
    
    
    for (i, layer) in architecture.layers.iter().enumerate() {
        if layer.layer_type.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Layer {} has empty type", i)
            ));
        }
    }
    
    Ok(serde_json::to_string_pretty(&architecture).unwrap())
}


#[pyfunction]
fn process_layers(json_str: &str) -> PyResult<Vec<HashMap<String, String>>> {
    let architecture: NetworkArchitecture = serde_json::from_str(json_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parsing error: {}", e)))?;
    
    let processor = LayerProcessor::new();
    let mut results = Vec::new();
    
    for (i, layer) in architecture.layers.iter().enumerate() {
        let config = LayerConfig {
            layer_type: layer.layer_type.clone(),
            filters: layer.filters,
            kernel_size: layer.kernel_size,
            units: layer.units,
            index: i,
        };
        
        let processed = processor.process_layer(&config);
        
        let mut result_map = HashMap::new();
        result_map.insert("name".to_string(), layer.name.clone().unwrap_or_else(|| format!("layer{}", i)));
        result_map.insert("type".to_string(), layer.layer_type.clone());
        result_map.insert("width".to_string(), processed.width.to_string());
        result_map.insert("height".to_string(), processed.height.to_string());
        result_map.insert("depth".to_string(), processed.depth.to_string());
        result_map.insert("position".to_string(), processed.position);
        
        results.push(result_map);
    }
    
    Ok(results)
}


#[pyfunction] 
fn compute_activations(inputs: Vec<f64>, activation_type: &str) -> PyResult<Vec<f64>> {
    let math = NeuronMath::new();
    
    let result = match activation_type.to_lowercase().as_str() {
        "relu" => math.relu(&inputs),
        "sigmoid" => math.sigmoid(&inputs),
        "tanh" => math.tanh(&inputs),
        "softmax" => math.softmax(&inputs),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unsupported activation function: {}", activation_type)
        )),
    };
    
    Ok(result)
}


#[pyfunction]
fn calculate_layer_positions(layer_count: usize, spacing: f64) -> PyResult<Vec<String>> {
    let mut positions = Vec::new();
    
    for i in 0..layer_count {
        let x = i as f64 * spacing;
        let position = format!("({:.1},0,0)", x);
        positions.push(position);
    }
    
    Ok(positions)
}


#[pyfunction]
fn benchmark_json_parsing(json_str: &str, iterations: usize) -> PyResult<f64> {
    let start = std::time::Instant::now();
    
    for _ in 0..iterations {
        let _: NetworkArchitecture = serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Parsing error: {}", e)))?;
    }
    
    let duration = start.elapsed();
    Ok(duration.as_secs_f64())
}


#[pymodule]
fn plotneuron_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_network_json, m)?)?;
    m.add_function(wrap_pyfunction!(process_layers, m)?)?;
    m.add_function(wrap_pyfunction!(compute_activations, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_layer_positions, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_json_parsing, m)?)?;
    Ok(())
}
