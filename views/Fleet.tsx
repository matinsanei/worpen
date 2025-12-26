
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

      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
        <div>
          <h1 className="text-2xl font-bold text-[#dfe1e5] tracking-tight mb-1">Fleet Management</h1>
          <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">
            {loading ? 'STATUS: INITIALIZING...' : `ACTIVE NODES: ${agents.length}`}
          </p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={handleSync}
            className="bg-[#2b2d30] hover:bg-[#393b40] border border-[#43454a] text-[#dfe1e5] text-[11px] font-bold px-4 py-2 rounded-[var(--radius)] transition-all flex items-center gap-2 uppercase tracking-tight"
          >
            <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
            Sync Fleet
          </button>
          <button className="bg-[#3574f0] hover:bg-[#3574f0e0] text-white text-[11px] font-bold px-5 py-2 rounded-[var(--radius)] shadow-lg transition-all active:scale-95 uppercase tracking-tight border border-[#3574f0]">
            Deploy Node
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        {agents.map((agent) => {
          const isOnline = agent.status === 'ONLINE';
          const isCrit = agent.cpuLoad > 90 || agent.memoryLoad > 90;

          return (
            <div key={agent.id} className="jb-card relative overflow-hidden group">
              <div className="p-5 flex flex-col gap-5">

                {/* Header */}
                <div className="flex justify-between items-start">
                  <div className="flex items-center gap-4">
                    <div className={`w-12 h-12 rounded-[var(--radius)] flex items-center justify-center text-lg font-bold shadow-inner transition-colors
                                ${isOnline ? 'bg-[#59a86915] text-[#59a869] border border-[#59a86930]' :
                        isCrit ? 'bg-[#e06c7515] text-[#e06c75] border border-[#e06c7530]' :
                          'bg-[#f2c55c15] text-[#f2c55c] border border-[#f2c55c30]'}
                             `}>
                      {agent.id.split('-')[1]}
                    </div>
                    <div>
                      <div className="font-bold text-[#dfe1e5] text-[15px] group-hover:text-[#3574f0] transition-colors">{agent.name}</div>
                      <div className="flex items-center gap-2 text-[10px] text-[#6e7073] font-bold uppercase mt-1">
                        <span className={`w-2 h-2 rounded-full ${isOnline ? 'bg-[#59a869] shadow-[0_0_8px_rgba(89,168,105,0.4)]' : 'bg-[#e06c75] shadow-[0_0_8px_rgba(224,108,117,0.4)]'}`}></span>
                        {agent.ip}
                      </div>
                    </div>
                  </div>
                  <button className="text-[#6e7073] hover:text-[#dfe1e5] transition-colors p-1 hover:bg-[#1e1f22] rounded">
                    <MoreHorizontal size={20} />
                  </button>
                </div>

                {/* Metrics */}
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-[#1e1f22]/50 rounded-[var(--radius)] p-3 border border-[#43454a]/50">
                    <div className="flex items-center gap-2 text-[10px] font-bold text-[#6e7073] uppercase mb-2 tracking-wider">
                      <Cpu size={14} className="text-[#3574f0]" /> CPU
                    </div>
                    <div className="text-xl font-bold text-[#dfe1e5] font-mono">{agent.cpuLoad}%</div>
                    <div className="w-full bg-[#1e1f22] h-1.5 mt-2.5 rounded-full overflow-hidden border border-[#43454a]">
                      <div className="bg-[#3574f0] h-full shadow-[0_0_8px_rgba(53,116,240,0.4)] transition-all duration-700" style={{ width: `${agent.cpuLoad}%` }}></div>
                    </div>
                  </div>
                  <div className="bg-[#1e1f22]/50 rounded-[var(--radius)] p-3 border border-[#43454a]/50">
                    <div className="flex items-center gap-2 text-[10px] font-bold text-[#6e7073] uppercase mb-2 tracking-wider">
                      <HardDrive size={14} className="text-[#a682e6]" /> MEM
                    </div>
                    <div className="text-xl font-bold text-[#dfe1e5] font-mono">{agent.memoryLoad}%</div>
                    <div className="w-full bg-[#1e1f22] h-1.5 mt-2.5 rounded-full overflow-hidden border border-[#43454a]">
                      <div className="bg-[#a682e6] h-full shadow-[0_0_8px_rgba(166,130,230,0.4)] transition-all duration-700" style={{ width: `${agent.memoryLoad}%` }}></div>
                    </div>
                  </div>
                </div>

                {/* Footer */}
                <div className="flex items-center justify-between pt-4 border-t border-[#43454a] text-[11px] font-bold text-[#6e7073] uppercase">
                  <div className="flex items-center gap-4">
                    <span className="flex items-center gap-1.5"><Terminal size={12} className="text-[#3574f0] opacity-70" /> v{agent.version}</span>
                    <span className="flex items-center gap-1.5"><Signal size={12} className="text-[#59a869] opacity-70" /> {agent.peers} PEERS</span>
                  </div>
                  <div className="flex gap-1.5">
                    <button
                      onClick={() => setActiveTerminal({ id: agent.id, name: agent.name })}
                      className="p-1.5 hover:bg-[#3574f015] border border-transparent hover:border-[#3574f030] rounded-[var(--radius)] text-[#6e7073] hover:text-[#3574f0] transition-all"
                      title="Open Shell"
                    >
                      <Code size={16} />
                    </button>
                    <button className="p-1.5 hover:bg-[#dfe1e510] border border-transparent hover:border-[#dfe1e520] rounded-[var(--radius)] text-[#6e7073] hover:text-[#dfe1e5] transition-all" title="Restart">
                      <RefreshCw size={16} />
                    </button>
                    <button className="p-1.5 hover:bg-[#e06c7515] border border-transparent hover:border-[#e06c7530] rounded-[var(--radius)] text-[#6e7073] hover:text-[#e06c75] transition-all" title="Shutdown">
                      <Power size={16} />
                    </button>
                  </div>
                </div>

              </div>
              <div className={`absolute bottom-0 left-0 h-[2px] w-0 group-hover:w-full transition-all duration-500 ${isOnline ? 'bg-[#59a869]' : 'bg-[#e06c75]'}`}></div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
