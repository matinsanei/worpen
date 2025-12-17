
import React, { useEffect, useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { InteractiveTerminal } from '../components/InteractiveTerminal';
import { MOCK_AGENTS } from '../constants';
import { AgentStatus } from '../types';
import { Cpu, HardDrive, Terminal, MoreHorizontal, Power, RefreshCw, Signal, Code } from 'lucide-react';
import { agentsApi } from '../api';

export const Fleet: React.FC = () => {
  const [agents, setAgents] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTerminal, setActiveTerminal] = useState<{
    id: string;
    name: string;
  } | null>(null);

  useEffect(() => {
    const fetchAgents = async () => {
      try {
        const data = await agentsApi.list();
        setAgents(data as any[]);
      } catch (error) {
        console.error('Failed to fetch agents:', error);
        setAgents(MOCK_AGENTS); // Fallback to mock data
      } finally {
        setLoading(false);
      }
    };

    fetchAgents();
    const interval = setInterval(fetchAgents, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  const handleSync = async () => {
    try {
      await agentsApi.sync();
      const data = await agentsApi.list();
      setAgents(data as any[]);
    } catch (error) {
      console.error('Failed to sync agents:', error);
    }
  };

  if (activeTerminal) {
    return (
      <div className="h-full p-6">
        <InteractiveTerminal
          type="agent"
          targetId={activeTerminal.id}
          targetName={activeTerminal.name}
          onClose={() => setActiveTerminal(null)}
        />
      </div>
    );
  }

  return (
    <div className="p-6 max-w-[1600px] mx-auto space-y-6">
      
      <div className="flex items-center justify-between">
         <div>
            <h1 className="text-2xl font-bold text-white">Fleet Management</h1>
            <p className="text-sm text-gray-500">
              {loading ? 'Loading...' : `Manage ${agents.length} active edge nodes.`}
            </p>
         </div>
         <div className="flex gap-2">
            <button 
              onClick={handleSync}
              className="bg-blue-600/20 hover:bg-blue-600/30 border border-blue-500/30 text-blue-400 text-sm font-medium px-4 py-2 rounded-md transition-all flex items-center gap-2"
            >
              <RefreshCw size={14} />
              Sync Fleet
            </button>
            <button className="bg-green-600 hover:bg-green-500 text-white text-sm font-medium px-4 py-2 rounded-md shadow-[0_0_15px_rgba(0,255,65,0.3)] transition-all">
              Deploy Node
            </button>
         </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {agents.map((agent) => {
          const isOnline = agent.status === AgentStatus.ONLINE;
          const isCrit = agent.status === AgentStatus.CRITICAL || agent.status === AgentStatus.OFFLINE;

          return (
            <TerminalFrame key={agent.id} noPadding className="h-full">
              <div className="p-5 flex flex-col gap-4">
                 
                 {/* Header */}
                 <div className="flex justify-between items-start">
                    <div className="flex items-center gap-3">
                       <div className={`w-10 h-10 rounded-md flex items-center justify-center text-lg font-bold
                          ${isOnline ? 'bg-green-500/10 text-green-500 border border-green-500/20' : 
                            isCrit ? 'bg-red-500/10 text-red-500 border border-red-500/20' : 
                            'bg-yellow-500/10 text-yellow-500 border border-yellow-500/20'}
                       `}>
                          {agent.id.split('-')[1]}
                       </div>
                       <div>
                          <div className="font-bold text-white text-sm">{agent.name}</div>
                          <div className="flex items-center gap-1.5 text-[10px] text-gray-500 font-mono mt-0.5">
                             <span className={`w-1.5 h-1.5 rounded-full ${isOnline ? 'bg-green-500' : 'bg-red-500'}`}></span>
                             {agent.ip}
                          </div>
                       </div>
                    </div>
                    <button className="text-gray-600 hover:text-white transition-colors">
                       <MoreHorizontal size={20} />
                    </button>
                 </div>

                 {/* Metrics */}
                 <div className="grid grid-cols-2 gap-3 py-2">
                    <div className="bg-white/5 rounded p-2 border border-white/5">
                       <div className="flex items-center gap-2 text-xs text-gray-400 mb-2">
                          <Cpu size={14} /> CPU
                       </div>
                       <div className="text-lg font-mono font-bold text-white">{agent.cpuLoad}%</div>
                       <div className="w-full bg-gray-800 h-1 mt-2 rounded-full overflow-hidden">
                          <div className="bg-blue-500 h-full" style={{ width: `${agent.cpuLoad}%` }}></div>
                       </div>
                    </div>
                    <div className="bg-white/5 rounded p-2 border border-white/5">
                       <div className="flex items-center gap-2 text-xs text-gray-400 mb-2">
                          <HardDrive size={14} /> MEM
                       </div>
                       <div className="text-lg font-mono font-bold text-white">{agent.memoryLoad}%</div>
                       <div className="w-full bg-gray-800 h-1 mt-2 rounded-full overflow-hidden">
                          <div className="bg-purple-500 h-full" style={{ width: `${agent.memoryLoad}%` }}></div>
                       </div>
                    </div>
                 </div>

                 {/* Footer */}
                 <div className="flex items-center justify-between pt-4 border-t border-white/5 text-xs text-gray-500">
                    <div className="flex items-center gap-4">
                       <span className="flex items-center gap-1"><Terminal size={12}/> v{agent.version}</span>
                       <span className="flex items-center gap-1"><Signal size={12}/> {agent.peers} peers</span>
                    </div>
                    <div className="flex gap-1">
                       <button 
                          onClick={() => setActiveTerminal({ id: agent.id, name: agent.name })}
                          className="p-1.5 hover:bg-green-500/20 rounded text-gray-400 hover:text-green-500 transition-colors" 
                          title="Open Shell"
                       >
                          <Code size={14} />
                       </button>
                       <button className="p-1.5 hover:bg-white/10 rounded text-gray-400 hover:text-white" title="Restart">
                          <RefreshCw size={14} />
                       </button>
                       <button className="p-1.5 hover:bg-red-500/20 rounded text-gray-400 hover:text-red-500" title="Shutdown">
                          <Power size={14} />
                       </button>
                    </div>
                 </div>

              </div>
            </TerminalFrame>
          );
        })}
      </div>
    </div>
  );
};
