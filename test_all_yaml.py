import requests
import json

def test_route(yaml_file, route_path, expected_checks, description):
    """Test a YAML route with validation"""
    print(f"\n{'='*80}")
    print(f"Testing: {description}")
    print(f"File: {yaml_file}")
    print(f"{'='*80}")
    
    # Load and register
    with open(yaml_file, 'r') as f:
        yaml_content = f.read()
    
    print("1️⃣ Registering route...")
    reg = requests.post(
        'http://127.0.0.1:3000/api/v1/dynamic-routes/register',
        data=yaml_content,
        headers={'Content-Type': 'application/x-yaml'}
    )
    print(f"   Status: {reg.status_code} {'✅' if reg.status_code == 201 else '❌'}")
    
    # Execute
    print("2️⃣ Executing route...")
    exec_res = requests.post(
        f'http://127.0.0.1:3000{route_path}',
        json={'user_data': {'name': 'test'}}
    )
    print(f"   Status: {exec_res.status_code} {'✅' if exec_res.status_code == 200 else '❌'}")
    
    # Validate
    r = exec_res.json()
    print("3️⃣ Validating results...\n")
    
    passed = 0
    failed = 0
    
    for check_name, field, expected in expected_checks:
        # Handle nested fields like "data.field"
        actual = r
        for part in field.split('.'):
            actual = actual.get(part) if isinstance(actual, dict) else None
            if actual is None:
                break
        
        match = str(actual) == str(expected)
        if match:
            passed += 1
            print(f"   ✅ {check_name:35} {str(actual):20}")
        else:
            failed += 1
            print(f"   ❌ {check_name:35} {str(actual):20} (expected: {expected})")
    
    print(f"\n📊 Results: {passed}/{len(expected_checks)} passed, {failed} failed")
    return passed, failed

# Test 1: Sequential Test
print("\n" + "🔬 COMPREHENSIVE YAML ROUTES TEST SUITE ".center(80, "="))

test1_checks = [
    ("Sum", "sum_result", "100.0"),
    ("Doubled", "doubled", "200.0"),
    ("Final", "final_math", "50.0"),
    ("Status", "status", "success"),
    ("Operations", "operations_performed", "15"),
]
p1, f1 = test_route(
    'sequential_test.yaml',
    '/api/test/sequential',
    test1_checks,
    "Sequential Operations (15 operations)"
)

# Test 2: Array Processing
test2_checks = [
    ("Total", "data.original_total", "1500.0"),
    ("Average", "data.average_price", "300.0"),
    ("Max", "data.max_price", "500.0"),
    ("Min", "data.min_price", "100.0"),
    ("Range", "data.price_range", "400.0"),
    ("Promo", "data.promo_message", "GREAT SAVINGS"),
    ("Calculations", "data.calculations_performed", "12"),
]
p2, f2 = test_route(
    'array_processing_test.yaml',
    '/api/test/array-processing',
    test2_checks,
    "Array Processing (12 calculations)"
)

# Test 3: Ultra Complex
test3_checks = [
    ("Total", "total", "438.0"),
    ("Average", "average", "87.6"),
    ("Grade", "grade", "B"),
    ("Status", "status", "GOOD"),
    ("Final Score", "final_score", "92.0"),
    ("Calculations", "calculations", "30"),
]
p3, f3 = test_route(
    'ultra_complex_test.yaml',
    '/api/test/ultra-complex',
    test3_checks,
    "Ultra Complex (30+ operations)"
)

# Summary
print("\n" + "="*80)
print("📊 FINAL SUMMARY")
print("="*80)
total_passed = p1 + p2 + p3
total_failed = f1 + f2 + f3
total_tests = total_passed + total_failed

print(f"\n✅ Sequential Test:     {p1}/{len(test1_checks)} passed")
print(f"✅ Array Processing:    {p2}/{len(test2_checks)} passed")
print(f"✅ Ultra Complex:       {p3}/{len(test3_checks)} passed")
print(f"\n🎯 TOTAL: {total_passed}/{total_tests} tests passed ({100*total_passed//total_tests}%)")

if total_failed == 0:
    print("\n🎉 ALL TESTS PASSED! 🎉")
else:
    print(f"\n⚠️  {total_failed} tests failed")

print("="*80)
