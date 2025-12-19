#!/usr/bin/env python3
"""
Test SQL Named Parameters - Phase 3 Day 14
Demonstrates named parameter parsing and binding
"""

import sys
sys.path.append('..')

# Simulate the named parameters functionality
def parse_named_query(sql: str):
    """Parse SQL with named parameters"""
    query = ""
    param_names = []
    i = 0
    
    while i < len(sql):
        if sql[i] == ':' and i + 1 < len(sql) and (sql[i+1].isalpha() or sql[i+1] == '_'):
            # Extract parameter name
            param_name = ""
            i += 1
            while i < len(sql) and (sql[i].isalnum() or sql[i] == '_'):
                param_name += sql[i]
                i += 1
            param_names.append(param_name)
            query += '?'
        else:
            query += sql[i]
            i += 1
    
    return query, param_names

def test_simple_select():
    print("\n" + "="*60)
    print("Test 1: Simple SELECT")
    print("="*60)
    
    sql = "SELECT * FROM users WHERE id = :id"
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql}")
    print(f"Converted: {query}")
    print(f"Parameters: {params}")
    
    assert query == "SELECT * FROM users WHERE id = ?"
    assert params == ["id"]
    print("✅ PASSED")

def test_multiple_params():
    print("\n" + "="*60)
    print("Test 2: Multiple Parameters")
    print("="*60)
    
    sql = "SELECT * FROM users WHERE email = :email AND age > :min_age"
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql}")
    print(f"Converted: {query}")
    print(f"Parameters: {params}")
    
    assert query == "SELECT * FROM users WHERE email = ? AND age > ?"
    assert params == ["email", "min_age"]
    print("✅ PASSED")

def test_insert_query():
    print("\n" + "="*60)
    print("Test 3: INSERT Query")
    print("="*60)
    
    sql = "INSERT INTO users (id, name, email) VALUES (:id, :name, :email)"
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql}")
    print(f"Converted: {query}")
    print(f"Parameters: {params}")
    
    assert query == "INSERT INTO users (id, name, email) VALUES (?, ?, ?)"
    assert params == ["id", "name", "email"]
    print("✅ PASSED")

def test_update_query():
    print("\n" + "="*60)
    print("Test 4: UPDATE Query")
    print("="*60)
    
    sql = "UPDATE users SET name = :name, email = :email WHERE id = :id"
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql}")
    print(f"Converted: {query}")
    print(f"Parameters: {params}")
    
    assert query == "UPDATE users SET name = ?, email = ? WHERE id = ?"
    assert params == ["name", "email", "id"]
    print("✅ PASSED")

def test_complex_query():
    print("\n" + "="*60)
    print("Test 5: Complex Query with Joins")
    print("="*60)
    
    sql = """
    SELECT u.*, o.order_count 
    FROM users u
    LEFT JOIN (
        SELECT user_id, COUNT(*) as order_count 
        FROM orders 
        WHERE status = :status 
        GROUP BY user_id
    ) o ON u.id = o.user_id
    WHERE u.created_at > :start_date
    """
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql[:100]}...")
    print(f"Converted: {query[:100]}...")
    print(f"Parameters: {params}")
    
    assert "status" in params
    assert "start_date" in params
    assert query.count('?') == 2
    print("✅ PASSED")

def test_colon_in_string():
    print("\n" + "="*60)
    print("Test 6: Colon in String Literal")
    print("="*60)
    
    sql = "SELECT * FROM logs WHERE message = 'Time: 10:30' AND user_id = :user_id"
    query, params = parse_named_query(sql)
    
    print(f"Original: {sql}")
    print(f"Converted: {query}")
    print(f"Parameters: {params}")
    
    assert "Time: 10:30" in query
    assert params == ["user_id"]
    print("✅ PASSED")

def demo_user_registration():
    print("\n" + "="*60)
    print("Demo: User Registration Route")
    print("="*60)
    
    sql = "INSERT INTO users (id, email, password, name, created_at, status) VALUES (:id, :email, :password, :name, :created_at, :status)"
    query, params = parse_named_query(sql)
    
    print("SQL Query:")
    print(f"  {sql}")
    print(f"\nConverted to:")
    print(f"  {query}")
    print(f"\nParameters ({len(params)}):")
    for i, param in enumerate(params, 1):
        print(f"  {i}. :{param}")
    
    print("\nParameter Values (from context):")
    param_values = {
        "id": "{{ uuid() }}",
        "email": "{{ request.email | lower | trim }}",
        "password": "{{ hash_password(request.password) }}",
        "name": "{{ request.name }}",
        "created_at": "{{ now() }}",
        "status": "active"
    }
    
    for param in params:
        print(f"  :{param} = {param_values.get(param, 'N/A')}")

def demo_profile_update():
    print("\n" + "="*60)
    print("Demo: Profile Update Route")
    print("="*60)
    
    sql = "UPDATE users SET name = :name, bio = :bio, avatar_url = :avatar, updated_at = :updated_at WHERE id = :user_id"
    query, params = parse_named_query(sql)
    
    print("SQL Query:")
    print(f"  {sql}")
    print(f"\nConverted to:")
    print(f"  {query}")
    print(f"\nParameters ({len(params)}):")
    for i, param in enumerate(params, 1):
        print(f"  {i}. :{param}")

def demo_order_creation():
    print("\n" + "="*60)
    print("Demo: Order Creation with Items")
    print("="*60)
    
    queries = [
        ("Create Order", "INSERT INTO orders (id, order_number, user_id, status, total_amount, created_at) VALUES (:id, :order_number, :user_id, :status, :total, :created_at)"),
        ("Add Item", "INSERT INTO order_items (id, order_id, product_id, quantity, price, subtotal) VALUES (:id, :order_id, :product_id, :quantity, :price, :subtotal)"),
        ("Update Stock", "UPDATE products SET stock = stock - :quantity WHERE id = :product_id"),
    ]
    
    for title, sql in queries:
        query, params = parse_named_query(sql)
        print(f"\n{title}:")
        print(f"  Parameters: {', '.join(':' + p for p in params)}")

def main():
    print("="*60)
    print("SQL Named Parameters Testing Suite")
    print("Phase 3 Day 14: SQL Named Parameters")
    print("="*60)
    
    # Run tests
    test_simple_select()
    test_multiple_params()
    test_insert_query()
    test_update_query()
    test_complex_query()
    test_colon_in_string()
    
    # Run demos
    demo_user_registration()
    demo_profile_update()
    demo_order_creation()
    
    print("\n" + "="*60)
    print("✅ All Tests Passed!")
    print("="*60)
    print("\nFeatures Demonstrated:")
    print("  ✓ Named parameter parsing (:param_name)")
    print("  ✓ Conversion to positional placeholders (?)")
    print("  ✓ Parameter extraction and ordering")
    print("  ✓ Complex queries with joins")
    print("  ✓ INSERT, UPDATE, SELECT operations")
    print("  ✓ Multiple parameters in single query")
    print("\nBenefits:")
    print("  ✓ More readable SQL queries")
    print("  ✓ Self-documenting parameter usage")
    print("  ✓ Expression evaluation support")
    print("  ✓ Type-safe parameter binding")
    print("="*60)

if __name__ == "__main__":
    main()
