import requests
import json

with open('demo_route.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

response = requests.post(
    'http://localhost:3000/api/v1/dynamic-routes',
    json=data
)

print(response.json())
