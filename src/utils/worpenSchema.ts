/**
 * JSON Schema for Worpen Dynamic Routes
 * Provides validation and autocomplete for route definitions
 */

export interface WorpenSchema {
  $schema: string;
  type: string;
  [key: string]: any;
}

export const WORPEN_ROUTE_SCHEMA: WorpenSchema = {
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "title": "Worpen Dynamic Route Definition",
  "required": ["name", "path", "method"],
  "properties": {
    "name": {
      "type": "string",
      "description": "Route name"
    },
    "description": {
      "type": "string",
      "description": "Route description"
    },
    "path": {
      "type": "string",
      "description": "URL path (e.g., /api/users)",
      "pattern": "^/"
    },
    "method": {
      "type": "string",
      "enum": ["GET", "POST", "PUT", "DELETE", "PATCH"],
      "description": "HTTP method"
    },
    "route_type": {
      "type": "string",
      "enum": ["http", "web_socket"],
      "default": "http",
      "description": "Route type (use 'web_socket' for WebSocket routes)"
    },
    "logic": {
      "type": "array",
      "description": "Array of logic operations to execute",
      "items": {
        "$ref": "#/definitions/LogicOperation"
      }
    },
    "parameters": {
      "type": "array",
      "description": "Route parameters",
      "items": {
        "$ref": "#/definitions/RouteParameter"
      }
    },
    "enabled": {
      "type": "boolean",
      "default": true,
      "description": "Whether the route is enabled"
    },
    "version": {
      "type": "string",
      "default": "1.0.0",
      "description": "Route version"
    },
    "auth_required": {
      "type": "boolean",
      "default": false,
      "description": "Whether authentication is required"
    },
    "rate_limit": {
      "type": "number",
      "description": "Rate limit (requests per minute)"
    },
    "response_schema": {
      "type": "object",
      "description": "Expected response schema"
    },
    "ws_hooks": {
      "$ref": "#/definitions/WebSocketHooks"
    }
  },
  "definitions": {
    "LogicOperation": {
      "oneOf": [
        { "$ref": "#/definitions/ReturnOp" },
        { "$ref": "#/definitions/CommentOp" },
        { "$ref": "#/definitions/SetOp" },
        { "$ref": "#/definitions/GetOp" },
        { "$ref": "#/definitions/IfOp" },
        { "$ref": "#/definitions/LoopOp" },
        { "$ref": "#/definitions/WhileOp" },
        { "$ref": "#/definitions/SwitchOp" },
        { "$ref": "#/definitions/BreakOp" },
        { "$ref": "#/definitions/ContinueOp" },
        { "$ref": "#/definitions/TryOp" },
        { "$ref": "#/definitions/ThrowOp" },
        { "$ref": "#/definitions/HttpRequestOp" },
        { "$ref": "#/definitions/QueryDbOp" },
        { "$ref": "#/definitions/SqlOp" },
        { "$ref": "#/definitions/RedisOp" },
        { "$ref": "#/definitions/WsOp" },
        { "$ref": "#/definitions/MathOp" },
        { "$ref": "#/definitions/StringOp" },
        { "$ref": "#/definitions/DateOp" },
        { "$ref": "#/definitions/JsonOp" },
        { "$ref": "#/definitions/LogOp" },
        { "$ref": "#/definitions/SleepOp" },
        { "$ref": "#/definitions/MapOp" },
        { "$ref": "#/definitions/FilterOp" },
        { "$ref": "#/definitions/AggregateOp" },
        { "$ref": "#/definitions/ParallelOp" },
        { "$ref": "#/definitions/DefineFunctionOp" },
        { "$ref": "#/definitions/CallFunctionOp" },
        { "$ref": "#/definitions/ExecuteScriptOp" }
      ]
    },
    "ReturnOp": {
      "type": "object",
      "required": ["return"],
      "additionalProperties": false,
      "properties": {
        "return": {
          "type": "object",
          "required": ["value"],
          "properties": {
            "value": {
              "description": "Value to return (supports templates like {{var}})"
            },
            "status": {
              "type": "integer",
              "description": "HTTP status code (default: 200)",
              "minimum": 100,
              "maximum": 599
            },
            "headers": {
              "type": "object",
              "description": "Custom HTTP headers",
              "additionalProperties": { "type": "string" }
            },
            "raw": {
              "type": "boolean",
              "description": "If true, send response as raw content (not wrapped in JSON)"
            }
          }
        }
      }
    },
    "CommentOp": {
      "type": "object",
      "required": ["comment"],
      "additionalProperties": false,
      "properties": {
        "comment": {
          "type": "string",
          "description": "Comment for documentation (no-op, 0 CPU cost)"
        }
      }
    },
    "SetOp": {
      "type": "object",
      "required": ["set"],
      "additionalProperties": false,
      "properties": {
        "set": {
          "type": "object",
          "required": ["var", "value"],
          "properties": {
            "var": {
              "type": "string",
              "description": "Variable name"
            },
            "value": {
              "description": "Value to set (supports templates)"
            }
          }
        }
      }
    },
    "GetOp": {
      "type": "object",
      "required": ["get"],
      "additionalProperties": false,
      "properties": {
        "get": {
          "type": "object",
          "required": ["var"],
          "properties": {
            "var": {
              "type": "string",
              "description": "Variable name to get"
            }
          }
        }
      }
    },
    "IfOp": {
      "type": "object",
      "required": ["if"],
      "additionalProperties": false,
      "properties": {
        "if": {
          "type": "object",
          "required": ["condition", "then"],
          "properties": {
            "condition": {
              "type": "string",
              "description": "Condition expression (e.g., age >= 18)"
            },
            "then": {
              "type": "array",
              "description": "Operations to execute if condition is true",
              "items": { "$ref": "#/definitions/LogicOperation" }
            },
            "otherwise": {
              "type": "array",
              "description": "Operations to execute if condition is false",
              "items": { "$ref": "#/definitions/LogicOperation" }
            }
          }
        }
      }
    },
    "LoopOp": {
      "type": "object",
      "required": ["loop"],
      "additionalProperties": false,
      "properties": {
        "loop": {
          "type": "object",
          "required": ["collection", "var", "body"],
          "properties": {
            "collection": {
              "type": "string",
              "description": "Collection to iterate over (supports templates)"
            },
            "var": {
              "type": "string",
              "description": "Variable name for each item"
            },
            "body": {
              "type": "array",
              "description": "Operations to execute for each item",
              "items": { "$ref": "#/definitions/LogicOperation" }
            }
          }
        }
      }
    },
    "WhileOp": {
      "type": "object",
      "required": ["while"],
      "additionalProperties": false,
      "properties": {
        "while": {
          "type": "object",
          "required": ["condition", "body"],
          "properties": {
            "condition": {
              "type": "string",
              "description": "Loop condition"
            },
            "body": {
              "type": "array",
              "description": "Operations to execute while condition is true",
              "items": { "$ref": "#/definitions/LogicOperation" }
            },
            "max_iterations": {
              "type": "integer",
              "description": "Maximum iterations to prevent infinite loops",
              "minimum": 1
            }
          }
        }
      }
    },
    "SwitchOp": {
      "type": "object",
      "required": ["switch"],
      "additionalProperties": false,
      "properties": {
        "switch": {
          "type": "object",
          "required": ["value", "cases"],
          "properties": {
            "value": {
              "type": "string",
              "description": "Value to match (supports templates)"
            },
            "cases": {
              "type": "array",
              "items": {
                "type": "object",
                "required": ["value", "operations"],
                "properties": {
                  "value": {},
                  "operations": {
                    "type": "array",
                    "items": { "$ref": "#/definitions/LogicOperation" }
                  }
                }
              }
            },
            "default": {
              "type": "array",
              "items": { "$ref": "#/definitions/LogicOperation" }
            }
          }
        }
      }
    },
    "BreakOp": {
      "type": "object",
      "required": ["break"],
      "additionalProperties": false,
      "properties": {
        "break": {
          "type": "object",
          "description": "Break out of loop"
        }
      }
    },
    "ContinueOp": {
      "type": "object",
      "required": ["continue"],
      "additionalProperties": false,
      "properties": {
        "continue": {
          "type": "object",
          "description": "Continue to next loop iteration"
        }
      }
    },
    "TryOp": {
      "type": "object",
      "required": ["try"],
      "additionalProperties": false,
      "properties": {
        "try": {
          "type": "object",
          "required": ["body", "catch"],
          "properties": {
            "body": {
              "type": "array",
              "items": { "$ref": "#/definitions/LogicOperation" }
            },
            "catch": {
              "type": "array",
              "items": { "$ref": "#/definitions/LogicOperation" }
            },
            "finally": {
              "type": "array",
              "items": { "$ref": "#/definitions/LogicOperation" }
            }
          }
        }
      }
    },
    "ThrowOp": {
      "type": "object",
      "required": ["throw"],
      "additionalProperties": false,
      "properties": {
        "throw": {
          "type": "object",
          "required": ["message"],
          "properties": {
            "message": { "type": "string" },
            "code": { "type": "string" }
          }
        }
      }
    },
    "HttpRequestOp": {
      "type": "object",
      "required": ["http_request"],
      "additionalProperties": false,
      "properties": {
        "http_request": {
          "type": "object",
          "required": ["url", "method"],
          "properties": {
            "url": {
              "type": "string",
              "description": "Request URL (supports templates)"
            },
            "method": {
              "type": "string",
              "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"],
              "description": "HTTP method"
            },
            "body": {
              "description": "Request body"
            },
            "headers": {
              "type": "object",
              "additionalProperties": { "type": "string" }
            },
            "timeout_ms": {
              "type": "integer",
              "minimum": 1
            }
          }
        }
      }
    },
    "QueryDbOp": {
      "type": "object",
      "required": ["query_db"],
      "additionalProperties": false,
      "properties": {
        "query_db": {
          "type": "object",
          "required": ["query"],
          "properties": {
            "query": {
              "type": "string",
              "description": "SQL query"
            },
            "params": {
              "type": "array",
              "description": "Query parameters"
            }
          }
        }
      }
    },
    "SqlOp": {
      "type": "object",
      "required": ["sql_op"],
      "additionalProperties": false,
      "properties": {
        "sql_op": {
          "type": "object",
          "required": ["query", "output_var"],
          "properties": {
            "query": {
              "type": "string",
              "description": "SQL query"
            },
            "args": {
              "type": "array",
              "description": "Query arguments (supports templates)"
            },
            "output_var": {
              "type": "string",
              "description": "Variable to store result"
            }
          }
        }
      }
    },
    "RedisOp": {
      "type": "object",
      "required": ["redis_op"],
      "additionalProperties": false,
      "properties": {
        "redis_op": {
          "type": "object",
          "required": ["command", "key"],
          "properties": {
            "command": {
              "type": "string",
              "enum": ["GET", "SET", "DEL", "EXPIRE", "INCR", "DECR"],
              "description": "Redis command"
            },
            "key": {
              "type": "string",
              "description": "Redis key (supports templates)"
            },
            "value": {
              "type": "string",
              "description": "Value for SET command"
            },
            "ttl_seconds": {
              "type": "integer",
              "minimum": 1
            },
            "output_var": {
              "type": "string"
            }
          }
        }
      }
    },
    "WsOp": {
      "type": "object",
      "required": ["ws_op"],
      "additionalProperties": false,
      "properties": {
        "ws_op": {
          "type": "object",
          "required": ["command", "message"],
          "properties": {
            "command": {
              "type": "string",
              "enum": ["send", "broadcast"]
            },
            "message": { "type": "string" },
            "channel": { "type": "string" }
          }
        }
      }
    },
    "MathOp": {
      "type": "object",
      "required": ["math_op"],
      "additionalProperties": false,
      "properties": {
        "math_op": {
          "type": "object",
          "required": ["operation", "args"],
          "properties": {
            "operation": {
              "type": "string",
              "enum": ["add", "subtract", "multiply", "divide", "modulo", "power", "sqrt", "abs", "ceil", "floor", "round", "min", "max", "sum", "avg"]
            },
            "args": {
              "type": "array",
              "description": "Arguments for operation"
            }
          }
        }
      }
    },
    "StringOp": {
      "type": "object",
      "required": ["string_op"],
      "additionalProperties": false,
      "properties": {
        "string_op": {
          "type": "object",
          "required": ["operation", "input"],
          "properties": {
            "operation": {
              "type": "string",
              "enum": ["upper", "lower", "trim", "split", "join", "replace", "substring", "concat", "length", "regex_match", "regex_replace"]
            },
            "input": { "type": "string" },
            "args": { "type": "array" }
          }
        }
      }
    },
    "DateOp": {
      "type": "object",
      "required": ["date_op"],
      "additionalProperties": false,
      "properties": {
        "date_op": {
          "type": "object",
          "required": ["operation"],
          "properties": {
            "operation": {
              "type": "string",
              "enum": ["now", "parse", "format", "add", "subtract", "diff", "is_before", "is_after"]
            },
            "args": { "type": "array" }
          }
        }
      }
    },
    "JsonOp": {
      "type": "object",
      "required": ["json_op"],
      "additionalProperties": false,
      "properties": {
        "json_op": {
          "type": "object",
          "required": ["operation", "input"],
          "properties": {
            "operation": {
              "type": "string",
              "enum": ["parse", "stringify", "merge", "get_path", "set_path"]
            },
            "input": { "type": "string" },
            "args": { "type": "array" }
          }
        }
      }
    },
    "LogOp": {
      "type": "object",
      "required": ["log"],
      "additionalProperties": false,
      "properties": {
        "log": {
          "type": "object",
          "required": ["message"],
          "properties": {
            "level": {
              "type": "string",
              "enum": ["debug", "info", "warn", "error"],
              "default": "info"
            },
            "message": { "type": "string" }
          }
        }
      }
    },
    "SleepOp": {
      "type": "object",
      "required": ["sleep"],
      "additionalProperties": false,
      "properties": {
        "sleep": {
          "type": "object",
          "required": ["duration_ms"],
          "properties": {
            "duration_ms": {
              "type": "integer",
              "minimum": 1
            }
          }
        }
      }
    },
    "MapOp": {
      "type": "object",
      "required": ["map"],
      "additionalProperties": false,
      "properties": {
        "map": {
          "type": "object",
          "required": ["input", "transform"],
          "properties": {
            "input": { "type": "string" },
            "transform": { "type": "string" }
          }
        }
      }
    },
    "FilterOp": {
      "type": "object",
      "required": ["filter"],
      "additionalProperties": false,
      "properties": {
        "filter": {
          "type": "object",
          "required": ["input", "condition"],
          "properties": {
            "input": { "type": "string" },
            "condition": { "type": "string" }
          }
        }
      }
    },
    "AggregateOp": {
      "type": "object",
      "required": ["aggregate"],
      "additionalProperties": false,
      "properties": {
        "aggregate": {
          "type": "object",
          "required": ["input", "operation"],
          "properties": {
            "input": { "type": "string" },
            "operation": {
              "type": "string",
              "enum": ["sum", "count", "avg", "min", "max"]
            }
          }
        }
      }
    },
    "ParallelOp": {
      "type": "object",
      "required": ["parallel"],
      "additionalProperties": false,
      "properties": {
        "parallel": {
          "type": "object",
          "required": ["tasks"],
          "properties": {
            "tasks": {
              "type": "array",
              "items": {
                "type": "array",
                "items": { "$ref": "#/definitions/LogicOperation" }
              }
            },
            "max_concurrent": { "type": "integer", "minimum": 1 }
          }
        }
      }
    },
    "DefineFunctionOp": {
      "type": "object",
      "required": ["define_function"],
      "additionalProperties": false,
      "properties": {
        "define_function": {
          "type": "object",
          "required": ["name", "params", "body"],
          "properties": {
            "name": { "type": "string" },
            "params": {
              "type": "array",
              "items": { "type": "string" }
            },
            "body": {
              "type": "array",
              "items": { "$ref": "#/definitions/LogicOperation" }
            }
          }
        }
      }
    },
    "CallFunctionOp": {
      "type": "object",
      "required": ["call_function"],
      "additionalProperties": false,
      "properties": {
        "call_function": {
          "type": "object",
          "required": ["name", "output_var"],
          "properties": {
            "name": { "type": "string" },
            "args": { "type": "array" },
            "output_var": { "type": "string" }
          }
        }
      }
    },
    "ExecuteScriptOp": {
      "type": "object",
      "required": ["execute_script"],
      "additionalProperties": false,
      "properties": {
        "execute_script": {
          "type": "object",
          "required": ["language", "code"],
          "properties": {
            "language": {
              "type": "string",
              "enum": ["python", "javascript", "lua"]
            },
            "code": { "type": "string" }
          }
        }
      }
    },
    "RouteParameter": {
      "type": "object",
      "required": ["name", "param_type", "data_type", "required"],
      "properties": {
        "name": { "type": "string" },
        "param_type": {
          "type": "string",
          "enum": ["path", "query", "body", "header"]
        },
        "data_type": {
          "type": "string",
          "enum": ["string", "number", "boolean", "object", "array"]
        },
        "required": { "type": "boolean" },
        "default_value": {},
        "validation": { "type": "string" }
      }
    },
    "WebSocketHooks": {
      "type": "object",
      "properties": {
        "on_connect": {
          "type": "array",
          "items": { "$ref": "#/definitions/LogicOperation" }
        },
        "on_message": {
          "type": "array",
          "items": { "$ref": "#/definitions/LogicOperation" }
        },
        "on_disconnect": {
          "type": "array",
          "items": { "$ref": "#/definitions/LogicOperation" }
        }
      }
    }
  }
};
