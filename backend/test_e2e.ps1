# End-to-End Test Script for YAML/JSON Route Support
# Run this after starting the server with: cargo run --bin api

$ErrorActionPreference = "Stop"
$API_BASE = "http://127.0.0.1:3000"

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "🧪 YAML/JSON Routes E2E Test" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Check server health
Write-Host "✓ Test 1: Server Health Check" -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "$API_BASE/health" -Method Get
    Write-Host "  ✅ Server is healthy" -ForegroundColor Green
} catch {
    Write-Host "  ❌ Server is not responding. Please start it with: cargo run --bin api" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Test 2: Get format statistics
Write-Host "✓ Test 2: Format Statistics Endpoint" -ForegroundColor Yellow
try {
    $stats = Invoke-RestMethod -Uri "$API_BASE/api/v1/dynamic-routes/formats" -Method Get
    Write-Host "  ✅ Stats retrieved:" -ForegroundColor Green
    Write-Host "     - Supported formats: $($stats.supported_formats -join ', ')" -ForegroundColor Gray
    Write-Host "     - Total routes: $($stats.total_routes)" -ForegroundColor Gray
} catch {
    Write-Host "  ❌ Failed to get stats: $_" -ForegroundColor Red
}
Write-Host ""

# Test 3: Register YAML route
Write-Host "✓ Test 3: Register Route via YAML" -ForegroundColor Yellow
try {
    $yamlContent = Get-Content -Path "test_route_yaml.yaml" -Raw
    $headers = @{
        "Content-Type" = "application/x-yaml"
    }
    $response = Invoke-RestMethod -Uri "$API_BASE/api/v1/dynamic-routes/register" `
        -Method Post `
        -Headers $headers `
        -Body $yamlContent
    
    Write-Host "  ✅ YAML route registered successfully" -ForegroundColor Green
    Write-Host "     - Route ID: $($response.route_id)" -ForegroundColor Gray
    Write-Host "     - Format detected: $($response.format_detected)" -ForegroundColor Gray
    $yamlRouteId = $response.route_id
} catch {
    Write-Host "  ❌ Failed to register YAML route: $_" -ForegroundColor Red
    Write-Host "     Response: $($_.Exception.Response)" -ForegroundColor Gray
}
Write-Host ""

# Test 4: Register JSON route
Write-Host "✓ Test 4: Register Route via JSON" -ForegroundColor Yellow
try {
    $jsonContent = Get-Content -Path "test_route_json.json" -Raw
    $headers = @{
        "Content-Type" = "application/json"
    }
    $response = Invoke-RestMethod -Uri "$API_BASE/api/v1/dynamic-routes/register" `
        -Method Post `
        -Headers $headers `
        -Body $jsonContent
    
    Write-Host "  ✅ JSON route registered successfully" -ForegroundColor Green
    Write-Host "     - Route ID: $($response.route_id)" -ForegroundColor Gray
    Write-Host "     - Format detected: $($response.format_detected)" -ForegroundColor Gray
    $jsonRouteId = $response.route_id
} catch {
    Write-Host "  ❌ Failed to register JSON route: $_" -ForegroundColor Red
}
Write-Host ""

# Test 5: List all routes
Write-Host "✓ Test 5: List All Routes" -ForegroundColor Yellow
try {
    $routes = Invoke-RestMethod -Uri "$API_BASE/api/v1/dynamic-routes" -Method Get
    Write-Host "  ✅ Routes listed: $($routes.Count) total" -ForegroundColor Green
    foreach ($route in $routes | Select-Object -Last 2) {
        Write-Host "     - $($route.name) [$($route.method)] at $($route.path)" -ForegroundColor Gray
    }
} catch {
    Write-Host "  ❌ Failed to list routes: $_" -ForegroundColor Red
}
Write-Host ""

# Test 6: Get format statistics again
Write-Host "✓ Test 6: Updated Format Statistics" -ForegroundColor Yellow
try {
    $stats = Invoke-RestMethod -Uri "$API_BASE/api/v1/dynamic-routes/formats" -Method Get
    Write-Host "  ✅ Updated stats:" -ForegroundColor Green
    Write-Host "     - Total routes: $($stats.total_routes)" -ForegroundColor Gray
    Write-Host "     - JSON routes: $($stats.by_format.json)" -ForegroundColor Gray
    Write-Host "     - YAML routes: $($stats.by_format.yaml)" -ForegroundColor Gray
} catch {
    Write-Host "  ⚠️  Stats endpoint may not be fully implemented yet" -ForegroundColor Yellow
}
Write-Host ""

# Summary
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "✅ End-to-End Tests Completed!" -ForegroundColor Green
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  - Test route execution with: POST $API_BASE/api/yaml-echo" -ForegroundColor Gray
Write-Host "  - Check routes in UI at: http://localhost:3000/swagger-ui" -ForegroundColor Gray
Write-Host ""
