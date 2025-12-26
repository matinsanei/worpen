
import React, { useState, useEffect, useRef } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_PIPELINES } from '../constants';
import { Pipeline, PipelineStage } from '../types';
import { GitBranch, PlayCircle, Clock, CheckCircle, XCircle, Loader2, ArrowRight, Terminal, Code } from 'lucide-react';

interface StageNodeProps {
  stage: PipelineStage;
  isLast: boolean;
}

const StageNode: React.FC<StageNodeProps> = ({ stage, isLast }) => {
  const getStatusColor = (s: string) => {
    switch (s) {
      case 'SUCCESS': return 'border-[#59a86940] text-[#59a869] bg-[#59a86910]';
      case 'FAILED': return 'border-[#e06c7540] text-[#e06c75] bg-[#e06c7510]';
      case 'RUNNING': return 'border-[#f2c55c40] text-[#f2c55c] bg-[#f2c55c10] animate-pulse';
      default: return 'border-[#43454a] text-[#6e7073] bg-[#2b2d30]';
    }
  };

  const getIcon = (s: string) => {
    switch (s) {
      case 'SUCCESS': return <CheckCircle size={14} />;
      case 'FAILED': return <XCircle size={14} />;
      case 'RUNNING': return <Loader2 size={14} className="animate-spin" />;
      default: return <div className="w-3 h-3 rounded-full border border-gray-600"></div>;
    }
  };

  return (
    <div className="flex items-center">
      <div className={`w-36 p-4 rounded-[var(--radius)] border ${getStatusColor(stage.status)} flex flex-col items-center gap-3 relative group cursor-pointer hover:scale-105 transition-all shadow-lg`}>
        <div className="text-[11px] font-bold uppercase tracking-wider">{stage.name}</div>
        <div className="p-2 rounded-full bg-current opacity-10 absolute inset-0 pointer-events-none"></div>
        {getIcon(stage.status)}
        <div className="text-[10px] font-bold opacity-70 uppercase tracking-tighter">{stage.duration}</div>

        {/* Tooltip */}
        <div className="absolute -bottom-10 left-1/2 -translate-x-1/2 bg-[#2b2d30] border border-[#43454a] text-[10px] font-bold text-[#dfe1e5] px-2.5 py-1.5 opacity-0 group-hover:opacity-100 transition-all z-20 whitespace-nowrap rounded shadow-2xl pointer-events-none">
          ID: {stage.id} // STATUS: {stage.status}
        </div>
      </div>
      {!isLast && (
        <div className="px-4 text-[#43454a] flex flex-col items-center gap-1">
          <ArrowRight size={20} className="opacity-50" />
          <div className="w-8 h-[1px] bg-[#43454a] opacity-30"></div>
        </div>
      )}
    </div>
  );
};

const ConsoleLog = ({ pipelineId }: { pipelineId: string }) => {
  const [logs, setLogs] = useState<string[]>([]);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setLogs([`> Initializing environment for ${pipelineId}...`]);

    const interval = setInterval(() => {
      setLogs(prev => {
        if (prev.length > 20) return prev.slice(1);
        const newLog = `[${new Date().toLocaleTimeString()}] ${Math.random() > 0.8 ? 'INFO: Layer cached' : 'DEBUG: Processing step...'}`;
        return [...prev, newLog];
      });
    }, 1200);

    return () => clearInterval(interval);
  }, [pipelineId]);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div ref={scrollRef} className="h-full bg-[#1e1f22] font-mono text-[11px] p-4 overflow-y-auto space-y-1.5 text-[#dfe1e5] border border-[#43454a] rounded-[var(--radius)] shadow-inner custom-scrollbar">
      {logs.map((l, i) => {
        const isInfo = l.includes('INFO');
        const isDebug = l.includes('DEBUG');
        const isInit = l.includes('Initializing');
        return (
          <div key={i} className="flex gap-3 leading-relaxed">
            <span className="text-[#6e7073] select-none">{(i + 1).toString().padStart(2, '0')}</span>
            <span className={`
                          ${isInfo ? 'text-[#59a869]' : isDebug ? 'text-[#6e7073]' : isInit ? 'text-[#3574f0] font-bold' : ''}
                      `}>{l}</span>
          </div>
        );
      })}
      <div className="flex gap-3 items-center">
        <span className="text-[#6e7073] select-none">{(logs.length + 1).toString().padStart(2, '0')}</span>
        <span className="animate-pulse text-[#3574f0] font-bold">_</span>
      </div>
    </div>
  );
};

