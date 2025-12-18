import requests
import json

API_URL = "http://localhost:3000/api/v1/dynamic-routes"

# Load benchmark HTML
with open('full_benchmark.json', 'r', encoding='utf-8') as f:
    benchmark_route = json.load(f)

print("Registering Full System Benchmark...")
response = requests.post(API_URL, json=benchmark_route)
print(f"Status: {response.status_code}")
print(f"Response: {response.json()}")

if response.status_code in [200, 201]:
    print("\n✅ Done! Open http://localhost:3000/benchmark")
else:
    print(f"\n❌ Error: {response.text}")
