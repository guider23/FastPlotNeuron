const express = require("express");
const router = express.Router();
const { exec } = require("child_process");
const path = require("path");
const fs = require("fs");

router.post("/", (req, res) => {
  console.log("🔄 Processing neural network generation request...");
  
  
  const tempDir = path.join(__dirname, "..", "temp");
  if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir);
  const jsonPath = path.join(tempDir, "arch.json");
  fs.writeFileSync(jsonPath, JSON.stringify(req.body, null, 2));

  
  const absJsonPath = path.resolve(jsonPath);
  
  
  const benchmark = req.query.benchmark === 'true' ? '--benchmark' : '';

  
  const command = `python scripts/generate_rust.py "${absJsonPath}" ${benchmark}`;
  
  console.log(`🦀 Executing with Rust acceleration: ${command}`);
  
  const startTime = Date.now();
  
  exec(command, (err, stdout, stderr) => {
    const duration = Date.now() - startTime;
    
    console.log("STDOUT:\n", stdout);
    console.log("STDERR:\n", stderr);
    console.log(`⏱️  Total execution time: ${duration}ms`);
    
    if (err) {
      console.error("ERROR OCCURRED:", err);
      return res.status(500).json({
        error: "Generation failed",
        details: `err: ${err}\nstdout: ${stdout}\nstderr: ${stderr}`,
        rust_acceleration: "failed"
      });
    }
    
    
    let performanceInfo = {};
    if (stdout.includes('Performance Summary:')) {
      const lines = stdout.split('\n');
      for (let line of lines) {
        if (line.includes('LaTeX compilation:')) {
          performanceInfo.latex_time = line.match(/(\d+\.\d+)s/)?.[1];
        }
        if (line.includes('PNG conversion:')) {
          performanceInfo.convert_time = line.match(/(\d+\.\d+)s/)?.[1];
        }
        if (line.includes('Total time:')) {
          performanceInfo.total_time = line.match(/(\d+\.\d+)s/)?.[1];
        }
      }
    }
    
    return res.json({ 
      imageUrl: "/outputs/main.png",
      rust_acceleration: "enabled",
      execution_time_ms: duration,
      performance: performanceInfo
    });
  });
});


router.post("/benchmark", (req, res) => {
  console.log("📊 Running performance benchmark...");
  
  const tempDir = path.join(__dirname, "..", "temp");
  if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir);
  const jsonPath = path.join(tempDir, "benchmark.json");
  fs.writeFileSync(jsonPath, JSON.stringify(req.body, null, 2));

  const absJsonPath = path.resolve(jsonPath);
  const command = `python scripts/generate_rust.py "${absJsonPath}" --benchmark`;
  
  exec(command, (err, stdout, stderr) => {
    if (err) {
      return res.status(500).json({
        error: "Benchmark failed",
        details: stderr
      });
    }
    
    
    let benchmarkResults = {
      rust_available: stdout.includes('Rust:'),
      speedup: null,
      rust_time: null,
      python_time: null
    };
    
    const lines = stdout.split('\n');
    for (let line of lines) {
      if (line.includes('🦀 Rust:')) {
        benchmarkResults.rust_time = line.match(/(\d+\.\d+)s/)?.[1];
      }
      if (line.includes('🐍 Python:')) {
        benchmarkResults.python_time = line.match(/(\d+\.\d+)s/)?.[1];
      }
      if (line.includes('⚡ Speedup:')) {
        benchmarkResults.speedup = line.match(/(\d+\.\d+)x/)?.[1];
      }
    }
    
    res.json({
      benchmark_results: benchmarkResults,
      full_output: stdout
    });
  });
});


router.post("/validate", (req, res) => {
  console.log("✅ Validating neural network JSON...");
  
  const tempDir = path.join(__dirname, "..", "temp");
  if (!fs.existsSync(tempDir)) fs.mkdirSync(tempDir);
  const jsonPath = path.join(tempDir, "validate.json");
  fs.writeFileSync(jsonPath, JSON.stringify(req.body, null, 2));

  const absJsonPath = path.resolve(jsonPath);
  const command = `cargo run --bin neuron-cli validate "${absJsonPath}"`;
  
  exec(command, { cwd: path.join(__dirname, "..") }, (err, stdout, stderr) => {
    if (err) {
      return res.status(400).json({
        valid: false,
        error: stderr || err.message
      });
    }
    
    
    const isValid = stdout.includes('✅ JSON is valid!');
    const layerCount = stdout.match(/Layers: (\d+)/)?.[1];
    const networkName = stdout.match(/Name: (.+)/)?.[1];
    
    res.json({
      valid: isValid,
      layer_count: layerCount ? parseInt(layerCount) : null,
      network_name: networkName,
      rust_validation: true,
      details: stdout
    });
  });
});

module.exports = router;
