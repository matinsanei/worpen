import React, { useState, useRef, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_AUTOMATION_RULES } from '../constants';
import { AutomationRule } from '../types';
import { Bot, Play, Save, Plus, Terminal, Zap } from 'lucide-react';

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
    <div className="relative flex flex-col h-full font-mono text-sm bg-[#080a08] border border-green-900 shadow-inner group">
      
      {/* Editor Container */}
      <div className="relative flex-1 overflow-hidden">
        {/* Line Numbers */}
        <div className="absolute left-0 top-0 bottom-0 w-10 bg-[#0d110d] border-r border-green-900 text-gray-600 text-right pr-2 pt-4 select-none leading-6 z-20">
          {code.split('\n').map((_, i) => <div key={i}>{i + 1}</div>)}
        </div>

        {/* Syntax Highlighted Underlay */}
        <pre 
          ref={preRef}
          className="absolute inset-0 left-10 pl-2 pt-4 pr-4 pb-4 m-0 overflow-hidden whitespace-pre-wrap break-all leading-6 pointer-events-none z-0"
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
          className={`absolute inset-0 left-10 pl-2 pt-4 pr-4 pb-4 h-full bg-transparent text-transparent caret-green-500 resize-none outline-none leading-6 z-10 whitespace-pre-wrap break-all ${readOnly ? 'cursor-not-allowed' : ''}`}
          style={{ color: 'transparent' }} 
        />

        {/* Autocomplete Popup */}
        {suggestions.length > 0 && (
          <div 
            className="absolute z-50 bg-[#051a05] border border-green-500 shadow-[0_0_15px_rgba(0,255,0,0.3)] min-w-[150px] flex flex-col"
            style={{ top: Math.min(suggestionPos.top, 200), left: Math.min(suggestionPos.left, 400) }}
          >
            <div className="bg-green-900 text-white text-[10px] px-2 py-1 uppercase tracking-wider font-bold">Suggestions</div>
            {suggestions.map((s, i) => (
              <button 
                key={s} 
                onClick={() => insertSuggestion(s)}
                className={`text-left px-2 py-1 text-xs text-green-300 hover:bg-green-700 hover:text-white flex items-center gap-2 ${i === 0 ? 'bg-green-900/30' : ''}`}
              >
                <Zap size={10} className="text-yellow-500"/> {s}
              </button>
            ))}
            <div className="text-[9px] text-gray-500 px-2 py-1 border-t border-green-900/50">TAB / ENTER to select</div>
          </div>
        )}
      </div>

      {/* Editor Status Bar */}
      <div className="bg-[#0d110d] border-t border-green-900 p-1 px-2 flex justify-between items-center text-[10px] text-gray-500 select-none">
         <div className="flex gap-4">
            <span>LINES: {code.split('\n').length}</span>
            <span>CHARS: {code.length}</span>
            <span className="text-green-700">UTF-8</span>
         </div>
         <div className="flex items-center gap-2">
            <div className={`w-2 h-2 rounded-full ${readOnly ? 'bg-gray-600' : 'bg-green-500 animate-pulse'}`}></div>
            <span>{readOnly ? 'READ_ONLY' : 'INSERT MODE'}</span>
         </div>
      </div>
    </div>
  );
};

export const AutomationView: React.FC = () => {
  const [selectedRule, setSelectedRule] = useState<AutomationRule>(MOCK_AUTOMATION_RULES[0]);

  return (
    <div className="h-full p-2 flex flex-col gap-4">
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2">
        <div>
           <h1 className="text-2xl font-bold flex items-center gap-2">
             <Bot className="text-purple-400" /> 
             SELF_HEALING_MATRIX
           </h1>
           <p className="text-xs text-green-600">AUTOMATED RESPONSE PROTOCOLS</p>
        </div>
        <button className="px-4 py-1 bg-purple-900/30 border border-purple-500 text-purple-200 text-xs hover:bg-purple-800 flex items-center gap-2 group">
           <Plus size={14} className="group-hover:rotate-90 transition-transform"/> NEW_STRATEGY
        </button>
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-3 gap-4 min-h-0">
        {/* List of Rules */}
        <div className="lg:col-span-1 flex flex-col gap-2 overflow-y-auto pr-1">
           {MOCK_AUTOMATION_RULES.map(rule => (
             <div 
               key={rule.id}
               onClick={() => setSelectedRule(rule)}
               className={`p-3 border cursor-pointer transition-all group ${
                 selectedRule.id === rule.id 
                 ? 'bg-green-900/30 border-green-400' 
                 : 'bg-black border-green-900 hover:border-green-600'
               }`}
             >
                <div className="flex justify-between items-start mb-1">
                  <span className={`font-bold text-sm ${selectedRule.id === rule.id ? 'text-white' : 'text-green-200'}`}>{rule.name}</span>
                  <div className={`w-2 h-2 rounded-full ${rule.active ? 'bg-green-500 shadow-[0_0_5px_#00ff00]' : 'bg-gray-600'}`}></div>
                </div>
                <div className="text-[10px] text-gray-400 mb-2 group-hover:text-gray-300">
                   TRIGGER: <span className="text-yellow-500">{rule.triggerEvent}</span>
                </div>
                <div className="flex justify-between text-[10px] text-gray-500 font-mono">
                   <span>LAST: {rule.lastRun}</span>
                   <span>TARGET: {rule.targetService}</span>
                </div>
             </div>
           ))}
        </div>

        {/* Code Editor Area */}
        <div className="lg:col-span-2 h-full min-h-[400px] flex flex-col">
          <TerminalFrame title={`EDIT_STRATEGY: ${selectedRule.name}`} className="flex-1 flex flex-col">
             <div className="flex flex-col h-full p-1 gap-2">
               
               {/* Using the new Rich Editor */}
               <div className="flex-1 min-h-0 relative">
                  <ScriptEditor initialCode={selectedRule.script} />
               </div>
               
               <div className="flex justify-between items-center pt-2 border-t border-green-900">
                  <div className="flex gap-2 text-[10px] text-gray-500">
                    <span className="flex items-center gap-1"><Terminal size={10}/> RUNTIME: V8_ISOLATE</span>
                    <span className="flex items-center gap-1"><Zap size={10}/> LATENCY: 2ms</span>
                  </div>
                  <div className="flex gap-3">
                    <button className="flex items-center gap-2 px-4 py-2 text-xs bg-gray-900 border border-gray-600 hover:bg-gray-800 text-gray-300 transition-colors">
                      <Play size={12}/> TEST_DRY_RUN
                    </button>
                    <button className="flex items-center gap-2 px-4 py-2 text-xs bg-green-900 border border-green-500 hover:bg-green-700 text-white font-bold transition-all hover:shadow-[0_0_10px_rgba(0,255,0,0.3)]">
                      <Save size={14} /> SAVE_AND_DEPLOY
                    </button>
                  </div>
               </div>
             </div>
          </TerminalFrame>
        </div>
      </div>
    </div>
  );
};