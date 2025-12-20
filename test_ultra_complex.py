import requests
import json

# Load YAML
with open('ultra_complex_test.yaml', 'r') as f:
    yaml_content = f.read()

# Register route
print("="*80)
print("ULTRA COMPLEX YAML TEST - DETAILED VALIDATION")
print("="*80)
print("\n1️⃣ Registering route...")
reg = requests.post(
    'http://127.0.0.1:3000/api/v1/dynamic-routes/register',
    data=yaml_content,
    headers={'Content-Type': 'application/x-yaml'}
)
print(f"   Status: {reg.status_code} {'✅' if reg.status_code == 201 else '❌'}")

# Execute route
print("\n2️⃣ Executing route...")
exec_res = requests.post(
    'http://127.0.0.1:3000/api/test/ultra-complex',
    json={'user_data': {'name': 'test'}}
)
print(f"   Status: {exec_res.status_code} {'✅' if exec_res.status_code == 200 else '❌'}")

# Parse results
r = exec_res.json()

# Validate each field
print("\n3️⃣ Validating calculations...\n")

tests = [
    ("Sum (85+92+78+95+88)", r.get("total"), "438.0"),
    ("Average (438/5)", r.get("average"), "87.6"),
    ("Grade (80≤87.6<90)", r.get("grade"), "B"),
    ("Status (uppercase)", r.get("status"), "GOOD"),
    ("Multiplier (B bonus)", r.get("multiplier"), "1.05"),
    ("Bonus Score (87.6×1.05)", r.get("bonus_score"), "91.98"),
    ("Final Score (round)", r.get("final_score"), "92.0"),
    ("Has Bonus (92<95)", r.get("has_bonus"), "false"),
    ("Bonus Amount", r.get("bonus_amount"), "0"),
    ("Reward", r.get("reward"), "No Reward"),
    ("Level", r.get("level"), "basic"),
    ("Calculations", r.get("calculations"), 30),
]

passed = 0
failed = 0

for name, actual, expected in tests:
    match = str(actual) == str(expected)
    if match:
        passed += 1
        print(f"   ✅ {name:30} {str(actual):15} (correct)")
    else:
        failed += 1
        print(f"   ❌ {name:30} {str(actual):15} (expected: {expected})")

print(f"\n   Message: {r.get('message')}")

print(f"\n📊 Results: {passed}/{len(tests)} passed, {failed} failed")
print("="*80)
