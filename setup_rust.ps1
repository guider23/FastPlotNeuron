


Write-Host " PlotNeuron Rust Integration Setup" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan


$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "Please run this script as Administrator" -ForegroundColor Red
    exit 1
}


function Test-Command($cmd) {
    try {
        Get-Command $cmd -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}


Write-Host "🔍 Checking prerequisites..." -ForegroundColor Yellow

$prerequisites = @()

if (Test-Command "rustc") {
    $rustVersion = rustc --version
    Write-Host "⭐ Rust found: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "💀 Rust not found. Installing..." -ForegroundColor Red

    if (Test-Command "choco") {
        choco install rust -y
    } else {
        Write-Host "Please install Rust manually from: https://rustup.rs/" -ForegroundColor Yellow
        $prerequisites += "Rust"
    }
}

if (Test-Command "python") {
    $pythonVersion = python --version
    Write-Host "✅ Python found: $pythonVersion" -ForegroundColor Green
} else {
    Write-Host "🤣 Python not found" -ForegroundColor Red
    $prerequisites += "Python 3.8+"
}

if (Test-Command "node") {
    $nodeVersion = node --version
    Write-Host "✅ Node.js found: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "💀 Node.js not found" -ForegroundColor Red
    $prerequisites += "Node.js"
}

if (Test-Command "pdflatex") {
    Write-Host "✅ LaTeX found" -ForegroundColor Green
} else {
    Write-Host "💀 LaTeX not found" -ForegroundColor Red
    $prerequisites += "LaTeX (MiKTeX or TeX Live)"
}

if (Test-Command "magick") {
    Write-Host "✅ ImageMagick found" -ForegroundColor Green
} else {
    Write-Host "😊 ImageMagick not found" -ForegroundColor Red
    $prerequisites += "ImageMagick"
}

if ($prerequisites.Count -gt 0) {
    Write-Host "❌ Missing prerequisites:" -ForegroundColor Red
    foreach ($req in $prerequisites) {
        Write-Host "   - $req" -ForegroundColor Red
    }
    Write-Host "Please install missing prerequisites and run this script again." -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ All prerequisites satisfied!" -ForegroundColor Green


Write-Host "`n😏👍 Setting up Rust environment..." -ForegroundColor Yellow

try {

    Write-Host "Installing maturin..." -ForegroundColor Gray
    pip install maturin


    Write-Host "Building Rust extension..." -ForegroundColor Gray
    maturin develop --release


    Write-Host "Building CLI tools..." -ForegroundColor Gray
    cargo build --release --bin neuron-cli

    Write-Host "✅ Rust setup completed!" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust setup failed: $_" -ForegroundColor Red
    exit 1
}


Write-Host "`n📦 Setting up Node.js environment..." -ForegroundColor Yellow

try {
    Set-Location "Server"
    npm install
    Set-Location ".."
    Write-Host "✅ Node.js setup completed!" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js setup failed: $_" -ForegroundColor Red
    exit 1
}


Write-Host "`n Testing installation..." -ForegroundColor Yellow

try {

    $testOutput = cargo run --bin neuron-cli validate examples/cnn_example.json 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Rust CLI working!" -ForegroundColor Green
    } else {
        Write-Host "🖕  Rust CLI test had issues, but continuing..." -ForegroundColor Yellow
    }


    python -c "import plotneuron_rust; print('✅ Python extension working!')"
    
} catch {
    Write-Host "⚠️  Some tests failed, but installation may still work: $_" -ForegroundColor Yellow
}

Write-Host "`n🎉 Setup Complete!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Test CLI: cargo run --bin neuron-cli validate examples/cnn_example.json" -ForegroundColor Gray
Write-Host "2. Start server: cd Server && npm run start-rust" -ForegroundColor Gray
Write-Host "3. Open browser: http://localhost:3000" -ForegroundColor Gray
Write-Host "4. Run benchmarks: python Server/scripts/generate_rust.py examples/cnn_example.json --benchmark" -ForegroundColor Gray

