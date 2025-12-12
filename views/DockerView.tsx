import React from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_CONTAINERS } from '../constants';
import { Box, Play, Square, RotateCw, Trash2, Terminal } from 'lucide-react';

export const DockerView: React.FC = () => {
  return (
    <div className="h-full p-2 flex flex-col gap-4">
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2">
        <div>
           <h1 className="text-2xl font-bold flex items-center gap-2">
             <Box className="text-blue-400" /> 
             DOCKER_ORCHESTRATION
           </h1>
           <p className="text-xs text-green-600">DIRECT SOCKET ACCESS // BOLLARD CRATE</p>
        </div>
        <div className="flex gap-2">
          <button className="px-3 py-1 bg-green-900/30 border border-green-600 text-xs hover:bg-green-800">
            PRUNE_SYSTEM
          </button>
          <button className="px-3 py-1 bg-green-900/30 border border-green-600 text-xs hover:bg-green-800">
            PULL_IMAGE
          </button>
        </div>
      </div>

      <TerminalFrame title="ACTIVE_CONTAINERS" className="flex-1">
        <table className="w-full text-left text-xs border-collapse">
          <thead>
            <tr className="bg-green-900/20 text-green-300 uppercase tracking-wider">
              <th className="p-3 border-b border-green-800">ID</th>
              <th className="p-3 border-b border-green-800">Image</th>
              <th className="p-3 border-b border-green-800">Name</th>
              <th className="p-3 border-b border-green-800">Node</th>
              <th className="p-3 border-b border-green-800">State</th>
              <th className="p-3 border-b border-green-800 text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="font-mono">
            {MOCK_CONTAINERS.map((container) => (
              <tr key={container.id} className="border-b border-green-900/30 hover:bg-green-900/10 group">
                <td className="p-3 text-gray-500">{container.id.substring(0,8)}</td>
                <td className="p-3 text-yellow-200">{container.image}</td>
                <td className="p-3 font-bold text-white">{container.name}</td>
                <td className="p-3 text-gray-400">{container.node}</td>
                <td className="p-3">
                  <span className={`px-2 py-0.5 text-[10px] uppercase font-bold border ${
                    container.state === 'RUNNING' 
                      ? 'border-green-600 text-green-400 bg-green-900/20' 
                      : 'border-red-600 text-red-400 bg-red-900/20'
                  }`}>
                    {container.status}
                  </span>
                </td>
                <td className="p-3 text-right">
                  <div className="flex justify-end gap-2 opacity-80 hover:opacity-100 transition-opacity">
                    <button className="flex items-center gap-1 px-2 py-1 border border-green-800 text-[10px] hover:bg-green-900 hover:text-green-300 transition-colors">
                       <Terminal size={10} /> LOGS
                    </button>
                    
                    {container.state !== 'RUNNING' ? (
                        <button className="flex items-center gap-1 px-2 py-1 border border-green-800 text-[10px] hover:bg-green-900 hover:text-green-400 transition-colors">
                           <Play size={10} /> START
                        </button>
                    ) : (
                        <button className="flex items-center gap-1 px-2 py-1 border border-yellow-900 text-[10px] text-yellow-600 hover:bg-yellow-900/20 hover:text-yellow-400 transition-colors">
                           <Square size={10} /> STOP
                        </button>
                    )}
                    
                    <button className="flex items-center gap-1 px-2 py-1 border border-red-900 text-[10px] text-red-600 hover:bg-red-900/20 hover:text-red-400 transition-colors">
                       <Trash2 size={10} /> RM
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </TerminalFrame>
      
      {/* Mini Console Preview */}
      <div className="h-32 bg-black border border-green-800 p-2 font-mono text-xs overflow-y-auto">
        <div className="text-gray-500">{'root@hive-core:~# docker ps --format "table {{.ID}}\\t{{.Image}}\\t{{.Status}}"'}</div>
        {MOCK_CONTAINERS.map(c => (
           <div key={c.id} className="text-green-700 whitespace-pre">
             {`${c.id.substring(0,8).padEnd(15)} ${c.image.padEnd(30)} ${c.status}`}
           </div>
        ))}
        <div className="flex gap-1 mt-1">
          <span className="text-green-500">root@hive-core:~#</span>
          <span className="animate-pulse block w-2 h-4 bg-green-500"></span>
        </div>
      </div>
    </div>
  );
};