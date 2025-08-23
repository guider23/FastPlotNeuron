
const express = require("express");
const bodyParser = require("body-parser");
const path = require("path");

const app = express();
const PORT = 3000;

app.use(bodyParser.json());
app.use("/outputs", express.static(path.join(__dirname, "plotter/output")));


const renderRustRoute = require("./routes/render_rust");
app.use("/generate", renderRustRoute);


app.get("/performance", (req, res) => {
  res.json({
    rust_enabled: true,
    features: [
      "Fast JSON parsing",
      "Optimized layer processing", 
      "Mathematical computations acceleration",
      "CLI tools for validation and benchmarking"
    ],
    endpoints: {
      "/generate": "Generate neural network diagrams (Rust-accelerated)",
      "/generate/benchmark": "Benchmark parsing performance",
      "/validate": "Validate neural network JSON"
    }
  });
});


app.get("/health", (req, res) => {
  res.json({
    status: "healthy",
    rust_acceleration: "enabled",
    timestamp: new Date().toISOString()
  });
});

app.listen(PORT, () => {
  console.log(`🚀 PlotNeuron Server (Rust-accelerated) running at http:
  console.log(`🦀 Rust integration enabled for enhanced performance`);
  console.log(`📊 Performance endpoint: http:
});
