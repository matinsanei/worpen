#!/usr/bin/env python3
"""
Register helper_functions_demo.json route
Phase 3 Day 13: Helper Functions Library Demo
"""

import requests
import json

BASE_URL = "http://localhost:3000"

def register_route(route_file: str):
    """Register a route from JSON file"""
    print(f"\n📝 Registering route from {route_file}...")
    
    with open(route_file, 'r', encoding='utf-8') as f:
        route = json.load(f)
    
    response = requests.post(f"{BASE_URL}/api/routes/register", json=route)
    
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Route registered: {data['route']['method']} {data['route']['path']}")
        print(f"   ID: {data['route']['id']}")
        return data['route']['id']
    else:
        print(f"❌ Failed to register: {response.status_code}")
        print(f"   {response.text}")
        return None

def test_route(path: str):
    """Test the registered route"""
    print(f"\n🧪 Testing route: GET {path}")
    
    response = requests.get(f"{BASE_URL}{path}")
    
    print(f"Status: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print("\n✅ Response:")
        print(json.dumps(data, indent=2, ensure_ascii=False))
        
        # Verify helper functions worked
        helpers = data.get('helpers', {})
        print("\n📊 Helper Function Results:")
        print(f"  UUID: {helpers.get('uuid')}")
        print(f"  Timestamp: {helpers.get('timestamp')}")
        print(f"  Random Number: {helpers.get('random_num')}")
        print(f"  Random String: {helpers.get('random_string')}")
        print(f"  Slug: {helpers.get('slug')}")
        print(f"  Email User: {helpers.get('email_user')}")
        print(f"  Formatted Number: {helpers.get('formatted_number')}")
        print(f"  Formatted Bytes: {helpers.get('formatted_bytes')}")
        
        return True
    else:
        print(f"❌ Failed: {response.text}")
        return False

def main():
    print("=" * 60)
    print("Helper Functions Demo - Phase 3 Day 13")
    print("Testing 40+ utility functions")
    print("=" * 60)
    
    # Register route
    route_id = register_route("helper_functions_demo.json")
    
    if route_id:
        # Test route
        success = test_route("/api/helpers/demo")
        
        if success:
            print("\n" + "=" * 60)
            print("✅ All helper functions working!")
            print("=" * 60)
        else:
            print("\n❌ Helper functions test failed")
    else:
        print("\n❌ Route registration failed")

if __name__ == "__main__":
    main()
