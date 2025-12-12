import React, { useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_AUTOMATION_RULES } from '../constants';
import { AutomationRule } from '../types';
import { Bot, Play, Save, Plus } from 'lucide-react';

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
        <button className="px-4 py-1 bg-purple-900/30 border border-purple-500 text-purple-200 text-xs hover:bg-purple-800 flex items-center gap-2">
           <Plus size={14} /> NEW_STRATEGY
        </button>
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-3 gap-4 min-h-0">
        {/* List of Rules */}
        <div className="lg:col-span-1 flex flex-col gap-2 overflow-y-auto">
           {MOCK_AUTOMATION_RULES.map(rule => (
             <div 
               key={rule.id}
               onClick={() => setSelectedRule(rule)}
               className={`p-3 border cursor-pointer transition-all ${
                 selectedRule.id === rule.id 
                 ? 'bg-green-900/30 border-green-400' 
                 : 'bg-black border-green-900 hover:border-green-600'
               }`}
             >
                <div className="flex justify-between items-start mb-1">
                  <span className="font-bold text-sm text-green-200">{rule.name}</span>
                  <div className={`w-2 h-2 rounded-full ${rule.active ? 'bg-green-500 shadow-[0_0_5px_#00ff00]' : 'bg-gray-600'}`}></div>
                </div>
                <div className="text-[10px] text-gray-400 mb-2">
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
        <div className="lg:col-span-2 h-full min-h-[300px]">
          <TerminalFrame title={`EDIT_STRATEGY: ${selectedRule.name}`} className="h-full">
             <div className="flex flex-col h-full">
               <div className="bg-[#1e1e1e] p-4 flex-1 font-mono text-sm text-gray-300 relative overflow-auto custom-scrollbar border border-green-900/50 shadow-inner">
                  {/* Fake Line Numbers */}
                  <div className="absolute left-0 top-4 bottom-4 w-8 text-right pr-2 text-gray-600 select-none border-r border-gray-700 leading-6">
                    {selectedRule.script.split('\n').map((_, i) => <div key={i}>{i + 1}</div>)}
                  </div>
                  <pre className="pl-10 leading-6 whitespace-pre-wrap">
                    <code>
                      <span className="text-green-600">// Worpen Agent Automation Script (JS/Rust-dsl)</span>
                      {'\n'}
                      {selectedRule.script}
                    </code>
                  </pre>
               </div>
               
               <div className="flex justify-end gap-3 mt-4 pt-2 border-t border-green-900">
                  <button className="flex items-center gap-2 px-4 py-2 text-xs bg-gray-900 border border-gray-600 hover:bg-gray-800 text-gray-300">
                    TEST_DRY_RUN
                  </button>
                  <button className="flex items-center gap-2 px-4 py-2 text-xs bg-green-900 border border-green-500 hover:bg-green-700 text-white font-bold">
                    <Save size={14} /> SAVE_AND_DEPLOY
                  </button>
               </div>
             </div>
          </TerminalFrame>
        </div>
      </div>
    </div>
  );
};