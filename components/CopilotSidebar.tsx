import React, { useState, useRef, useEffect } from 'react';
import { X, Send, Sparkles, Loader2, Code, CheckCircle, MessageSquare, AlertTriangle, Settings, Trash2, ChevronDown, Zap, Clock, Cpu, Copy, Edit2, RotateCw, Route, Cog } from 'lucide-react';
import { getAIConfig, isAIConfigured } from '../services/aiConfig';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  codeBlocks?: Array<{ language: string; code: string }>;
  timestamp: Date;
  model?: string;
}

interface ChatHistory {
  contextType: 'route' | 'function';
  messages: Message[];
  timestamp: number;
}

interface AIModel {
  id: string;
  name?: string;
  created?: number;
  owned_by?: string;
}

interface CopilotSidebarProps {
  isOpen: boolean;
  onClose: () => void;
  currentCode: string;
  codeLanguage: 'json' | 'yaml';
  contextType: 'route' | 'function';
  onApplyCode: (code: string) => void;
}

export const CopilotSidebar: React.FC<CopilotSidebarProps> = ({
  isOpen,
  onClose,
  currentCode,
  codeLanguage,
  contextType,
  onApplyCode,
}) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedModel, setSelectedModel] = useState<string>('');
  const [editingMessageId, setEditingMessageId] = useState<string | null>(null);
  const [editedContent, setEditedContent] = useState<string>('');
  const [showModelSelector, setShowModelSelector] = useState(false);
  const [chatHistories, setChatHistories] = useState<ChatHistory[]>([]);
  const [showHistory, setShowHistory] = useState(false);
  const [availableModels, setAvailableModels] = useState<AIModel[]>([]);
  const [loadingModels, setLoadingModels] = useState(false);
  const [currentSessionTimestamp, setCurrentSessionTimestamp] = useState<number>(Date.now());
  const [width, setWidth] = useState(() => {
    const saved = localStorage.getItem('worpen-copilot-width');
    return saved ? parseInt(saved) : 384; // default 384px (96 * 4)
  });
  const [isDragging, setIsDragging] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  // Fetch available models from API
  const fetchModels = async () => {
    setLoadingModels(true);
    try {
      const config = getAIConfig();
      const endpoint = config.endpoint.endsWith('/')
        ? config.endpoint + 'models'
        : config.endpoint + '/models';

      const response = await fetch(endpoint, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${config.apiKey}`,
        },
      });

      if (!response.ok) {
        throw new Error('Failed to fetch models');
      }

      const data = await response.json();
      console.log('API Response:', data);
      
      let models = [];
      if (data.data && Array.isArray(data.data)) {
        models = data.data;
      } else if (data.models && Array.isArray(data.models)) {
        models = data.models;
      } else if (Array.isArray(data)) {
        models = data;
      }
      
      // Filter chat models only
      const chatModels = models.filter((m: AIModel) => 
        m.id && (
          !m.id.includes('embedding') && 
          !m.id.includes('whisper') && 
          !m.id.includes('tts') &&
          !m.id.includes('dall-e') &&
          !m.id.includes('vision')
        )
      );

      console.log('Filtered models:', chatModels);
      setAvailableModels(chatModels);

      // Set default model if not set
      if (!selectedModel && chatModels.length > 0) {
        const defaultModel = chatModels.find((m: AIModel) => m.id.includes('gpt-4')) || chatModels[0];
        setSelectedModel(defaultModel.id);
      }
    } catch (error) {
      console.error('Failed to fetch models:', error);
      // Fallback to config model as fallback
      const config = getAIConfig();
      const fallbackModel = localStorage.getItem('worpen-copilot-model') || config.model || 'gpt-4o';
      setSelectedModel(fallbackModel);
      // Set a default model in the list
      setAvailableModels([{ id: fallbackModel, name: fallbackModel }]);
    } finally {
      setLoadingModels(false);
    }
  };

  // Load chat history from localStorage
  useEffect(() => {
    const savedHistories = localStorage.getItem('worpen-copilot-history');
    if (savedHistories) {
      try {
        const parsed = JSON.parse(savedHistories);
        // Convert timestamp strings back to Date objects
        const historiesWithDates = parsed.map((h: ChatHistory) => ({
          ...h,
          messages: h.messages.map((m: Message) => ({
            ...m,
            timestamp: new Date(m.timestamp),
          })),
        }));
        setChatHistories(historiesWithDates);
      } catch (error) {
        console.error('Failed to load chat history:', error);
      }
    }

    // Load selected model and fetch models from API
    const config = getAIConfig();
    const savedModel = localStorage.getItem('worpen-copilot-model');
    setSelectedModel(savedModel || config.model);
    
    if (isAIConfigured()) {
      fetchModels();
    }
  }, []);

  // Save chat history to localStorage
  useEffect(() => {
    if (messages.length > 0) {
      const currentHistory: ChatHistory = {
        contextType,
        messages,
        timestamp: currentSessionTimestamp,
      };

      const updatedHistories = [currentHistory, ...chatHistories.filter(h => h.timestamp !== currentSessionTimestamp)].slice(0, 10);
      setChatHistories(updatedHistories);
      localStorage.setItem('worpen-copilot-history', JSON.stringify(updatedHistories));
    }
  }, [messages, currentSessionTimestamp, contextType]);

  // Save selected model to localStorage
  useEffect(() => {
    if (selectedModel) {
      localStorage.setItem('worpen-copilot-model', selectedModel);
    }
  }, [selectedModel]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isOpen]);
  // Save width to localStorage
  useEffect(() => {
    localStorage.setItem('worpen-copilot-width', width.toString());
  }, [width]);

  // Resize handlers
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;
      
      const newWidth = window.innerWidth - e.clientX;
      const clampedWidth = Math.min(Math.max(newWidth, 320), 800); // min 320px, max 800px
      setWidth(clampedWidth);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = 'col-resize';
      document.body.style.userSelect = 'none';
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
  }, [isDragging]);
  const extractCodeBlocks = (content: string): Array<{ language: string; code: string }> => {
    const codeBlockRegex = /```(\w+)?\n([\s\S]*?)```/g;
    const blocks: Array<{ language: string; code: string }> = [];
    let match;

    while ((match = codeBlockRegex.exec(content)) !== null) {
      blocks.push({
        language: match[1] || 'text',
        code: match[2].trim(),
      });
    }

    return blocks;
  };

  // Extract model name from full path (e.g., "azureml://.../gpt-4o/..." -> "gpt-4o")
  const getModelName = (modelId: string): string => {
    // Check if it's a full path
    if (modelId.includes('/models/')) {
      const match = modelId.match(/\/models\/([^\/]+)/);
      if (match) return match[1];
    }
    // Otherwise return as-is
    return modelId;
  };

  const sendMessage = async () => {
    if (!input.trim() || loading) return;

    if (!isAIConfigured()) {
      setError('AI is not configured. Please set up your API key in Settings.');
      return;
    }

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: input.trim(),
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setLoading(true);
    setError(null);
    
    // Reset textarea height with smooth transition
    if (inputRef.current) {
      inputRef.current.style.height = '40px';
    }

    try {
      const config = getAIConfig();
      const endpoint = config.endpoint.endsWith('/')
        ? config.endpoint + 'chat/completions'
        : config.endpoint + '/chat/completions';

      // Build context-aware system prompt
      const systemPrompt = `You are Worpen Copilot, an expert backend engineer specializing in Worpen Engine logic.

The user is building a ${contextType === 'route' ? 'Dynamic Route' : 'Global Function'} using ${codeLanguage.toUpperCase()}.

Current ${contextType} code:
\`\`\`${codeLanguage}
${currentCode}
\`\`\`

Available Worpen operations:
- set: Create/update variables. Example: {"set": {"var": "name", "value": "{{body.input}}"}}
- math_op: Math operations (add, subtract, multiply, divide, modulo, power)
- if: Conditional logic with comparison operators (equals, not_equals, greater_than, less_than, etc.)
- for_each: Loop through collections
- http_request: Make HTTP calls (GET, POST, PUT, DELETE)
- string_op: String manipulation (uppercase, lowercase, trim, split, join, replace, substring)
- array_push, array_pop: Array operations
- query_db: Database queries (PostgreSQL)
- return: Return HTTP response

When providing code:
- Output valid ${codeLanguage.toUpperCase()} format
- Use triple backticks with language identifier
- Be concise but complete
- Explain your changes briefly

Help the user debug, optimize, or generate new logic. Be friendly and professional.`;

      // Extract clean model name from potentially full path
      const modelToUse = getModelName(selectedModel || config.model);

      const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${config.apiKey}`,
        },
        body: JSON.stringify({
          model: modelToUse,
          messages: [
            { role: 'system', content: systemPrompt },
            ...messages.map((msg) => ({
              role: msg.role,
              content: msg.content,
            })),
            { role: 'user', content: userMessage.content },
          ],
          temperature: 0.7,
          max_tokens: 2000,
        }),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`AI API Error: ${response.status} - ${errorText}`);
      }

      const data = await response.json();
      const assistantContent = data.choices?.[0]?.message?.content;

      if (!assistantContent) {
        throw new Error('No response from AI');
      }

      const codeBlocks = extractCodeBlocks(assistantContent);

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: assistantContent,
        codeBlocks,
        timestamp: new Date(),
        model: modelToUse,
      };

      setMessages((prev) => [...prev, assistantMessage]);
    } catch (err: any) {
      setError(err.message || 'Failed to get response from AI');
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const handleApplyCode = (code: string) => {
    onApplyCode(code);
  };

  const clearChat = () => {
    setMessages([]);
    setError(null);
    setCurrentSessionTimestamp(Date.now()); // Start new session
  };

  const loadHistory = (history: ChatHistory) => {
    setMessages(history.messages);
    setCurrentSessionTimestamp(history.timestamp); // Use existing session timestamp
    setShowHistory(false);
    setError(null);
  };

  const deleteHistory = (timestamp: number) => {
    const updatedHistories = chatHistories.filter(h => h.timestamp !== timestamp);
    setChatHistories(updatedHistories);
    localStorage.setItem('worpen-copilot-history', JSON.stringify(updatedHistories));
  };

  const deleteMessage = (messageId: string) => {
    setMessages(prev => prev.filter(msg => msg.id !== messageId));
  };

  const clearAllHistory = () => {
    setChatHistories([]);
    localStorage.removeItem('worpen-copilot-history');
    setShowHistory(false);
  };

  const copyMessageContent = (content: string) => {
    navigator.clipboard.writeText(content);
  };

  const startEditingMessage = (message: Message) => {
    setEditingMessageId(message.id);
    setEditedContent(message.content);
  };

  const saveEditedMessage = () => {
    if (!editingMessageId || !editedContent.trim()) return;
    
    setMessages(prev => prev.map(msg => 
      msg.id === editingMessageId 
        ? { ...msg, content: editedContent.trim() }
        : msg
    ));
    setEditingMessageId(null);
    setEditedContent('');
  };

  const cancelEditing = () => {
    setEditingMessageId(null);
    setEditedContent('');
  };

  const reRunMessage = async (messageContent: string) => {
    setInput(messageContent);
    // Auto-send the message
    setTimeout(() => {
      const userMessage: Message = {
        id: Date.now().toString(),
        role: 'user',
        content: messageContent.trim(),
        timestamp: new Date(),
      };

      setMessages((prev) => [...prev, userMessage]);
      setInput('');
      setLoading(true);
      setError(null);

      // Call the AI with the same content
      (async () => {
        try {
          const config = getAIConfig();
          const endpoint = config.endpoint.endsWith('/')
            ? config.endpoint + 'chat/completions'
            : config.endpoint + '/chat/completions';

          const systemPrompt = `You are Worpen Copilot, an expert backend engineer specializing in Worpen Engine logic.

The user is building a ${contextType === 'route' ? 'Dynamic Route' : 'Global Function'} using ${codeLanguage.toUpperCase()}.

Current ${contextType} code:
\`\`\`${codeLanguage}
${currentCode}
\`\`\`

Available Worpen operations:
- set: Create/update variables. Example: {"set": {"var": "name", "value": "{{body.input}}"}}
- math_op: Math operations (add, subtract, multiply, divide, modulo, power)
- if: Conditional logic with comparison operators (equals, not_equals, greater_than, less_than, etc.)
- for_each: Loop through collections
- http_request: Make HTTP calls (GET, POST, PUT, DELETE)
- string_op: String manipulation (uppercase, lowercase, trim, split, join, replace, substring)
- array_push, array_pop: Array operations
- query_db: Database queries (PostgreSQL)
- return: Return HTTP response

When providing code:
- Output valid ${codeLanguage.toUpperCase()} format
- Use triple backticks with language identifier
- Be concise but complete
- Explain your changes briefly

Help the user debug, optimize, or generate new logic. Be friendly and professional.`;

          const modelToUse = getModelName(selectedModel || config.model);

          const response = await fetch(endpoint, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'Authorization': `Bearer ${config.apiKey}`,
            },
            body: JSON.stringify({
              model: modelToUse,
              messages: [
                { role: 'system', content: systemPrompt },
                ...messages.map((msg) => ({
                  role: msg.role,
                  content: msg.content,
                })),
                { role: 'user', content: userMessage.content },
              ],
              temperature: 0.7,
              max_tokens: 2000,
            }),
          });

          if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`AI API Error: ${response.status} - ${errorText}`);
          }

          const data = await response.json();
          const assistantContent = data.choices?.[0]?.message?.content;

          if (!assistantContent) {
            throw new Error('No response from AI');
          }

          const codeBlocks = extractCodeBlocks(assistantContent);

          const assistantMessage: Message = {
            id: (Date.now() + 1).toString(),
            role: 'assistant',
            content: assistantContent,
            codeBlocks,
            timestamp: new Date(),
            model: modelToUse,
          };

          setMessages((prev) => [...prev, assistantMessage]);
        } catch (err: any) {
          setError(err.message || 'Failed to get response from AI');
        } finally {
          setLoading(false);
        }
      })();
    }, 100);
  };

  if (!isOpen) return null;

  const getModelDisplayName = (modelId: string) => {
    const model = availableModels.find(m => m.id === modelId);
    return model?.name || modelId;
  };

  return (
    <div className="h-full border-l border-[#3d3f43] bg-gradient-to-b from-[#1e1f22] to-[#252629] flex shadow-2xl relative" style={{ width: `${width}px` }}>
      {/* Resizer */}
      <div
        onMouseDown={handleMouseDown}
        className="absolute left-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-[#3574f0]/50 transition-colors z-50 group"
      >
        <div className="absolute left-0 top-0 bottom-0 w-1 bg-[#3d3f43] group-hover:bg-[#3574f0] transition-colors"></div>
      </div>
      
      {/* Content */}
      <div className="flex-1 flex flex-col overflow-hidden">{/* Header */}
      <div className="relative px-3 py-2.5 border-b border-[#3d3f43] bg-[#1e1f22]">
        <div className="flex items-center justify-between">
          {/* Left: Logo and Title */}
          <div className="flex items-center gap-2">
            <div className="w-7 h-7 rounded-lg bg-[#3574f0] flex items-center justify-center shadow-lg">
              <MessageSquare size={16} className="text-white" strokeWidth={2.5} />
            </div>
            <div>
              <h3 className="text-xs font-bold text-white">Worpen Copilot</h3>
              <p className="text-[9px] text-[#8b8d91]">AI-Powered Assistant</p>
            </div>
          </div>

          {/* Right: Model Selector + Action Buttons */}
          <div className="flex items-center gap-1">
            {/* Model Selector Button */}
            <div className="relative">
              <button
                onClick={() => setShowModelSelector(!showModelSelector)}
                disabled={loadingModels}
                className="flex items-center gap-1.5 px-2 py-1.5 bg-[#2d2f33] hover:bg-[#3d3f43] border border-[#3d3f43] rounded-lg transition-all"
                title="Select AI Model"
              >
                <Cpu size={13} className="text-[#3574f0]" />
                <ChevronDown size={11} className={`text-[#8b8d91] transition-transform ${showModelSelector ? 'rotate-180' : ''}`} />
              </button>

              {/* Model Selector Dropdown */}
              {showModelSelector && (
                <div className="absolute top-full right-0 mt-2 w-64 bg-[#252629] border border-[#3d3f43] rounded-lg shadow-2xl z-[100] max-h-64 overflow-y-auto custom-scrollbar">
                  {loadingModels ? (
                    <div className="p-3 text-center text-[#8b8d91] flex items-center justify-center gap-2">
                      <Loader2 size={12} className="animate-spin" />
                      <span className="text-xs">Loading models...</span>
                    </div>
                  ) : availableModels.length === 0 ? (
                    <div className="p-3 text-center">
                      <p className="text-xs text-[#8b8d91] mb-2">No models found</p>
                      <button
                        onClick={fetchModels}
                        className="text-xs text-[#3574f0] hover:text-[#4a8cf7] transition-colors"
                      >
                        Retry
                      </button>
                    </div>
                  ) : (
                    availableModels.map((model) => (
                      <button
                        key={model.id}
                        onClick={() => {
                          setSelectedModel(model.id);
                          setShowModelSelector(false);
                        }}
                        className={`w-full px-3 py-2 text-left hover:bg-[#2d2f33] transition-all border-b border-[#3d3f43]/30 last:border-b-0 ${
                          selectedModel === model.id ? 'bg-[#3574f0]/10' : ''
                        }`}
                      >
                        <div className="flex items-center justify-between">
                          <div>
                            <div className={`text-xs font-bold ${selectedModel === model.id ? 'text-[#3574f0]' : 'text-white'}`}>
                              {model.name || model.id}
                            </div>
                            {model.owned_by && (
                              <div className="text-[9px] text-[#8b8d91] mt-0.5">{model.owned_by}</div>
                            )}
                          </div>
                          {selectedModel === model.id && (
                            <CheckCircle size={12} className="text-[#3574f0]" />
                          )}
                        </div>
                      </button>
                    ))
                  )}
                </div>
              )}
            </div>

            {/* History Button */}
            <button
              onClick={() => setShowHistory(!showHistory)}
              className={`p-1.5 rounded-lg transition-all ${
                showHistory 
                  ? 'bg-[#3574f0]/20 text-[#3574f0]' 
                  : 'text-[#8b8d91] hover:bg-[#2d2f33] hover:text-white'
              }`}
              title="Chat history"
            >
              <Clock size={15} />
            </button>

            {/* Clear Button */}
            {messages.length > 0 && (
              <button
                onClick={clearChat}
                className="p-1.5 text-[#8b8d91] hover:text-[#e06c75] hover:bg-[#2d2f33] rounded-lg transition-all"
                title="Clear chat"
              >
                <Trash2 size={15} />
              </button>
            )}

            {/* Close Button */}
            <button
              onClick={onClose}
              className="p-1.5 text-[#8b8d91] hover:text-white hover:bg-[#2d2f33] rounded-lg transition-all"
            >
              <X size={15} />
            </button>
          </div>
        </div>
      </div>

      {/* Chat History Panel */}
      {showHistory && (
        <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-b from-[#1e1f22] to-[#252629] z-40 flex flex-col backdrop-blur-xl">
          <div className="flex items-center justify-between p-4 border-b border-[#3d3f43]/50">
            <div className="flex items-center gap-2">
              <Clock size={18} className="text-[#3574f0]" />
              <h4 className="text-sm font-bold text-white">Chat History</h4>
            </div>
            <div className="flex gap-2">
              {chatHistories.length > 0 && (
                <button
                  onClick={clearAllHistory}
                  className="text-[11px] px-3 py-1.5 bg-[#e06c75]/10 text-[#e06c75] hover:bg-[#e06c75]/20 rounded-lg transition-all font-medium"
                >
                  Clear All
                </button>
              )}
              <button
                onClick={() => setShowHistory(false)}
                className="p-1.5 text-[#8b8d91] hover:text-white hover:bg-[#2d2f33] rounded-lg transition-all"
              >
                <X size={16} />
              </button>
            </div>
          </div>
          <div className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-3">
            {chatHistories.length === 0 ? (
              <div className="h-full flex flex-col items-center justify-center text-center text-[#8b8d91]">
                <div className="p-4 bg-[#2d2f33]/50 rounded-2xl mb-3">
                  <Clock size={48} className="opacity-20" />
                </div>
                <p className="text-sm font-medium">No chat history yet</p>
                <p className="text-xs mt-1">Start a conversation to see it here</p>
              </div>
            ) : (
              chatHistories.map((history) => (
                <div
                  key={history.timestamp}
                  className="bg-gradient-to-br from-[#2d2f33] to-[#252629] border border-[#3d3f43] rounded-xl p-4 hover:border-[#3574f0]/50 hover:shadow-lg transition-all group"
                >
                  <div className="flex items-start justify-between gap-3 mb-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <div className="flex items-center gap-1.5 text-xs font-bold text-white px-2 py-1 bg-gradient-to-r from-[#3574f0]/20 to-[#c678dd]/20 rounded-lg">
                          {history.contextType === 'route' ? (
                            <>
                              <Route size={12} className="text-[#3574f0]" />
                              <span>Route Builder</span>
                            </>
                          ) : (
                            <>
                              <Cog size={12} className="text-[#c678dd]" />
                              <span>Functions</span>
                            </>
                          )}
                        </div>
                      </div>
                      <div className="text-[11px] text-[#8b8d91] mb-1">
                        {new Date(history.timestamp).toLocaleString()}
                      </div>
                      <div className="text-xs text-[#8b8d91]">
                        {history.messages.length} messages
                      </div>
                    </div>
                    <button
                      onClick={() => deleteHistory(history.timestamp)}
                      className="opacity-0 group-hover:opacity-100 p-2 text-[#e06c75] hover:bg-[#e06c75]/10 rounded-lg transition-all"
                    >
                      <Trash2 size={14} />
                    </button>
                  </div>
                  <button
                    onClick={() => loadHistory(history)}
                    className="w-full px-3 py-2 bg-[#3574f0] hover:bg-[#2d5fc7] text-white text-xs font-bold rounded-lg transition-all"
                  >
                    Load Conversation
                  </button>
                </div>
              ))
            )}
          </div>
        </div>
      )}

      {/* Messages */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-4">
        {messages.length === 0 && (
          <div className="h-full flex flex-col items-center justify-center text-center px-4">
            <div className="relative mb-6">
              <div className="absolute inset-0 bg-gradient-to-br from-[#3574f0] to-[#c678dd] rounded-3xl blur-2xl opacity-20"></div>
              <div className="relative p-6 bg-[#2d2f33]/50 rounded-3xl">
                <Sparkles size={64} className="text-[#3574f0]" />
              </div>
            </div>
            <h4 className="text-base font-bold text-white mb-2">How can I help you?</h4>
            <p className="text-xs text-[#8b8d91] mb-6 max-w-xs">
              Ask me anything about your {contextType === 'route' ? 'route' : 'function'} logic. I'm here to assist!
            </p>
            <div className="w-full max-w-sm space-y-2">
              <button
                onClick={() => setInput('Add tax calculation logic')}
                className="w-full px-4 py-3 bg-gradient-to-r from-[#2d2f33] to-[#252629] hover:from-[#3574f0]/20 hover:to-[#c678dd]/20 text-white text-sm rounded-xl transition-all hover:scale-105 text-left border border-[#3d3f43] hover:border-[#3574f0]/50 shadow-lg"
              >
                <Zap size={14} className="inline mr-2 text-[#3574f0]" />
                Add tax calculation logic
              </button>
              <button
                onClick={() => setInput('Check if user is admin')}
                className="w-full px-4 py-3 bg-gradient-to-r from-[#2d2f33] to-[#252629] hover:from-[#3574f0]/20 hover:to-[#c678dd]/20 text-white text-sm rounded-xl transition-all hover:scale-105 text-left border border-[#3d3f43] hover:border-[#3574f0]/50 shadow-lg"
              >
                <Zap size={14} className="inline mr-2 text-[#3574f0]" />
                Check if user is admin
              </button>
              <button
                onClick={() => setInput('Optimize this code')}
                className="w-full px-4 py-3 bg-gradient-to-r from-[#2d2f33] to-[#252629] hover:from-[#3574f0]/20 hover:to-[#c678dd]/20 text-white text-sm rounded-xl transition-all hover:scale-105 text-left border border-[#3d3f43] hover:border-[#3574f0]/50 shadow-lg"
              >
                <Zap size={14} className="inline mr-2 text-[#3574f0]" />
                Optimize this code
              </button>
            </div>
          </div>
        )}

        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'} animate-fadeIn`}
          >
            <div
              className={`max-w-[85%] rounded-2xl p-4 shadow-lg ${
                message.role === 'user'
                  ? 'bg-gradient-to-r from-[#3574f0] to-[#4a8cf7] text-white'
                  : 'bg-gradient-to-br from-[#2d2f33] to-[#252629] border border-[#3d3f43] text-white'
              }`}
            >
              {message.role === 'assistant' && (
                <div className="flex items-center justify-between mb-3 pb-2 border-b border-[#3d3f43]">
                  <div className="flex items-center gap-2">
                    <div className="p-1 bg-gradient-to-br from-[#3574f0]/20 to-[#c678dd]/20 rounded-lg">
                      <Sparkles size={12} className="text-[#3574f0]" />
                    </div>
                    <span className="text-[11px] font-bold uppercase tracking-wider text-[#3574f0]">AI Assistant</span>
                  </div>
                  {message.model && (
                    <div className="flex items-center gap-1.5 px-2 py-1 bg-[#3574f0]/10 rounded-lg">
                      <Cpu size={10} className="text-[#3574f0]" />
                      <span className="text-[9px] text-[#3574f0] font-bold">
                        {getModelDisplayName(message.model)}
                      </span>
                    </div>
                  )}
                </div>
              )}
              
              {editingMessageId === message.id ? (
                <div className="space-y-2">
                  <textarea
                    value={editedContent}
                    onChange={(e) => setEditedContent(e.target.value)}
                    className="w-full px-3 py-2 bg-[#1e1f22] border border-[#3d3f43] rounded-lg text-white text-sm resize-none custom-scrollbar"
                    rows={4}
                  />
                  <div className="flex gap-2">
                    <button
                      onClick={saveEditedMessage}
                      className="flex items-center gap-1.5 px-3 py-1.5 bg-[#3574f0] hover:bg-[#2d5fc7] text-white text-xs font-bold rounded-lg transition-all"
                    >
                      <CheckCircle size={12} />
                      Save
                    </button>
                    <button
                      onClick={cancelEditing}
                      className="flex items-center gap-1.5 px-3 py-1.5 bg-[#3d3f43] hover:bg-[#4d4f53] text-white text-xs font-bold rounded-lg transition-all"
                    >
                      <X size={12} />
                      Cancel
                    </button>
                  </div>
                </div>
              ) : (
                <div>
                  <div className="text-sm whitespace-pre-wrap break-words leading-relaxed">
                    {message.content.split(/```[\s\S]*?```/).map((text, i, arr) => {
                      if (i < arr.length - 1 || i === 0) {
                        return <span key={i}>{text}</span>;
                      }
                      return null;
                    })}
                  </div>
                  
                  {/* Timestamp and Action Buttons */}
                  <div className="flex items-center justify-between mt-3">
                    <div className="flex gap-1">
                      <button
                        onClick={() => copyMessageContent(message.content)}
                        className="p-1.5 hover:bg-white/10 text-white/70 hover:text-white rounded transition-all"
                        title="Copy message"
                      >
                        <Copy size={13} />
                      </button>
                      {message.role === 'user' && (
                        <>
                          <button
                            onClick={() => startEditingMessage(message)}
                            className="p-1.5 hover:bg-white/10 text-white/70 hover:text-white rounded transition-all"
                            title="Edit message"
                          >
                            <Edit2 size={13} />
                          </button>
                          <button
                            onClick={() => reRunMessage(message.content)}
                            className="p-1.5 hover:bg-white/10 text-white/70 hover:text-white rounded transition-all"
                            title="Re-run this message"
                          >
                            <RotateCw size={13} />
                          </button>
                        </>
                      )}
                      <button
                        onClick={() => deleteMessage(message.id)}
                        className="p-1.5 hover:bg-[#e06c75]/10 text-white/70 hover:text-[#e06c75] rounded transition-all"
                        title="Delete message"
                      >
                        <Trash2 size={13} />
                      </button>
                    </div>
                    <p className="text-[10px] opacity-40 font-medium">
                      {message.timestamp.toLocaleTimeString()}
                    </p>
                  </div>
                </div>
              )}

              {message.codeBlocks && message.codeBlocks.length > 0 && (
                <div className="mt-4 space-y-3">
                  {message.codeBlocks.map((block, index) => (
                    <div key={index} className="bg-[#1e1f22] border border-[#3d3f43] rounded-xl overflow-hidden shadow-lg">
                      <div className="flex items-center justify-between px-4 py-2 bg-[#252629] border-b border-[#3d3f43]">
                        <div className="flex items-center gap-2">
                          <Code size={14} className="text-[#3574f0]" />
                          <span className="text-[11px] font-mono text-[#8b8d91] uppercase font-bold tracking-wider">{block.language}</span>
                        </div>
                        <button
                          onClick={() => handleApplyCode(block.code)}
                          className="flex items-center gap-1.5 px-3 py-1.5 bg-gradient-to-r from-[#59a869] to-[#4a9158] hover:from-[#4a9158] hover:to-[#3b7946] text-white text-[11px] font-bold rounded-lg transition-all shadow-md hover:shadow-lg hover:scale-105"
                        >
                          <CheckCircle size={12} />
                          Apply Code
                        </button>
                      </div>
                      <pre className="p-4 text-xs font-mono text-[#dfe1e5] overflow-x-auto custom-scrollbar leading-relaxed">
                        {block.code}
                      </pre>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}

        {loading && (
          <div className="flex justify-start animate-fadeIn">
            <div className="max-w-[85%] rounded-2xl p-4 bg-gradient-to-br from-[#2d2f33] to-[#252629] border border-[#3d3f43] shadow-lg">
              <div className="flex items-center gap-3">
                <div className="relative">
                  <Loader2 size={20} className="animate-spin text-[#3574f0]" />
                  <div className="absolute inset-0 bg-[#3574f0] rounded-full blur-md opacity-30 animate-pulse"></div>
                </div>
                <div>
                  <div className="text-sm text-white font-medium">AI is thinking...</div>
                  <div className="text-xs text-[#8b8d91] mt-0.5">Analyzing your request</div>
                </div>
              </div>
            </div>
          </div>
        )}

        {error && (
          <div className="bg-gradient-to-r from-[#e06c75]/10 to-[#e06c75]/5 border border-[#e06c75]/30 rounded-xl p-4 flex items-start gap-3 shadow-lg animate-fadeIn">
            <div className="p-2 bg-[#e06c75]/10 rounded-lg">
              <AlertTriangle size={18} className="text-[#e06c75]" />
            </div>
            <div className="flex-1">
              <div className="text-xs font-bold text-[#e06c75] mb-1">Error</div>
              <div className="text-xs text-[#e06c75]/80">{error}</div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Modern Input with Glass Effect */}
      <div className="p-3 border-t border-[#3d3f43]/50 bg-gradient-to-b from-[#1e1f22] to-[#252629] backdrop-blur-xl">
        <div className="relative flex items-end gap-2">
          <textarea
            ref={inputRef}
            value={input}
            onChange={(e) => {
              setInput(e.target.value);
              // Auto-resize textarea
              const textarea = e.target;
              textarea.style.height = 'auto';
              textarea.style.height = Math.min(textarea.scrollHeight, 120) + 'px';
            }}
            onKeyDown={handleKeyDown}
            placeholder="Ask anything..."
            rows={1}
            maxLength={2000}
            className="flex-1 px-3 py-2 bg-gradient-to-r from-[#2d2f33] to-[#252629] border border-[#3d3f43] rounded-lg text-white text-sm placeholder-[#8b8d91] focus:outline-none focus:border-[#3574f0] focus:ring-1 focus:ring-[#3574f0]/30 transition-all resize-none custom-scrollbar shadow-inner max-h-[120px] overflow-y-auto leading-5"
            style={{ minHeight: '40px', height: '40px', transition: 'height 0.2s ease' }}
          />
          <button
            onClick={sendMessage}
            disabled={!input.trim() || loading}
            className="flex-shrink-0 p-2 bg-[#3574f0] hover:bg-[#2d5fc7] disabled:bg-[#3d3f43] disabled:opacity-50 text-white rounded-lg transition-all self-end"
          >
            {loading ? <Loader2 size={16} className="animate-spin" /> : <Send size={16} />}
          </button>
        </div>
        <div className="flex items-center justify-between mt-1.5 px-0.5">
          <div className="flex items-center gap-1.5 text-[8px] text-[#8b8d91]">
            <kbd className="px-1 py-0.5 bg-[#2d2f33] border border-[#3d3f43] rounded text-[7px] font-mono">Enter</kbd>
            <span>send</span>
            <span className="text-[#3d3f43]">•</span>
            <kbd className="px-1 py-0.5 bg-[#2d2f33] border border-[#3d3f43] rounded text-[7px] font-mono">Shift+Enter</kbd>
            <span>new line</span>
          </div>
          <div className={`text-[8px] font-mono font-bold ${input.length > 1800 ? 'text-[#e06c75]' : 'text-[#8b8d91]'}`}>
            {input.length}/2000
          </div>
        </div>
      </div>
      </div>
    </div>
  );
};
