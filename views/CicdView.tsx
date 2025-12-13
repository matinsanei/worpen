
import React, { useState, useEffect, useRef } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_PIPELINES } from '../constants';
import { Pipeline, PipelineStage } from '../types';
import { GitBranch, PlayCircle, Clock, CheckCircle, XCircle, Loader2, ArrowRight, Terminal } from 'lucide-react';

const StageNode = ({ stage, isLast }: { stage: PipelineStage, isLast: boolean }) => {
  const getStatusColor = (s: string) => {
    switch(s) {
      case 'SUCCESS': return 'border-green-500 text-green-400 bg-green-900/20';
      case 'FAILED': return 'border-red-500 text-red-400 bg-red-900/20';
      case 'RUNNING': return 'border-yellow-500 text-yellow-400 bg-yellow-900/20 animate-pulse';
      default: return 'border-gray-700 text-gray-600 bg-gray-900/10';
    }
  };

  const getIcon = (s: string) => {
    switch(s) {
      case 'SUCCESS': return <CheckCircle size={14} />;
      case 'FAILED': return <XCircle size={14} />;
      case 'RUNNING': return <Loader2 size={14} className="animate-spin" />;
      default: return <div className="w-3 h-3 rounded-full border border-gray-600"></div>;
    }
  };

  return (
    <div className="flex items-center">
      <div className={`w-32 p-3 border ${getStatusColor(stage.status)} flex flex-col items-center gap-2 relative group cursor-pointer hover:brightness-125 transition-all`}>
        <div className="text-[10px] font-bold tracking-wider">{stage.name}</div>
        {getIcon(stage.status)}
        <div className="text-[9px] font-mono opacity-70">{stage.duration}</div>
        
        {/* Tooltip */}
        <div className="absolute -bottom-8 bg-black border border-green-800 text-[9px] px-2 py-1 opacity-0 group-hover:opacity-100 transition-opacity z-10 whitespace-nowrap">
           ID: {stage.id} // STATUS: {stage.status}
        </div>
      </div>
      {!isLast && (
        <div className="px-2 text-gray-600">
           <ArrowRight size={16} />
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
    <div ref={scrollRef} className="h-full bg-black font-mono text-xs p-2 overflow-y-auto space-y-1 text-gray-400 border border-green-900/30">
        {logs.map((l, i) => <div key={i}>{l}</div>)}
        <div className="animate-pulse text-green-500">_</div>
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
    <div className="h-full p-2 flex flex-col gap-4">
      {/* Header */}
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2">
        <div>
           <h1 className="text-2xl font-bold flex items-center gap-2">
             <GitBranch className="text-pink-400" /> 
             PIPELINE_RUNNER
           </h1>
           <p className="text-xs text-green-600">CONTINUOUS INTEGRATION & DEPLOYMENT MATRIX</p>
        </div>
        <button 
            onClick={triggerRun}
            className={`px-4 py-1 border text-xs flex items-center gap-2 font-bold
            ${isRunning ? 'bg-yellow-900/50 border-yellow-500 text-yellow-200 cursor-not-allowed' : 'bg-green-900 border-green-500 text-white hover:bg-green-700'}`}
        >
           {isRunning ? <Loader2 size={14} className="animate-spin"/> : <PlayCircle size={14} />}
           {isRunning ? 'DISPATCHING...' : 'TRIGGER_PIPELINE'}
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4 flex-1 min-h-0">
         
         {/* Sidebar: Pipeline List */}
         <div className="lg:col-span-1 flex flex-col gap-2 overflow-y-auto pr-1">
            {MOCK_PIPELINES.map(pipe => (
                <div 
                    key={pipe.id}
                    onClick={() => setSelectedPipeline(pipe)}
                    className={`p-3 border cursor-pointer transition-all group relative overflow-hidden
                        ${selectedPipeline.id === pipe.id 
                            ? 'bg-green-900/20 border-green-500' 
                            : 'bg-black border-green-900/50 hover:border-green-600'
                        }`}
                >
                    <div className="flex justify-between items-start mb-1">
                        <span className={`font-bold text-xs truncate ${selectedPipeline.id === pipe.id ? 'text-white' : 'text-green-300'}`}>
                            {pipe.name.toUpperCase()}
                        </span>
                        {pipe.status === 'RUNNING' && <div className="w-2 h-2 rounded-full bg-yellow-500 animate-pulse"></div>}
                        {pipe.status === 'FAILED' && <div className="w-2 h-2 rounded-full bg-red-500"></div>}
                        {pipe.status === 'SUCCESS' && <div className="w-2 h-2 rounded-full bg-green-500"></div>}
                    </div>
                    <div className="flex items-center gap-1 text-[10px] text-gray-500 mb-2">
                         <GitBranch size={10} /> {pipe.branch}
                    </div>
                    <div className="text-[9px] font-mono text-gray-600 truncate border-t border-green-900/30 pt-1">
                        {pipe.commit}
                    </div>
                </div>
            ))}
         </div>

         {/* Main Content */}
         <div className="lg:col-span-3 flex flex-col gap-4">
             {/* Pipeline Visualizer */}
             <TerminalFrame title={`FLOW: ${selectedPipeline.name}`} className="min-h-[200px]">
                <div className="flex flex-col h-full">
                    <div className="flex justify-between items-start mb-6 text-[10px] text-gray-400">
                        <div>
                            AUTHOR: <span className="text-green-400">{selectedPipeline.author}</span>
                        </div>
                        <div className="flex items-center gap-1">
                            <Clock size={10} /> LAST_RUN: {selectedPipeline.lastRun}
                        </div>
                    </div>
                    
                    <div className="flex-1 flex items-center justify-center overflow-x-auto pb-4">
                        <div className="flex items-center">
                            {/* Start Node */}
                            <div className="mr-4 text-gray-600 text-[10px] font-mono">GIT_PUSH</div>
                            <ArrowRight size={16} className="text-gray-700 mr-4" />
                            
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
             </TerminalFrame>

             {/* Live Logs */}
             <TerminalFrame title="BUILD_CONSOLE_OUTPUT" className="flex-1 min-h-[200px]">
                 <div className="flex flex-col h-full">
                    <div className="flex items-center gap-2 bg-gray-900 p-1 mb-2 text-[10px] text-gray-400">
                        <Terminal size={10} />
                        <span>runner-worker-01:~/workspace/{selectedPipeline.name}</span>
                    </div>
                    <div className="flex-1 min-h-0 relative">
                        <div className="absolute inset-0">
                            <ConsoleLog pipelineId={selectedPipeline.id} />
                        </div>
                    </div>
                 </div>
             </TerminalFrame>
         </div>

      </div>
    </div>
  );
};
