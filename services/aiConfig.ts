/**
 * AI Configuration Service
 * Manages AI provider settings stored in localStorage
 */

export interface AIConfig {
  apiKey: string;
  model: string;
  endpoint: string;
  enabled: boolean;
}

const STORAGE_KEY = 'worpen_ai_config';

const DEFAULT_CONFIG: AIConfig = {
  apiKey: '',
  model: 'gpt-4o',
  endpoint: 'https://models.inference.ai.azure.com',
  enabled: false,
};

/**
 * Get AI configuration from localStorage
 */
export function getAIConfig(): AIConfig {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      return { ...DEFAULT_CONFIG, ...parsed };
    }
  } catch (error) {
    console.error('Failed to load AI config:', error);
  }
  return DEFAULT_CONFIG;
}

/**
 * Save AI configuration to localStorage
 */
export function saveAIConfig(config: Partial<AIConfig>): void {
  try {
    const current = getAIConfig();
    const updated = { ...current, ...config };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
  } catch (error) {
    console.error('Failed to save AI config:', error);
    throw new Error('Failed to save configuration');
  }
}

/**
 * Clear AI configuration
 */
export function clearAIConfig(): void {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch (error) {
    console.error('Failed to clear AI config:', error);
  }
}

/**
 * Check if AI is properly configured
 */
export function isAIConfigured(): boolean {
  const config = getAIConfig();
  return !!(config.enabled && config.apiKey && config.model && config.endpoint);
}

/**
 * Test AI connection by making a simple API call
 */
export async function testAIConnection(config: AIConfig): Promise<{ success: boolean; message: string }> {
  if (!config.apiKey) {
    return { success: false, message: 'API Key is required' };
  }

  if (!config.endpoint) {
    return { success: false, message: 'API Endpoint is required' };
  }

  if (!config.model) {
    return { success: false, message: 'Model name is required' };
  }

  try {
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
          {
            role: 'user',
            content: 'Say OK if you can hear me.',
          },
        ],
        max_tokens: 10,
        temperature: 0.1,
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      let errorMessage = `HTTP ${response.status}: ${response.statusText}`;
      
      try {
        const errorJson = JSON.parse(errorText);
        if (errorJson.error?.message) {
          errorMessage = errorJson.error.message;
        } else if (errorJson.message) {
          errorMessage = errorJson.message;
        }
      } catch {
        // Use default error message
      }

      return { success: false, message: errorMessage };
    }

    const data = await response.json();
    
    if (data.choices && data.choices.length > 0) {
      return { 
        success: true, 
        message: 'Connection verified! AI is responding correctly.' 
      };
    }

    return { 
      success: false, 
      message: 'Unexpected response format from AI provider' 
    };
  } catch (error: any) {
    return { 
      success: false, 
      message: error.message || 'Network error. Please check your connection and endpoint URL.' 
    };
  }
}

/**
 * Available AI Models (suggestions for dropdown)
 */
export const AVAILABLE_MODELS = [
  { id: 'gpt-4o', name: 'GPT-4o (Recommended)', provider: 'OpenAI/GitHub' },
  { id: 'gpt-4o-mini', name: 'GPT-4o Mini', provider: 'OpenAI/GitHub' },
  { id: 'gpt-4', name: 'GPT-4', provider: 'OpenAI' },
  { id: 'gpt-3.5-turbo', name: 'GPT-3.5 Turbo', provider: 'OpenAI' },
  { id: 'Phi-3-medium-128k-instruct', name: 'Phi-3 Medium', provider: 'Microsoft' },
  { id: 'Phi-3-small-128k-instruct', name: 'Phi-3 Small', provider: 'Microsoft' },
];

/**
 * Popular AI Endpoints
 */
export const POPULAR_ENDPOINTS = [
  { url: 'https://models.inference.ai.azure.com', name: 'GitHub Models (Free)' },
  { url: 'https://api.openai.com/v1', name: 'OpenAI (Paid)' },
];
