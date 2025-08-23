
pub struct NeuronMath;

impl NeuronMath {
    pub fn new() -> Self {
        Self
    }
    
    
    pub fn relu(&self, inputs: &[f64]) -> Vec<f64> {
        inputs.iter().map(|&x| x.max(0.0)).collect()
    }
    
    
    pub fn sigmoid(&self, inputs: &[f64]) -> Vec<f64> {
        inputs.iter().map(|&x| 1.0 / (1.0 + (-x).exp())).collect()
    }
    
    
    pub fn tanh(&self, inputs: &[f64]) -> Vec<f64> {
        inputs.iter().map(|&x| x.tanh()).collect()
    }
    
    
    pub fn softmax(&self, inputs: &[f64]) -> Vec<f64> {
        if inputs.is_empty() {
            return Vec::new();
        }
        
        
        let max_val = inputs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        
        let exp_vals: Vec<f64> = inputs.iter()
            .map(|&x| (x - max_val).exp())
            .collect();
        
        
        let sum: f64 = exp_vals.iter().sum();
        
        
        exp_vals.iter().map(|&x| x / sum).collect()
    }
    
    
    pub fn matrix_multiply(&self, a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
        if a.is_empty() || b.is_empty() {
            return Err("Empty matrices".to_string());
        }
        
        let rows_a = a.len();
        let cols_a = a[0].len();
        let rows_b = b.len();
        let cols_b = b[0].len();
        
        if cols_a != rows_b {
            return Err("Incompatible matrix dimensions".to_string());
        }
        
        let mut result = vec![vec![0.0; cols_b]; rows_a];
        
        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        
        Ok(result)
    }
    
    
    pub fn calculate_conv_output_size(
        &self,
        input_size: u32,
        kernel_size: u32,
        stride: u32,
        padding: u32,
    ) -> u32 {
        ((input_size + 2 * padding - kernel_size) / stride) + 1
    }
    
    
    pub fn normalize(&self, inputs: &[f64]) -> Vec<f64> {
        if inputs.is_empty() {
            return Vec::new();
        }
        
        let mean: f64 = inputs.iter().sum::<f64>() / inputs.len() as f64;
        let variance: f64 = inputs.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / inputs.len() as f64;
        
        let std_dev = (variance + 1e-8).sqrt(); 
        
        inputs.iter()
            .map(|&x| (x - mean) / std_dev)
            .collect()
    }
    
    
    pub fn clip_gradients(&self, gradients: &[f64], max_norm: f64) -> Vec<f64> {
        let norm: f64 = gradients.iter().map(|&x| x * x).sum::<f64>().sqrt();
        
        if norm > max_norm {
            let scale = max_norm / norm;
            gradients.iter().map(|&x| x * scale).collect()
        } else {
            gradients.to_vec()
        }
    }
}