export const CicdView: React.FC = () => {
  const [selectedPipeline, setSelectedPipeline] = useState<Pipeline>(MOCK_PIPELINES[0]);
  const [isRunning, setIsRunning] = useState(false);

  const triggerRun = () => {
    setIsRunning(true);
    setTimeout(() => setIsRunning(false), 3000);
  };

  return (
    <div className="h-full p-6 flex flex-col gap-6 max-w-[1600px] mx-auto overflow-hidden">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-3 text-[#dfe1e5] tracking-tight">
            <div className="p-2 rounded-[var(--radius)] bg-[#3574f015] border border-[#3574f020]">
              <GitBranch className="text-[#3574f0]" size={20} />
            </div>
            Pipeline Runner
          </h1>
          <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">Continuous Integration & Deployment Matrix</p>
        </div>
        <button
          onClick={triggerRun}
          className={`px-5 py-2.5 rounded-[var(--radius)] text-[11px] flex items-center gap-2 font-bold transition-all uppercase tracking-tight shadow-lg border
            ${isRunning
              ? 'bg-[#2b2d30] border-[#43454a] text-[#f2c55c] cursor-not-allowed'
              : 'bg-[#59a869] border-[#59a869] text-white hover:bg-[#59a869e0] active:scale-95'
            }`}
        >
          {isRunning ? <Loader2 size={14} className="animate-spin" /> : <PlayCircle size={14} />}
          {isRunning ? 'Dispatching...' : 'Trigger Pipeline'}
        </button>
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">

        {/* Sidebar: Pipeline List */}
        <div className="lg:col-span-3 flex flex-col gap-3 overflow-y-auto pr-2 custom-scrollbar">
          {MOCK_PIPELINES.map(pipe => (
            <div
              key={pipe.id}
              onClick={() => setSelectedPipeline(pipe)}
              className={`jb-card p-4 cursor-pointer transition-all group relative overflow-hidden ${selectedPipeline.id === pipe.id
                ? 'border-[#3574f0] bg-[#3574f008]'
                : 'hover:border-[#6a6e75] hover:bg-[#393b4040]'
                }`}
            >
              <div className="flex justify-between items-start mb-2">
                <span className={`font-bold text-[13px] truncate transition-colors ${selectedPipeline.id === pipe.id ? 'text-[#3574f0]' : 'text-[#dfe1e5]'}`}>
                  {pipe.name}
                </span>
                {pipe.status === 'RUNNING' && <div className="w-2 h-2 rounded-full bg-[#f2c55c] shadow-[0_0_8px_rgba(242,197,92,0.4)] animate-pulse"></div>}
                {pipe.status === 'FAILED' && <div className="w-2 h-2 rounded-full bg-[#e06c75] shadow-[0_0_8px_rgba(224,108,117,0.4)]"></div>}
                {pipe.status === 'SUCCESS' && <div className="w-2 h-2 rounded-full bg-[#59a869] shadow-[0_0_8px_rgba(89,168,105,0.4)]"></div>}
              </div>
              <div className="flex items-center gap-2 text-[10px] text-[#6e7073] font-bold uppercase mb-3 px-1">
                <GitBranch size={12} className="opacity-50" /> {pipe.branch}
              </div>
              <div className="text-[10px] font-mono text-[#6e7073] truncate border-t border-[#43454a] pt-2 px-1 italic">
                {pipe.commit}
              </div>
              {selectedPipeline.id === pipe.id && (
                <div className="absolute top-0 left-0 w-1 h-full bg-[#3574f0]"></div>
              )}
            </div>
          ))}
        </div>

        {/* Main Content */}
        <div className="lg:col-span-9 flex flex-col gap-6 overflow-hidden">
          {/* Pipeline Visualizer */}
          <div className="jb-card flex flex-col min-h-[250px] shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
              <div className="flex items-center gap-2">
                <Code size={14} className="text-[#3574f0]" />
                <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Flow Analysis: {selectedPipeline.name}</span>
              </div>
              <div className="flex gap-5 text-[10px] font-bold text-[#6e7073] uppercase tracking-tighter">
                <span className="flex items-center gap-1.5"><Terminal size={12} className="text-[#3574f0] opacity-70" /> AUTHOR: {selectedPipeline.author}</span>
                <span className="flex items-center gap-1.5"><Clock size={12} className="opacity-70" /> UPDATED: {selectedPipeline.lastRun}</span>
              </div>
            </div>

            <div className="flex-1 flex items-center justify-center p-8 bg-[#1e1f22]/30 overflow-x-auto custom-scrollbar">
              <div className="flex items-center">
                {/* Start Node */}
                <div className="mr-6 flex flex-col items-center gap-2">
                  <div className="w-10 h-10 rounded-full border-2 border-[#43454a] flex items-center justify-center text-[#3574f0] bg-[#3574f010] shadow-xl">
                    <GitBranch size={18} />
                  </div>
                  <span className="text-[10px] font-bold text-[#6e7073] uppercase tracking-widest">Git Push</span>
                </div>
                <div className="px-4 text-[#43454a]">
                  <ArrowRight size={24} className="opacity-30" />
                </div>

                {selectedPipeline.stages.map((stage, idx) => (
                  <StageNode
                    key={stage.id}
                    stage={stage}
                    isLast={idx === selectedPipeline.stages.length - 1}
                  />
                ))}
              </div>
            </div>
          </div>

          {/* Live Logs */}
          <div className="jb-card flex-1 flex flex-col min-h-[300px] overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
              <div className="flex items-center gap-2">
                <Terminal size={14} className="text-[#3574f0]" />
                <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Runner Console Output</span>
              </div>
              <span className="text-[10px] font-bold text-[#6e7073] uppercase italic">worker-active: worpen-isolated-01</span>
            </div>
            <div className="flex-1 p-4 bg-[#1e1f22] overflow-hidden flex flex-col">
              <div className="flex-1 min-h-0 relative">
                <div className="absolute inset-0">
                  <ConsoleLog pipelineId={selectedPipeline.id} />
                </div>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
};
