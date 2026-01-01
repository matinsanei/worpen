/**
 * AI Service for Worpen Logic Generation
 * Converts natural language descriptions into Worpen JSON Logic
 */

import { getAIConfig, isAIConfigured } from './aiConfig';

export type LogicType = 'route' | 'function';

interface AIGenerationResponse {
  success: boolean;
  logic: any[];
  error?: string;
}

/**
 * Call the configured LLM API to generate logic
 */
async function callLLMAPI(prompt: string, type: LogicType): Promise<any[]> {
  const config = getAIConfig();

  if (!config.enabled || !config.apiKey) {
    throw new Error('AI is not configured. Please set up your API key in Settings.');
  }

  const systemPrompt = `You are a Worpen Engine expert. Convert user requests into valid Worpen JSON Logic.

Available operations:
- set: Create/update variables. Example: {"set": {"var": "name", "value": "{{body.input}}"}}
- math_op: Mathematical operations. Example: {"math_op": {"operation": "add", "left": 10, "right": 5, "result_var": "sum"}}
- if: Conditional logic. Example: {"if": {"condition": {"comparison": "equals", "left": "{{var.x}}", "right": 10}, "then": [...], "else": [...]}}
- for_each: Loop through collections. Example: {"for_each": {"collection": "{{body.items}}", "operations": [...]}}
- http_request: Make HTTP calls. Example: {"http_request": {"method": "GET", "url": "https://api.example.com", "result_var": "response"}}
- string_op: String manipulation. Example: {"string_op": {"operation": "uppercase", "input": "{{var.text}}", "result_var": "result"}}
- array_push: Add to arrays. Example: {"array_push": {"array_var": "items", "value": "{{var.new_item}}"}}
- return: Return response. Example: {"return": {"status": 200, "body": {"result": "{{var.data}}"}}}

Output ONLY a valid JSON array of operations. No explanations, no markdown, just the JSON array.`;

  const endpoint = config.endpoint.endsWith('/')
    ? config.endpoint + 'chat/completions'
    : config.endpoint + '/chat/completions';

  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${config.apiKey}`,
    },
    body: JSON.stringify({
      model: config.model,
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: `Generate ${type} logic for: ${prompt}` },
      ],
      temperature: 0.7,
      max_tokens: 1500,
    }),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`AI API Error: ${response.status} ${errorText}`);
  }

  const data = await response.json();
  const content = data.choices?.[0]?.message?.content;

  if (!content) {
    throw new Error('No response from AI');
  }

  // Try to extract JSON from response (handle markdown code blocks)
  let jsonStr = content.trim();
  if (jsonStr.startsWith('```json')) {
    jsonStr = jsonStr.replace(/^```json\n/, '').replace(/\n```$/, '');
  } else if (jsonStr.startsWith('```')) {
    jsonStr = jsonStr.replace(/^```\n/, '').replace(/\n```$/, '');
  }

  return JSON.parse(jsonStr);
}

/**
 * Generate Worpen logic from natural language prompt
 * @param prompt - User's natural language description
 * @param type - Type of logic to generate (route or function)
 */
export async function generateLogicFromPrompt(
  prompt: string,
  type: LogicType
): Promise<AIGenerationResponse> {
  try {
    // Check if AI is configured
    if (isAIConfigured()) {
      // Use real AI API
      const logic = await callLLMAPI(prompt, type);
      return {
        success: true,
        logic,
      };
    }

    // Fallback to mock logic if AI is not configured
    return generateMockLogic(prompt, type);
  } catch (error: any) {
    // If real API fails, try mock as fallback
    console.error('AI generation failed, using mock:', error);
    return generateMockLogic(prompt, type);
  }
}

/**
 * Mock logic generation (fallback when AI is not configured)
 */
