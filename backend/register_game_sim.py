import requests
import json

# Register game action API
with open('game_action_api.json', 'r', encoding='utf-8') as f:
    api_data = json.load(f)

print("Registering game action API...")
api_response = requests.post(
    'http://localhost:3000/api/v1/dynamic-routes',
    json=api_data
)
print("API Status:", api_response.status_code)
print("API Response:", api_response.text)

# Register simulator page
with open('game_simulator.json', 'r', encoding='utf-8') as f:
    sim_data = json.load(f)

print("\nRegistering simulator page...")
sim_response = requests.post(
    'http://localhost:3000/api/v1/dynamic-routes',
    json=sim_data
)
print("Sim Status:", sim_response.status_code)
print("Sim Response:", sim_response.text)

print("\n✅ Done! Open http://localhost:3000/game-sim")
