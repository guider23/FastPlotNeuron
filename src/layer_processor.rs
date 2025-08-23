use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LayerConfig {
    pub layer_type: String,
    pub filters: Option<u32>,
    pub kernel_size: Option<u32>,
    pub units: Option<u32>,
    pub index: usize,
}

#[derive(Debug, Serialize)]
pub struct ProcessedLayer {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub position: String,
    pub connections: Vec<String>,
}

pub struct LayerProcessor {
    default_spacing: f64,
}

impl LayerProcessor {
    pub fn new() -> Self {
        Self {
            default_spacing: 5.0,
        }
    }
    
    pub fn process_layer(&self, config: &LayerConfig) -> ProcessedLayer {
        let (width, height, depth) = self.calculate_dimensions(config);
        let position = self.calculate_position(config);
        
        ProcessedLayer {
            width,
            height,
            depth,
            position,
            connections: vec![], 
        }
    }
    
    fn calculate_dimensions(&self, config: &LayerConfig) -> (f64, f64, f64) {
        match config.layer_type.to_lowercase().as_str() {
            "conv" | "convolution" => {
                let filters = config.filters.unwrap_or(64) as f64;
                let kernel = config.kernel_size.unwrap_or(3) as f64;
                (
                    2.0 + (filters / 100.0), 
                    15.0 + kernel,           
                    15.0 + kernel,           
                )
            }
            "pool" | "maxpool" | "avgpool" => {
                (1.0, 8.0, 8.0) 
            }
            "dense" | "linear" | "fc" => {
                let units = config.units.unwrap_or(128) as f64;
                (
                    1.5,                     
                    3.0 + (units / 50.0),    
                    8.0 + (units / 100.0),   
                )
            }
            "dropout" => (1.0, 1.0, 8.0), 
            "batchnorm" | "bn" => (0.5, 12.0, 12.0), 
            _ => (2.0, 10.0, 10.0), 
        }
    }
    
    fn calculate_position(&self, config: &LayerConfig) -> String {
        let x = config.index as f64 * self.default_spacing;
        format!("({:.1},0,0)", x)
    }
    
    
    pub fn calculate_optimal_spacing(&self, layers: &[LayerConfig]) -> f64 {
        if layers.is_empty() {
            return self.default_spacing;
        }
        
        let total_layers = layers.len() as f64;
        let base_spacing = 4.0;
        
        
        if total_layers > 10.0 {
            base_spacing * 0.8 
        } else if total_layers < 4.0 {
            base_spacing * 1.3 
        } else {
            base_spacing
        }
    }
    
    
    pub fn generate_connections(&self, layers: &[LayerConfig]) -> Vec<(String, String)> {
        let mut connections = Vec::new();
        
        for i in 1..layers.len() {
            let prev_name = format!("layer{}", i - 1);
            let curr_name = format!("layer{}", i);
            connections.push((prev_name, curr_name));
        }
        
        connections
    }
}
