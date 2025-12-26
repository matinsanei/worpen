import React, { useState, useRef, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_AUTOMATION_RULES } from '../constants';
import { AutomationRule } from '../types';
import { Bot, Play, Save, Plus, Terminal, Zap, Code } from 'lucide-react';

// --- WORPEN LANGUAGE DEFINITIONS ---
const COMPLETIONS: Record<string, string[]> = {
  'docker.': ['restart', 'scale', 'prune', 'replicas', 'logs', 'inspect', 'list_containers'],
  'log.': ['info', 'warn', 'error', 'success', 'debug'],
  'notify.': ['slack', 'discord', 'email', 'pagerduty'],
  'metrics.': ['cpu_usage', 'memory_usage', 'disk_io', 'latency', 'request_count'],
  'event.': ['exit_code', 'service_name', 'timestamp', 'node_id']
};

const ScriptEditor = ({ initialCode, readOnly = false }: { initialCode: string, readOnly?: boolean }) => {
  const [code, setCode] = useState(initialCode);
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [suggestionPos, setSuggestionPos] = useState({ top: 0, left: 0 });
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const preRef = useRef<HTMLPreElement>(null);

  useEffect(() => {
    setCode(initialCode);
  }, [initialCode]);

  // Syntax Highlighting Logic
  const highlightCode = (input: string) => {
    let html = input
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");

    // Order matters: Strings -> Comments -> Keywords -> Methods -> Objects -> Numbers

    // Strings
    html = html.replace(/(['"`])(.*?)\1/g, '<span class="text-yellow-300">$1$2$1</span>');

    // Comments
    html = html.replace(/(\/\/.*)/g, '<span class="text-green-700 italic">$1</span>');

    // Control Flow / Keywords
    html = html.replace(/\b(await|if|else|return|let|const|var|function|async|for|while)\b/g, '<span class="text-purple-400 font-bold">$1</span>');

    // Worpen Objects
    html = html.replace(/\b(docker|log|notify|metrics|event)\b/g, '<span class="text-blue-400 font-bold">$1</span>');

    // Methods
    html = html.replace(/\b(restart|scale|prune|info|warn|error|slack|push|replicas|logs)\b/g, '<span class="text-green-300">$1</span>');

    // Numbers
    html = html.replace(/\b(\d+)\b/g, '<span class="text-orange-400">$1</span>');

    return html;
  };

  const handleInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newVal = e.target.value;
    setCode(newVal);

    // Simple autocomplete trigger detection
    const cursorPos = e.target.selectionStart;
    const textBeforeCursor = newVal.substring(0, cursorPos);
    const lastWordMatch = textBeforeCursor.match(/([a-zA-Z0-9_]+\.)$/); // Match "object."

    if (lastWordMatch) {
      const trigger = lastWordMatch[1];
      if (COMPLETIONS[trigger]) {
        setSuggestions(COMPLETIONS[trigger]);

        // Estimation of cursor position for the popup
        const lines = textBeforeCursor.substring(0, cursorPos).split('\n');
        const currentLineIndex = lines.length - 1;
        const currentLineChars = lines[lines.length - 1].length;

        setSuggestionPos({
          top: (currentLineIndex + 1) * 24, // 24px line height
          left: (currentLineChars * 8.5) + 40 // ~8.5px char width + padding
        });
        return;
      }
    }
    setSuggestions([]);
  };

  const insertSuggestion = (sugg: string) => {
    if (!textareaRef.current) return;

    const cursorPos = textareaRef.current.selectionStart;
    const text = code;
    const textBefore = text.substring(0, cursorPos);
    const textAfter = text.substring(cursorPos);

    // Append suggestion
    const newCode = textBefore + sugg + textAfter;
    setCode(newCode);
    setSuggestions([]);

    // Focus back and move cursor
    setTimeout(() => {
      textareaRef.current?.focus();
      const newPos = cursorPos + sugg.length;
      textareaRef.current?.setSelectionRange(newPos, newPos);
    }, 10);
  };

  const handleScroll = () => {
    if (textareaRef.current && preRef.current) {
      preRef.current.scrollTop = textareaRef.current.scrollTop;
      preRef.current.scrollLeft = textareaRef.current.scrollLeft;
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Tab') {
      e.preventDefault();
      const start = e.currentTarget.selectionStart;
      const end = e.currentTarget.selectionEnd;
      const value = e.currentTarget.value;
      setCode(value.substring(0, start) + "  " + value.substring(end));
      setTimeout(() => {
        textareaRef.current!.selectionStart = textareaRef.current!.selectionEnd = start + 2;
      }, 0);
    } else if (e.key === 'Enter' && suggestions.length > 0) {
      e.preventDefault();
      insertSuggestion(suggestions[0]);
    }
  };

  return (
    <div className="relative flex flex-col h-full font-mono text-sm bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] shadow-inner group overflow-hidden">

      {/* Editor Container */}
      <div className="relative flex-1 overflow-hidden">
        {/* Line Numbers */}
        <div className="absolute left-0 top-0 bottom-0 w-10 bg-[#2b2d30] border-r border-[#43454a] text-[#6e7073] text-right pr-2 pt-4 select-none leading-6 z-20 text-[11px]">
          {code.split('\n').map((_, i) => <div key={i}>{i + 1}</div>)}
        </div>

        {/* Syntax Highlighted Underlay */}
        <pre
          ref={preRef}
          className="absolute inset-0 left-10 pl-2 pt-4 pr-4 pb-4 m-0 overflow-hidden whitespace-pre-wrap break-all leading-6 pointer-events-none z-0 text-[#dfe1e5]"
          dangerouslySetInnerHTML={{ __html: highlightCode(code) + '<br/>' }}
        />

        {/* Transparent Input Overlay */}
        <textarea
          ref={textareaRef}
          value={code}
          onChange={handleInput}
          onScroll={handleScroll}
          onKeyDown={handleKeyDown}
          spellCheck={false}
          disabled={readOnly}
          className={`absolute inset-0 left-10 pl-4 pt-4 pr-4 pb-4 h-full bg-transparent text-transparent caret-[#3574f0] resize-none outline-none leading-6 z-10 whitespace-pre-wrap break-all ${readOnly ? 'cursor-not-allowed' : ''}`}
          style={{ color: 'transparent' }}
        />

        {/* Autocomplete Popup */}
        {suggestions.length > 0 && (
          <div
            className="absolute z-50 bg-[#2b2d30] border border-[#43454a] shadow-xl min-w-[180px] flex flex-col rounded-md overflow-hidden"
            style={{ top: Math.min(suggestionPos.top, 200), left: Math.min(suggestionPos.left, 400) }}
          >
            <div className="bg-[#1e1f22] text-[#6e7073] text-[9px] px-2.5 py-1.5 uppercase tracking-wider font-bold border-b border-[#43454a]">Suggestions</div>
            {suggestions.map((s, i) => (
              <button
                key={s}
                onClick={() => insertSuggestion(s)}
                className={`text-left px-3 py-1.5 text-xs text-[#dfe1e5] hover:bg-[#3574f0] hover:text-white flex items-center gap-2 transition-colors ${i === 0 ? 'bg-[#3574f015]' : ''}`}
              >
                <Zap size={10} className="text-[#f2c55c]" /> {s}
              </button>
            ))}
            <div className="text-[9px] text-[#6e7073] px-2.5 py-1.5 bg-[#1e1f22] border-t border-[#43454a]">TAB / ENTER to select</div>
          </div>
        )}
      </div>

      {/* Editor Status Bar */}
      <div className="bg-[#2b2d30] border-t border-[#43454a] p-1 px-3 flex justify-between items-center text-[10px] text-[#6e7073] select-none font-sans">
        <div className="flex gap-4">
          <span className="font-bold">LINES: {code.split('\n').length}</span>
          <span className="font-bold">CHARS: {code.length}</span>
          <span className="text-[#59a869] font-bold">UTF-8</span>
        </div>
        <div className="flex items-center gap-2">
          <div className={`w-2 h-2 rounded-full ${readOnly ? 'bg-[#6e7073]' : 'bg-[#59a869] shadow-[0_0_8px_rgba(89,168,105,0.4)] animate-pulse'}`}></div>
          <span className="font-bold uppercase tracking-tighter">{readOnly ? 'READ_ONLY' : 'INSERT MODE'}</span>
        </div>
      </div>
    </div>
  );
};

export const AutomationView: React.FC = () => {
  const [selectedRule, setSelectedRule] = useState<AutomationRule>(MOCK_AUTOMATION_RULES[0]);

  return (
    <div className="h-full p-6 flex flex-col gap-6 max-w-[1600px] mx-auto overflow-hidden">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-3 text-[#dfe1e5] tracking-tight">
            <div className="p-2 rounded-[var(--radius)] bg-[#a682e615] border border-[#a682e620]">
              <Bot className="text-[#a682e6]" size={20} />
            </div>
            Self-Healing Matrix
          </h1>
          <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">Automated Response Protocols // V8 Isolation Engine</p>
        </div>
        <button className="px-5 py-2 bg-[#3574f0] hover:bg-[#3574f0e0] text-white text-[11px] font-bold rounded-[var(--radius)] shadow-lg transition-all active:scale-95 flex items-center gap-2 uppercase tracking-tight border border-[#3574f0] group">
          <Plus size={14} className="group-hover:rotate-90 transition-transform" /> NEW STRATEGY
        </button>
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">
        {/* List of Rules */}
        <div className="lg:col-span-4 flex flex-col gap-3 overflow-y-auto pr-2 custom-scrollbar">
          {MOCK_AUTOMATION_RULES.map(rule => (
            <div
              key={rule.id}
              onClick={() => setSelectedRule(rule)}
              className={`jb-card p-4 cursor-pointer transition-all group relative overflow-hidden ${selectedRule.id === rule.id
                ? 'border-[#3574f0] bg-[#3574f008]'
                : 'hover:border-[#6a6e75] hover:bg-[#393b4040]'
                }`}
            >
              <div className="flex justify-between items-start mb-2">
                <span className={`font-bold text-[14px] transition-colors ${selectedRule.id === rule.id ? 'text-[#3574f0]' : 'text-[#dfe1e5]'}`}>{rule.name}</span>
                <div className={`w-2.5 h-2.5 rounded-full ${rule.active ? 'bg-[#59a869] shadow-[0_0_10px_rgba(89,168,105,0.4)]' : 'bg-[#43454a]'}`}></div>
              </div>
              <div className="text-[10px] text-[#6e7073] font-bold uppercase tracking-wider mb-3">
                TRIGGER: <span className="text-[#f2c55c]">{rule.triggerEvent}</span>
              </div>
              <div className="flex justify-between text-[11px] text-[#6e7073] font-bold uppercase">
                <span className="flex items-center gap-1.5"><Terminal size={12} className="opacity-50" /> {rule.lastRun}</span>
                <span className="flex items-center gap-1.5"><Zap size={12} className="opacity-50 text-[#a682e6]" /> {rule.targetService}</span>
              </div>
              {selectedRule.id === rule.id && (
                <div className="absolute top-0 left-0 w-1 h-full bg-[#3574f0]"></div>
              )}
            </div>
          ))}
        </div>

        {/* Code Editor Area */}
        <div className="lg:col-span-8 flex flex-col h-full min-h-[400px]">
          <div className="jb-card flex-1 flex flex-col overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
              <div className="flex items-center gap-2">
                <Code size={14} className="text-[#3574f0]" />
                <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider italic">Edit Strategy: {selectedRule.name}</span>
              </div>
              <div className="flex gap-4 text-[10px] font-bold text-[#6e7073] uppercase">
                <span className="flex items-center gap-1.5"><Terminal size={12} className="text-[#3574f0] opacity-70" /> V8 ISOLATE</span>
                <span className="flex items-center gap-1.5"><Zap size={12} className="text-[#f2c55c] opacity-70" /> 2ms LATENCY</span>
              </div>
            </div>
            <div className="flex-1 p-4 flex flex-col gap-4 overflow-hidden">
              <div className="flex-1 min-h-0 relative">
                <ScriptEditor initialCode={selectedRule.script} />
              </div>

              <div className="flex justify-end gap-3 pt-2">
                <button className="flex items-center gap-2 px-6 py-2.5 text-[11px] font-bold bg-[#2b2d30] border border-[#43454a] hover:bg-[#393b40] text-[#dfe1e5] transition-all rounded-[var(--radius)] uppercase tracking-tight">
                  <Play size={14} /> Run Dry Test
                </button>
                <button className="flex items-center gap-2 px-6 py-2.5 text-[11px] font-bold bg-[#59a869] border border-[#59a869] hover:bg-[#59a869e0] text-white transition-all rounded-[var(--radius)] uppercase tracking-tight shadow-lg shadow-[#59a86920]">
                  <Save size={16} /> Save & Deploy
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};