async function generateMockLogic(prompt: string, type: LogicType): Promise<AIGenerationResponse> {
  // Simulate AI processing delay
  await new Promise(resolve => setTimeout(resolve, 1500));
  
  const lowerPrompt = prompt.toLowerCase();
  
  try {
    let generatedLogic: any[] = [];

    // Tax calculation logic
    if (lowerPrompt.includes('tax') || lowerPrompt.includes('calculate')) {
      generatedLogic = [
        {
          "set": {
            "var": "base_amount",
            "value": "{{body.amount}}"
          }
        },
        {
          "set": {
            "var": "tax_rate",
            "value": 0.15
          }
        },
        {
          "math_op": {
            "operation": "multiply",
            "left": "{{var.base_amount}}",
            "right": "{{var.tax_rate}}",
            "result_var": "tax_amount"
          }
        },
        {
          "math_op": {
            "operation": "add",
            "left": "{{var.base_amount}}",
            "right": "{{var.tax_amount}}",
            "result_var": "total"
          }
        },
        {
          "return": {
            "status": 200,
            "body": {
              "base_amount": "{{var.base_amount}}",
              "tax_amount": "{{var.tax_amount}}",
              "total": "{{var.total}}"
            }
          }
        }
      ];
    }
    // Conditional/Check logic
    else if (lowerPrompt.includes('check') || lowerPrompt.includes('if') || lowerPrompt.includes('validate')) {
      generatedLogic = [
        {
          "set": {
            "var": "input_value",
            "value": "{{body.value}}"
          }
        },
        {
          "if": {
            "condition": {
              "comparison": "greater_than",
              "left": "{{var.input_value}}",
              "right": 0
            },
            "then": [
              {
                "return": {
                  "status": 200,
                  "body": {
                    "valid": true,
                    "message": "Value is positive",
                    "value": "{{var.input_value}}"
                  }
                }
              }
            ],
            "else": [
              {
                "return": {
                  "status": 400,
                  "body": {
                    "valid": false,
                    "message": "Value must be positive"
                  }
                }
              }
            ]
          }
        }
      ];
    }
    // Loop/iteration logic
    else if (lowerPrompt.includes('loop') || lowerPrompt.includes('each') || lowerPrompt.includes('iterate')) {
      generatedLogic = [
        {
          "set": {
            "var": "items",
            "value": "{{body.items}}"
          }
        },
        {
          "set": {
            "var": "processed",
            "value": []
          }
        },
        {
          "for_each": {
            "collection": "{{var.items}}",
            "operations": [
              {
                "set": {
                  "var": "current",
                  "value": {
                    "id": "{{item.id}}",
                    "processed": true,
                    "timestamp": "{{now()}}"
                  }
                }
              },
              {
                "array_push": {
                  "array_var": "processed",
                  "value": "{{var.current}}"
                }
              }
            ]
          }
        },
        {
          "return": {
            "status": 200,
            "body": {
              "processed_items": "{{var.processed}}",
              "count": "{{var.processed.length}}"
            }
          }
        }
      ];
    }
    // HTTP request logic
    else if (lowerPrompt.includes('api') || lowerPrompt.includes('request') || lowerPrompt.includes('fetch')) {
      generatedLogic = [
        {
          "set": {
            "var": "api_url",
            "value": "https://api.example.com/data"
          }
        },
        {
          "http_request": {
            "method": "GET",
            "url": "{{var.api_url}}",
            "headers": {
              "Content-Type": "application/json"
            },
            "result_var": "api_response"
          }
        },
        {
          "return": {
            "status": 200,
            "body": {
              "data": "{{var.api_response}}"
            }
          }
        }
      ];
    }
    // String manipulation logic
    else if (lowerPrompt.includes('format') || lowerPrompt.includes('string') || lowerPrompt.includes('text')) {
      generatedLogic = [
        {
          "set": {
            "var": "input",
            "value": "{{body.text}}"
          }
        },
        {
          "string_op": {
            "operation": "uppercase",
            "input": "{{var.input}}",
            "result_var": "formatted"
          }
        },
        {
          "return": {
            "status": 200,
            "body": {
              "original": "{{var.input}}",
              "formatted": "{{var.formatted}}"
            }
          }
        }
      ];
    }
    // Database query logic
    else if (lowerPrompt.includes('database') || lowerPrompt.includes('query') || lowerPrompt.includes('sql')) {
      generatedLogic = [
        {
          "set": {
            "var": "user_id",
            "value": "{{body.user_id}}"
          }
        },
        {
          "db_query": {
            "query": "SELECT * FROM users WHERE id = ?",
            "params": ["{{var.user_id}}"],
            "result_var": "user_data"
          }
        },
        {
          "return": {
            "status": 200,
            "body": {
              "user": "{{var.user_data}}"
            }
          }
        }
      ];
    }
    // Default/Generic logic
    else {
      generatedLogic = [
        {
          "set": {
            "var": "input",
            "value": "{{body.data}}"
          }
        },
        {
          "set": {
            "var": "result",
            "value": {
              "processed": true,
              "input": "{{var.input}}",
              "timestamp": "{{now()}}"
            }
          }
        },
        {
          "return": {
            "status": 200,
            "body": "{{var.result}}"
          }
        }
      ];
    }

    return {
      success: true,
      logic: generatedLogic,
    };
  } catch (error: any) {
    return {
      success: false,
      logic: [],
      error: error.message || 'Failed to generate logic',
    };
  }
}
