import requests
import json

API_URL = "http://localhost:3000/api/v1/dynamic-routes"

# Load standard benchmark
with open('standard_benchmark.json', 'r', encoding='utf-8') as f:
    benchmark = json.load(f)

print("Registering Standard Benchmark Suite...")
response = requests.post(API_URL, json=benchmark)
print(f"Status: {response.status_code}")
print(f"Response: {response.json()}")

if response.status_code in [200, 201]:
    print("\n✅ Done! Open http://localhost:3000/standard-bench")
    print("\n📊 This benchmark includes:")
    print("  ✓ Warmup phase (eliminates JIT bias)")
    print("  ✓ Statistical analysis (P50, P95, P99, Std Dev)")
    print("  ✓ Multiple load levels (Normal, High, Extreme)")
    print("  ✓ Industry comparison (vs Go, Node.js, Python, Java)")
    print("  ✓ ISO/IEC 25010 compliance")
else:
    print(f"\n❌ Error: {response.text}")
