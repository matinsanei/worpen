import React from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_AGENTS } from '../constants';
import { AgentStatus } from '../types';
import { Cpu, HardDrive, Terminal } from 'lucide-react';

export const Fleet: React.FC = () => {
  const getStatusColor = (status: AgentStatus) => {
    switch(status) {
      case AgentStatus.ONLINE: return 'text-green-500 bg-green-900/20 border-green-700';
      case AgentStatus.OFFLINE: return 'text-gray-500 bg-gray-900/20 border-gray-700';
      case AgentStatus.HEALING: return 'text-yellow-500 bg-yellow-900/20 border-yellow-700';
      case AgentStatus.CRITICAL: return 'text-red-500 bg-red-900/20 border-red-700';
    }
  };

  return (
    <div className="h-full p-2 flex flex-col gap-4">
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2 mb-4">
        <div>
           <h1 className="text-2xl font-bold">FLEET_COMMAND</h1>
           <p className="text-xs text-green-600">MANAGE DISTRIBUTED AGENTS [RUST_BINARY_V2.0]</p>
        </div>
        <button className="px-4 py-1 bg-green-900 hover:bg-green-700 text-green-100 text-xs border border-green-500 uppercase">
           + Deploy New Agent
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 overflow-y-auto pb-10">
        {MOCK_AGENTS.map((agent) => (
          <TerminalFrame key={agent.id} title={agent.id.toUpperCase()} className="h-auto">
            <div className="flex flex-col gap-3">
              <div className="flex justify-between items-center">
                <span className="text-lg font-bold">{agent.name}</span>
                <span className={`px-2 py-0.5 text-[10px] border ${getStatusColor(agent.status)} uppercase font-bold`}>
                  {agent.status}
                </span>
              </div>
              
              <div className="grid grid-cols-2 gap-2 text-xs text-gray-400">
                 <div>IP: <span className="text-green-300">{agent.ip}</span></div>
                 <div>OS: <span className="text-green-300">{agent.os}</span></div>
                 <div>UPTIME: <span className="text-green-300">{agent.uptime}</span></div>
                 <div>VER: <span className="text-green-300">{agent.version}</span></div>
              </div>

              {/* Resource Bars */}
              <div className="space-y-2 mt-2">
                <div className="flex items-center gap-2">
                   <Cpu size={14} />
                   <div className="flex-1 h-2 bg-gray-800 border border-gray-600">
                      <div 
                        className={`h-full ${agent.cpuLoad > 80 ? 'bg-red-500' : 'bg-green-500'}`} 
                        style={{ width: `${agent.cpuLoad}%` }}
                      ></div>
                   </div>
                   <span className="text-xs w-8 text-right">{agent.cpuLoad}%</span>
                </div>
                <div className="flex items-center gap-2">
                   <HardDrive size={14} />
                   <div className="flex-1 h-2 bg-gray-800 border border-gray-600">
                      <div 
                        className={`h-full ${agent.memoryLoad > 80 ? 'bg-red-500' : 'bg-green-500'}`} 
                        style={{ width: `${agent.memoryLoad}%` }}
                      ></div>
                   </div>
                   <span className="text-xs w-8 text-right">{agent.memoryLoad}%</span>
                </div>
              </div>

              {/* Quick Actions */}
              <div className="flex gap-2 mt-4 pt-4 border-t border-green-900/50">
                <button className="flex-1 py-1 text-[10px] bg-green-900/30 hover:bg-green-800 border border-green-800 text-green-300 flex items-center justify-center gap-1">
                  <Terminal size={10} /> SSH
                </button>
                <button className="flex-1 py-1 text-[10px] bg-red-900/30 hover:bg-red-800 border border-red-800 text-red-300">
                  RESTART
                </button>
              </div>
            </div>
          </TerminalFrame>
        ))}
      </div>
    </div>
  );
};