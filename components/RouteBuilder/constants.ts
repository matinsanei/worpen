export const ROUTE_TEMPLATES = {
    yamlSimple: {
        name: "YAML: Hello World",
        json: `name: Hello World API
description: Simple greeting endpoint
path: /api/custom/hello
method: GET
enabled: true
version: 1.0.0
auth_required: false

logic:
  - return:
      value:
        message: Hello from Worpen!
        timestamp: "{{now()}}"
        status: success`
    },
    yamlConditional: {
        name: "YAML: Age Checker",
        json: `name: Age Verification
description: Check if user is adult
path: /api/custom/verify-age
method: POST
enabled: true

parameters:
  - name: age
    param_type: body
    data_type: number
    required: true

logic:
  - if:
      condition: age >= 18
      then:
        - return:
            value:
              status: adult
              message: Access granted
              can_vote: true
      otherwise:
        - return:
            value:
              status: minor
              message: Access denied
              can_vote: false`
    },
    yamlDatabase: {
        name: "YAML: DB Query",
        json: `name: User Lookup
description: Find user by ID
path: /api/custom/user/:id
method: GET
enabled: true

parameters:
  - name: id
    param_type: path
    data_type: number
    required: true

logic:
  - query_db:
      query: SELECT * FROM agents WHERE id = $1 LIMIT 1
      params:
        - "{{path.id}}"
  
  - if:
      condition: db_result.length > 0
      then:
        - return:
            value:
              found: true
              user: "{{db_result[0]}}"
      otherwise:
        - return:
            value:
              found: false
              error: User not found`
    },
    yamlLoop: {
        name: "YAML: Loop Example",
        json: `name: Batch Processor
description: Process multiple items
path: /api/custom/process-batch
method: POST
enabled: true

parameters:
  - name: items
    param_type: body
    data_type: array
    required: true

logic:
  - set:
      var: results
      value: []
  
  - for_each:
      collection: "{{body.items}}"
      operations:
        - set:
            var: processed
            value:
              id: "{{item.id}}"
              status: processed
              timestamp: "{{now()}}"
        
        - set:
            var: results
            value: "{{results + [processed]}}"
  
  - return:
      value:
        total: "{{body.items.length}}"
        processed: "{{results}}"`
    },
    simple: {
        name: "JSON: Simple Echo",
        json: JSON.stringify({
            name: "Echo API",
            description: "Returns your input",
            path: "/api/custom/echo",
            method: "POST",
            logic: [{
                "return": { "value": { "message": "{{input}}" } }
            }],
            parameters: [],
            enabled: true,
            version: "1.0.0",
            auth_required: false
        }, null, 2)
    },
    database: {
        name: "Database Query",
        json: JSON.stringify({
            name: "Users API",
            description: "Get users from database",
            path: "/api/custom/users",
            method: "GET",
            logic: [
                { "query_db": { "query": "SELECT * FROM agents LIMIT 10", "params": [] } },
                { "return": { "value": { "users": "{{db_result}}" } } }
            ],
            parameters: [],
            enabled: true,
            version: "1.0.0",
            auth_required: false
        }, null, 2)
    },
    conditional: {
        name: "Conditional Logic",
        json: JSON.stringify({
            name: "Age Check API",
            description: "Check if user is adult",
            path: "/api/custom/age-check",
            method: "POST",
            logic: [{
                "if": {
                    "condition": "age >= 18",
                    "then": [{ "return": { "value": { "status": "adult", "can_vote": true } } }],
                    "otherwise": [{ "return": { "value": { "status": "minor", "can_vote": false } } }]
                }
            }],
            parameters: [{ "name": "age", "param_type": "body", "data_type": "number", "required": true }],
            enabled: true,
            version: "1.0.0",
            auth_required: false
        }, null, 2)
    },
    advanced: {
        name: "Advanced Showcase",
        json: JSON.stringify({
            name: "Order Processing API",
            description: "Complex order processing with all features",
            path: "/api/custom/process-order",
            method: "POST",
            logic: [
                {
                    "set": {
                        "var": "user_id",
                        "value": "{{body.user_id}}"
                    }
                },
                {
                    "query_db": {
                        "query": "SELECT balance FROM users WHERE id = $1",
                        "params": ["{{user_id}}"]
                    }
                },
                {
                    "set": {
                        "var": "balance",
                        "value": "{{db_result[0].balance}}"
                    }
                },
                {
                    "if": {
                        "condition": "balance >= body.amount",
                        "then": [
                            {
                                "http_request": {
                                    "url": "https://api.payment.com/charge",
                                    "method": "POST",
                                    "body": {
                                        "user_id": "{{user_id}}",
                                        "amount": "{{body.amount}}"
                                    }
                                }
                            },
                            {
                                "query_db": {
                                    "query": "INSERT INTO orders (user_id, amount, status) VALUES ($1, $2, $3)",
                                    "params": ["{{user_id}}", "{{body.amount}}", "completed"]
                                }
                            },
                            {
                                "return": {
                                    "value": {
                                        "success": true,
                                        "order_id": "{{db_result.id}}",
                                        "message": "Order processed successfully"
                                    }
                                }
                            }
                        ],
                        "otherwise": [
                            {
                                "return": {
                                    "value": {
                                        "success": false,
                                        "error": "Insufficient balance",
                                        "required": "{{body.amount}}",
                                        "available": "{{balance}}"
                                    }
                                }
                            }
                        ]
                    }
                }
            ],
            parameters: [
                { "name": "user_id", "param_type": "body", "data_type": "number", "required": true },
                { "name": "amount", "param_type": "body", "data_type": "number", "required": true }
            ],
            enabled: true,
            version: "1.0.0",
            auth_required: true
        }, null, 2)
    }
};

export const AVAILABLE_FONTS = [
    { name: 'JetBrains Mono', family: '"JetBrains Mono", monospace' },
    { name: 'Fira Code', family: '"Fira Code", monospace' },
    { name: 'IBM Plex Mono', family: '"IBM Plex Mono", monospace' },
    { name: 'Roboto Mono', family: '"Roboto Mono", monospace' },
    { name: 'Source Code Pro', family: '"Source Code Pro", monospace' },
    { name: 'Inconsolata', family: '"Inconsolata", monospace' },
    { name: 'Courier Prime', family: '"Courier Prime", monospace' },
    { name: 'Ubuntu Mono', family: '"Ubuntu Mono", monospace' },
    { name: 'Anonymous Pro', family: '"Anonymous Pro", monospace' },
    { name: 'Space Mono', family: '"Space Mono", monospace' },
    { name: 'Nanum Gothic Coding', family: '"Nanum Gothic Coding", monospace' },
    { name: 'Overpass Mono', family: '"Overpass Mono", monospace' },
    { name: 'Cutive Mono', family: '"Cutive Mono", monospace' },
    { name: 'Nova Mono', family: '"Nova Mono", monospace' },
];
