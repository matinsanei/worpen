import React, { useState, useRef, useEffect } from 'react';
import { X, Send, Sparkles, Loader2, Code, CheckCircle, MessageSquare, AlertTriangle } from 'lucide-react';
import { getAIConfig, isAIConfigured } from '../services/aiConfig';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  codeBlocks?: Array<{ language: string; code: string }>;
  timestamp: Date;
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
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

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
  };

  if (!isOpen) return null;

  return (
    <div className="h-full w-96 border-l border-[#43454a] bg-[#2b2d30] flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-[#43454a] bg-gradient-to-r from-[#3574f0]/10 to-[#c678dd]/10">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-[#3574f0] to-[#c678dd] flex items-center justify-center">
            <Sparkles size={16} className="text-white" />
          </div>
          <div>
            <h3 className="text-sm font-bold text-[#dfe1e5]">Worpen Copilot</h3>
            <p className="text-[10px] text-[#6e7073]">Context-aware assistant</p>
          </div>
        </div>
        <div className="flex items-center gap-1">
          {messages.length > 0 && (
            <button
              onClick={clearChat}
              className="p-1.5 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] rounded-[var(--radius)] transition-colors text-xs"
              title="Clear chat"
            >
              Clear
            </button>
          )}
          <button
            onClick={onClose}
            className="p-1.5 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] rounded-[var(--radius)] transition-colors"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-3">
        {messages.length === 0 && (
          <div className="h-full flex flex-col items-center justify-center text-center text-[#6e7073] px-4">
            <MessageSquare size={48} className="mb-3 opacity-20" />
            <p className="text-sm mb-2">How can I help you?</p>
            <div className="space-y-1.5 text-xs">
              <button
                onClick={() => setInput('Add tax calculation logic')}
                className="w-full px-3 py-2 bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] rounded-[var(--radius)] transition-colors text-left"
              >
                Add tax calculation logic
              </button>
              <button
                onClick={() => setInput('Check if user is admin')}
                className="w-full px-3 py-2 bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] rounded-[var(--radius)] transition-colors text-left"
              >
                Check if user is admin
              </button>
              <button
                onClick={() => setInput('Optimize this code')}
                className="w-full px-3 py-2 bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] rounded-[var(--radius)] transition-colors text-left"
              >
                Optimize this code
              </button>
            </div>
          </div>
        )}

        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[85%] rounded-lg p-3 ${
                message.role === 'user'
                  ? 'bg-[#3574f0] text-white'
                  : 'bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5]'
              }`}
            >
              {message.role === 'assistant' && (
                <div className="flex items-center gap-1.5 mb-2 text-[#3574f0]">
                  <Sparkles size={12} />
                  <span className="text-[10px] font-bold uppercase tracking-wider">Copilot</span>
                </div>
              )}
              
              <div className="text-sm whitespace-pre-wrap break-words">
                {message.content.split(/```[\s\S]*?```/).map((text, i, arr) => {
                  if (i < arr.length - 1 || i === 0) {
                    return <span key={i}>{text}</span>;
                  }
                  return null;
                })}
              </div>

              {message.codeBlocks && message.codeBlocks.length > 0 && (
                <div className="mt-3 space-y-2">
                  {message.codeBlocks.map((block, index) => (
                    <div key={index} className="bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] overflow-hidden">
                      <div className="flex items-center justify-between px-3 py-1.5 bg-[#2b2d30] border-b border-[#43454a]">
                        <div className="flex items-center gap-1.5">
                          <Code size={12} className="text-[#6e7073]" />
                          <span className="text-[10px] font-mono text-[#6e7073] uppercase">{block.language}</span>
                        </div>
                        <button
                          onClick={() => handleApplyCode(block.code)}
                          className="flex items-center gap-1 px-2 py-1 bg-[#59a869] hover:bg-[#4a9158] text-white text-[10px] font-bold rounded transition-all active:scale-95"
                        >
                          <CheckCircle size={10} />
                          Apply
                        </button>
                      </div>
                      <pre className="p-3 text-xs font-mono text-[#dfe1e5] overflow-x-auto custom-scrollbar">
                        {block.code}
                      </pre>
                    </div>
                  ))}
                </div>
              )}

              <p className="text-[9px] opacity-50 mt-2">
                {message.timestamp.toLocaleTimeString()}
              </p>
            </div>
          </div>
        ))}

        {loading && (
          <div className="flex justify-start">
            <div className="max-w-[85%] rounded-lg p-3 bg-[#1e1f22] border border-[#43454a]">
              <div className="flex items-center gap-2 text-[#3574f0]">
                <Loader2 size={14} className="animate-spin" />
                <span className="text-xs">Copilot is thinking...</span>
              </div>
            </div>
          </div>
        )}

        {error && (
          <div className="bg-[#e06c75]/10 border border-[#e06c75]/30 rounded-[var(--radius)] p-3 flex items-start gap-2">
            <AlertTriangle size={14} className="text-[#e06c75] flex-shrink-0 mt-0.5" />
            <div className="flex-1">
              <p className="text-xs text-[#e06c75]">{error}</p>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="p-3 border-t border-[#43454a] bg-[#2b2d30]">
        <div className="flex gap-2">
          <textarea
            ref={inputRef}
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask Copilot anything..."
            rows={3}
            className="flex-1 px-3 py-2 bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] text-[#dfe1e5] text-sm placeholder-[#6e7073] focus:outline-none focus:border-[#3574f0] transition-colors resize-none custom-scrollbar"
          />
          <button
            onClick={sendMessage}
            disabled={!input.trim() || loading}
            className="px-3 bg-[#3574f0] hover:bg-[#2d5fc7] disabled:bg-[#43454a] disabled:opacity-50 text-white rounded-[var(--radius)] transition-all active:scale-95"
          >
            <Send size={16} />
          </button>
        </div>
        <p className="text-[9px] text-[#6e7073] mt-2 text-center">
          Press Enter to send • Shift+Enter for new line
        </p>
      </div>
    </div>
  );
};
