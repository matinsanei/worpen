import requests
import json

API_URL = "http://localhost:3000/api/v1/dynamic-routes"

# Load Yalda showcase
with open('yalda_showcase.json', 'r', encoding='utf-8') as f:
    showcase = json.load(f)

print("[*] Registering Yalda Showcase...")
response = requests.post(API_URL, json=showcase)
print(f"Status: {response.status_code}")
print(f"Response: {response.json()}")

if response.status_code in [200, 201]:
    print("\n[✓] شب یلدا مبارک!")
    print("[~] Open: http://localhost:3000/yalda")
    print("\n[i] این صفحه شامل:")
    print("  [-] تبریک شب یلدا با طراحی خاص")
    print("  [-] معرفی کامل Worpen Framework")
    print("  [-] نتایج Benchmark و مقایسه")
    print("  [-] نمونه کدها")
    print("  [-] دلایل استفاده از Worpen")
    print("\n[!] آماده برای اشتراک‌گذاری در کانال!")
else:
    print(f"\n[X] Error: {response.text}")
