import requests
import json

# Register physics API
with open('physics_api.json', 'r', encoding='utf-8') as f:
    api_data = json.load(f)

print("Registering physics API...")
api_response = requests.post(
    'http://localhost:3000/api/v1/dynamic-routes',
    json=api_data
)
print("Status:", api_response.status_code)
print("Response:", api_response.text)

# Register game page
with open('physics_game.json', 'r', encoding='utf-8') as f:
    game_data = json.load(f)

print("\nRegistering game page...")
game_response = requests.post(
    'http://localhost:3000/api/v1/dynamic-routes',
    json=game_data
)
print("Status:", game_response.status_code)
print("Response:", game_response.text)
