
import React, { useState, useEffect, useRef } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_PIPELINES } from '../constants';
import { Pipeline, PipelineStage } from '../types';
import { GitBranch, PlayCircle, Clock, CheckCircle, XCircle, Loader2, ArrowRight, Terminal, Code, Cpu, Activity, ShieldCheck } from 'lucide-react';

interface StageNodeProps {
  stage: PipelineStage;
  index: number;
}

const StageNode: React.FC<StageNodeProps> = ({ stage, index }) => {
  const getStatusStyles = (s: string) => {
    switch (s) {
      case 'SUCCESS': return {
        border: 'border-[#59a86940]',
        text: 'text-[#59a869]',
        bg: 'bg-[#59a86910]',
        glow: 'shadow-[0_0_15px_rgba(89,168,105,0.15)]',
        icon: <CheckCircle size={14} className="text-[#59a869]" />
      };
      case 'FAILED': return {
        border: 'border-[#e06c7540]',
        text: 'text-[#e06c75]',
        bg: 'bg-[#e06c7510]',
        glow: 'shadow-[0_0_15px_rgba(224,108,117,0.15)]',
        icon: <XCircle size={14} className="text-[#e06c75]" />
      };
      case 'RUNNING': return {
        border: 'border-[#f2c55c60]',
        text: 'text-[#f2c55c]',
        bg: 'bg-[#f2c55c05]',
        glow: 'shadow-[0_0_20px_rgba(242,197,92,0.2)]',
        icon: <Loader2 size={14} className="text-[#f2c55c] animate-spin" />
      };
      default: return {
        border: 'border-[#43454a]',
        text: 'text-[#6e7073]',
        bg: 'bg-[#2b2d3080]',
        glow: '',
        icon: <Activity size={14} className="text-[#6e7073]" />
      };
    }
  };

  const styles = getStatusStyles(stage.status);

  return (
    <div className="relative group flex flex-col items-center">
      {/* Connection Point Left */}
      <div className="absolute left-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-[#43454a] z-10 border border-[#1e1f22]"></div>

      <div className={`w-40 min-h-[90px] p-3 rounded-lg border-2 ${styles.border} ${styles.bg} ${styles.glow} flex flex-col justify-between items-start gap-2 relative transition-all duration-300 group-hover:scale-[1.02] group-hover:border-opacity-100 backdrop-blur-sm`}>
        <div className="flex justify-between items-center w-full">
          <span className={`text-[10px] font-black uppercase tracking-[0.1em] ${styles.text}`}>
            {stage.name}
          </span>
          {styles.icon}
        </div>

        <div className="w-full space-y-2">
          <div className="flex items-center gap-1.5 text-[9px] font-mono text-[#6e7073]">
            <Clock size={10} />
            {stage.duration}
          </div>
        </div>

        {stage.status === 'RUNNING' && (
          <div className="w-full h-1 bg-[#43454a]/30 rounded-full overflow-hidden mt-1">
            <div className="h-full bg-[#f2c55c] animate-[loading-bar_1.5s_infinite_linear] w-1/3 shadow-[0_0_8px_rgba(242,197,92,0.5)]"></div>
          </div>
        )}

        {/* Technical accents */}
        <div className="absolute top-0 right-0 p-1 opacity-20">
          <Cpu size={10} className={styles.text} />
        </div>

        {/* Tooltip */}
        <div className="absolute -bottom-12 left-1/2 -translate-x-1/2 bg-[#2b2d30] border border-[#43454a] text-[9px] font-mono text-[#dfe1e5] px-2 py-1 opacity-0 group-hover:opacity-100 transition-all z-20 whitespace-nowrap rounded shadow-2xl pointer-events-none translate-y-2 group-hover:translate-y-0">
          ID: {stage.id} // STATUS: {stage.status}
        </div>
      </div>

      {/* Connection Point Right */}
      <div className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-[#43454a] z-10 border border-[#1e1f22]"></div>
    </div>
  );
};

const ConnectingLine = ({ isFirst = false }: { isFirst?: boolean }) => {
  return (
    <div className="relative w-16 h-10 flex items-center justify-center">
      <svg width="64" height="40" viewBox="0 0 64 40" fill="none" xmlns="http://www.w3.org/2000/svg" className="absolute inset-0 overflow-visible">
        <path d="M0 20H64" stroke="#43454a" strokeWidth="2" strokeDasharray="4 4" className="opacity-40" />
        <path d="M0 20H64" stroke="url(#flowGradient)" strokeWidth="2" strokeDasharray="20 44" strokeLinecap="round">
          <animate attributeName="stroke-dashoffset" from="64" to="0" dur="2s" repeatCount="indefinite" />
        </path>
        <defs>
          <linearGradient id="flowGradient" x1="0" y1="0" x2="64" y2="0" gradientUnits="userSpaceOnUse">
            <stop stopColor="#3574f0" stopOpacity="0" />
            <stop offset="0.5" stopColor="#3574f0" />
            <stop offset="1" stopColor="#3574f0" stopOpacity="0" />
          </linearGradient>
        </defs>
      </svg>
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
            <span className="text-[#6e7073] select-none text-[10px]">{(i + 1).toString().padStart(2, '0')}</span>
            <span className={`
                          ${isInfo ? 'text-[#59a869]' : isDebug ? 'text-[#6e7073]' : isInit ? 'text-[#3574f0] font-bold' : ''}
                      `}>{l}</span>
          </div>
        );
      })}
      <div className="flex gap-3 items-center">
        <span className="text-[#6e7073] select-none text-[10px]">{(logs.length + 1).toString().padStart(2, '0')}</span>
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
    <div className="min-h-full p-6 flex flex-col gap-6 max-w-[1600px] mx-auto">
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
        <div className="lg:col-span-9 flex flex-col gap-6">
          {/* Pipeline Visualizer */}
          <div className="jb-card flex flex-col min-h-[350px] shadow-2xl overflow-hidden group">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/80 backdrop-blur-md z-10">
              <div className="flex items-center gap-2">
                <Code size={14} className="text-[#3574f0]" />
                <span className="text-[11px] font-black text-[#dfe1e5] uppercase tracking-widest">Flow Analysis: {selectedPipeline.name}</span>
              </div>
              <div className="flex gap-5 text-[10px] font-bold text-[#6e7073] uppercase tracking-tighter">
                <span className="flex items-center gap-1.5"><Terminal size={12} className="text-[#3574f0] opacity-70" /> {selectedPipeline.author}</span>
                <span className="flex items-center gap-1.5"><Clock size={12} className="opacity-70" /> {selectedPipeline.lastRun}</span>
              </div>
            </div>

            <div className="flex-1 relative bg-[#1e1f22] overflow-x-auto custom-scrollbar">
              {/* Background Grid Pattern */}
              <div className="absolute inset-0 opacity-[0.03] pointer-events-none"
                style={{ backgroundImage: 'radial-gradient(#dfe1e5 1px, transparent 1px)', backgroundSize: '24px 24px' }}></div>

              <div className="absolute inset-0 flex items-center justify-center min-w-fit p-12">
                <div className="flex items-center">
                  {/* Source Node */}
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-12 h-12 rounded-xl border-2 border-[#3574f040] bg-[#3574f010] flex items-center justify-center text-[#3574f0] shadow-[0_0_20px_rgba(53,116,240,0.15)] relative group/source">
                      <GitBranch size={20} />
                      <div className="absolute inset-0 rounded-xl bg-[#3574f010] animate-pulse pointer-events-none"></div>
                    </div>
                    <span className="text-[9px] font-black text-[#6e7073] uppercase tracking-widest">vcs_trigger</span>
                  </div>

                  <ConnectingLine isFirst={true} />

                  {selectedPipeline.stages.map((stage, idx) => (
                    <React.Fragment key={stage.id}>
                      <StageNode stage={stage} index={idx} />
                      {idx < selectedPipeline.stages.length - 1 && <ConnectingLine />}
                    </React.Fragment>
                  ))}

                  {/* Output Node */}
                  <ConnectingLine />
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-12 h-12 rounded-xl border-2 border-[#59a86940] bg-[#59a86910] flex items-center justify-center text-[#59a869] shadow-[0_0_20px_rgba(89,168,105,0.15)] relative">
                      <ShieldCheck size={20} />
                    </div>
                    <span className="text-[9px] font-black text-[#6e7073] uppercase tracking-widest">ready_prod</span>
                  </div>
                </div>
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

      <style dangerouslySetInnerHTML={{
        __html: `
                @keyframes loading-bar {
                    0% { transform: translateX(-100%); }
                    100% { transform: translateX(300%); }
                }
            `}} />
    </div>
  );
};
