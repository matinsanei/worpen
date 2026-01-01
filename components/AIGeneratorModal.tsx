import React, { useState } from 'react';
import { X, Sparkles, Loader2, CheckCircle, AlertTriangle, Copy, Wand2 } from 'lucide-react';
import { generateLogicFromPrompt, LogicType } from '../services/aiService';
import Editor from "@monaco-editor/react";

interface AIGeneratorModalProps {
  isOpen: boolean;
  onClose: () => void;
  onUseCode: (code: string) => void;
  type: LogicType;
}

export const AIGeneratorModal: React.FC<AIGeneratorModalProps> = ({
  isOpen,
  onClose,
  onUseCode,
  type,
}) => {
  const [prompt, setPrompt] = useState<string>('');
  const [generatedCode, setGeneratedCode] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<boolean>(false);

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      setError('Please describe what you want the logic to do');
      return;
    }

    setLoading(true);
    setError(null);
    setSuccess(false);

    try {
      const response = await generateLogicFromPrompt(prompt, type);
      
      if (response.success) {
        const formatted = JSON.stringify(response.logic, null, 2);
        setGeneratedCode(formatted);
        setSuccess(true);
      } else {
        setError(response.error || 'Failed to generate logic');
      }
    } catch (err: any) {
      setError(err.message || 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  const handleUseCode = () => {
    onUseCode(generatedCode);
    handleClose();
  };

  const handleClose = () => {
    setPrompt('');
    setGeneratedCode('');
    setError(null);
    setSuccess(false);
    onClose();
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedCode);
  };

  const examplePrompts = type === 'route' ? [
    'Calculate tax on a purchase amount',
    'Validate user input and check if value is positive',
    'Loop through items and process each one',
    'Make an HTTP request to external API',
  ] : [
    'Add two numbers together',
    'Format a string to uppercase',
    'Calculate shipping fee based on weight',
    'Convert temperature from Celsius to Fahrenheit',
  ];

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="w-full max-w-4xl max-h-[90vh] flex flex-col bg-[#2b2d30] border border-[#43454a] rounded-lg shadow-2xl overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-[#43454a] bg-gradient-to-r from-[#3574f0]/10 to-[#c678dd]/10">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-[#3574f0] to-[#c678dd] flex items-center justify-center shadow-lg">
              <Sparkles size={20} className="text-white" />
            </div>
            <div>
              <h2 className="text-lg font-bold text-[#dfe1e5]">AI Logic Generator</h2>
              <p className="text-xs text-[#6e7073]">
                Describe what you want in plain English
              </p>
            </div>
          </div>
          <button
            onClick={handleClose}
            className="p-2 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] rounded-[var(--radius)] transition-colors"
          >
            <X size={18} />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-4">
          {/* Prompt Input */}
          <div>
            <label className="block text-xs font-semibold text-[#dfe1e5] mb-2 uppercase tracking-wider">
              Describe Your Logic
            </label>
            <textarea
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder="Example: Calculate the total price including 15% tax..."
              className="w-full h-32 px-4 py-3 bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] text-[#dfe1e5] placeholder-[#6e7073] focus:outline-none focus:border-[#3574f0] transition-colors resize-none text-sm"
              disabled={loading}
            />
          </div>

          {/* Example Prompts */}
          <div>
            <p className="text-xs font-semibold text-[#6e7073] mb-2 uppercase tracking-wider">
              Example Prompts:
            </p>
            <div className="flex flex-wrap gap-2">
              {examplePrompts.map((example, idx) => (
                <button
                  key={idx}
                  onClick={() => setPrompt(example)}
                  disabled={loading}
                  className="px-3 py-1.5 text-xs bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] rounded-full transition-colors disabled:opacity-50"
                >
                  {example}
                </button>
              ))}
            </div>
          </div>

          {/* Generate Button */}
          <div className="flex items-center gap-3">
            <button
              onClick={handleGenerate}
              disabled={loading || !prompt.trim()}
              className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-[#3574f0] to-[#c678dd] hover:from-[#2d5fc7] hover:to-[#a85ec4] text-white font-bold rounded-[var(--radius)] transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
            >
              {loading ? (
                <>
                  <Loader2 size={18} className="animate-spin" />
                  Generating Magic...
                </>
              ) : (
                <>
                  <Wand2 size={18} />
                  Generate Logic
                </>
              )}
            </button>

            {success && (
              <div className="flex items-center gap-2 text-[#59a869] text-sm">
                <CheckCircle size={16} />
                <span className="font-medium">Generated successfully!</span>
              </div>
            )}
          </div>

          {/* Error Message */}
          {error && (
            <div className="flex items-start gap-2 p-3 bg-[#e06c75]/10 border border-[#e06c75]/30 rounded-[var(--radius)] text-[#e06c75]">
              <AlertTriangle size={16} className="flex-shrink-0 mt-0.5" />
              <p className="text-xs">{error}</p>
            </div>
          )}

          {/* Generated Code Preview */}
          {generatedCode && (
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <label className="block text-xs font-semibold text-[#dfe1e5] uppercase tracking-wider">
                  Generated Logic
                </label>
                <button
                  onClick={handleCopy}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] text-xs rounded-[var(--radius)] transition-colors"
                >
                  <Copy size={12} />
                  Copy
                </button>
              </div>
              <div className="h-[300px] border border-[#43454a] rounded-[var(--radius)] overflow-hidden">
                <Editor
                  height="100%"
                  defaultLanguage="json"
                  theme="vs-dark"
                  value={generatedCode}
                  options={{
                    readOnly: true,
                    minimap: { enabled: false },
                    fontSize: 12,
                    fontFamily: "'Fira Code', 'Monaco', monospace",
                    fontLigatures: false,
                    letterSpacing: 0,
                    lineHeight: 18,
                    scrollBeyondLastLine: false,
                    automaticLayout: true,
                    padding: { top: 12, bottom: 12 },
                    wordWrap: 'off',
                    smoothScrolling: false,
                    cursorBlinking: 'solid',
                    renderWhitespace: 'selection',
                  }}
                />
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        {generatedCode && (
          <div className="flex items-center justify-between p-4 border-t border-[#43454a] bg-[#2b2d30]">
            <p className="text-xs text-[#6e7073]">
              Review the generated code and click "Use Code" to insert it into your editor
            </p>
            <div className="flex items-center gap-2">
              <button
                onClick={handleClose}
                className="px-4 py-2 bg-[#393b40] hover:bg-[#4e5157] text-[#dfe1e5] text-sm font-medium rounded-[var(--radius)] transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleUseCode}
                className="px-6 py-2 bg-[#59a869] hover:bg-[#4a9158] text-white text-sm font-bold rounded-[var(--radius)] transition-all active:scale-95"
              >
                Use Code
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
