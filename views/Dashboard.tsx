
import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { StatsChart } from '../components/StatsChart';
import { MOCK_LOGS, MOCK_AGENTS } from '../constants';
import { AgentStatus } from '../types';
import { 
  Activity, Server, Zap, ShieldCheck, 
  Cpu, Network, Globe, Clock, ArrowUpRight 
} from 'lucide-react';

const KPICard = ({ title, value, label, icon: Icon, trend, color = "green" }: any) => (
  <div className="relative overflow-hidden rounded-sm bg-[#0a0a0a] border border-white/5 p-5 group hover:border-green-500/20 transition-all">
    <div className="flex justify-between items-start mb-4">
       <div className={`p-2 rounded-md bg-${color}-500/10 text-${color}-500`}>
         <Icon size={20} />
       </div>
       {trend && (
         <div className="flex items-center gap-1 text-[10px] font-mono text-green-400 bg-green-900/20 px-2 py-0.5 rounded-full">
            <ArrowUpRight size={10} /> {trend}
         </div>
       )}
    </div>
    <div>
       <div className="text-3xl font-bold text-white mb-1 tracking-tight">{value}</div>
       <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">{title}</div>
    </div>
    <div className="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-green-500/20 to-transparent transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"></div>
  </div>
);

const TopologyVis = () => {
  return (
    <div className="w-full h-full relative flex items-center justify-center overflow-hidden">
       {/* Background Radar */}
       <div className="absolute inset-0 border-[0.5px] border-green-900/30 rounded-full scale-[1.5] opacity-20"></div>
       <div className="absolute inset-0 border-[0.5px] border-green-900/30 rounded-full scale-[1] opacity-20"></div>
       <div className="absolute inset-0 border-[0.5px] border-green-900/30 rounded-full scale-[0.5] opacity-20"></div>
       
       {/* Rotating Elements */}
       <div className="absolute w-[300px] h-[300px] border border-green-500/10 rounded-full animate-[spin_10s_linear_infinite]"></div>
       <div className="absolute w-[200px] h-[200px] border border-dashed border-green-500/20 rounded-full animate-[spin_15s_linear_infinite_reverse]"></div>

       {/* Nodes */}
       {MOCK_AGENTS.map((agent, i) => {
          const angle = (i / MOCK_AGENTS.length) * 2 * Math.PI;
          const r = 100;
          const x = Math.cos(angle) * r;
          const y = Math.sin(angle) * r;
          const isOnline = agent.status === AgentStatus.ONLINE;
          
          return (
            <div 
              key={agent.id}
              className="absolute w-3 h-3 -ml-1.5 -mt-1.5 rounded-full shadow-[0_0_10px_rgba(0,0,0,0.8)] z-10 transition-transform hover:scale-150 cursor-pointer"
              style={{ 
                 transform: `translate(${x}px, ${y}px)`,
                 backgroundColor: isOnline ? '#00ff41' : '#ef4444'
              }}
              title={agent.name}
            >
               <div className={`absolute inset-0 rounded-full ${isOnline ? 'animate-ping bg-green-500' : ''} opacity-50`}></div>
            </div>
          )
       })}

       {/* Core */}
       <div className="relative z-20 w-12 h-12 bg-black border border-green-500 rounded-full flex items-center justify-center shadow-[0_0_20px_rgba(0,255,65,0.2)]">
          <Globe className="text-green-500 animate-pulse" size={20} />
       </div>
    </div>
  );
};

export const Dashboard: React.FC = () => {
  const [data, setData] = useState<any[]>([]);

  useEffect(() => {
    const points = Array.from({ length: 20 }, (_, i) => ({
      time: i,
      value: Math.floor(Math.random() * 40) + 40
    }));
    setData(points);
    
    const int = setInterval(() => {
      setData(prev => {
        const next = [...prev.slice(1)];
        next.push({ time: prev[prev.length - 1].time + 1, value: Math.floor(Math.random() * 40) + 40 });
        return next;
      });
    }, 1000);
    return () => clearInterval(int);
  }, []);

  return (
    <div className="p-6 space-y-6 max-w-[1600px] mx-auto">
      
      {/* Header Info */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
         <div>
            <h1 className="text-2xl font-bold text-white tracking-tight">System Overview</h1>
            <p className="text-sm text-gray-500">Real-time telemetry and fleet orchestration.</p>
         </div>
         <div className="flex items-center gap-4 bg-white/5 rounded-full px-4 py-1.5 border border-white/10">
            <div className="flex items-center gap-2 text-xs">
               <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
               <span className="text-green-400 font-medium">SYSTEM OPTIMAL</span>
            </div>
            <div className="h-4 w-[1px] bg-white/10"></div>
            <div className="text-xs text-gray-400 font-mono">
               UPTIME: <span className="text-white">99.998%</span>
            </div>
         </div>
      </div>

      {/* KPI Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <KPICard title="Total Nodes" value="05" icon={Server} trend="+2 New" color="blue" />
        <KPICard title="Req / Second" value="1.2k" icon={Zap} trend="+12%" color="yellow" />
        <KPICard title="Avg Load" value="42%" icon={Cpu} color="purple" />
        <KPICard title="Security" value="100%" icon={ShieldCheck} color="green" />
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 h-[500px]">
         
         {/* Main Chart */}
         <div className="lg:col-span-2 h-full flex flex-col gap-6">
            <TerminalFrame title="Cluster Load Distribution" className="flex-1">
               <div className="h-full w-full p-2">
                  <StatsChart data={data} dataKey="value" color="#00ff41" />
               </div>
            </TerminalFrame>
            
            <div className="h-1/3 grid grid-cols-2 gap-4">
               <TerminalFrame title="Network I/O" noPadding>
                  <div className="h-full flex items-center justify-center text-gray-600 text-xs">
                     <Network size={32} className="mb-2 opacity-20" />
                     {/* Placeholder for sparkline */}
                  </div>
               </TerminalFrame>
               <TerminalFrame title="Storage Pools" noPadding>
                  <div className="p-4 space-y-3">
                     <div className="space-y-1">
                        <div className="flex justify-between text-xs text-gray-400"><span>NVMe Pool A</span> <span>78%</span></div>
                        <div className="h-1.5 w-full bg-gray-800 rounded-full overflow-hidden">
                           <div className="h-full bg-blue-500 w-[78%]"></div>
                        </div>
                     </div>
                     <div className="space-y-1">
                        <div className="flex justify-between text-xs text-gray-400"><span>Backup Array</span> <span>22%</span></div>
                        <div className="h-1.5 w-full bg-gray-800 rounded-full overflow-hidden">
                           <div className="h-full bg-purple-500 w-[22%]"></div>
                        </div>
                     </div>
                  </div>
               </TerminalFrame>
            </div>
         </div>

         {/* Topology & Feed */}
         <div className="lg:col-span-1 flex flex-col gap-6">
            <TerminalFrame title="Hive Topology" className="h-[250px]" noPadding>
               <TopologyVis />
            </TerminalFrame>

            <TerminalFrame title="System Events" className="flex-1" noPadding>
               <div className="overflow-y-auto h-full p-0">
                  {MOCK_LOGS.map((log) => (
                     <div key={log.id} className="flex gap-3 p-3 border-b border-white/5 hover:bg-white/5 transition-colors text-xs">
                        <div className="mt-0.5">
                           <Clock size={12} className="text-gray-600" />
                        </div>
                        <div className="flex-1 min-w-0">
                           <div className="flex justify-between mb-0.5">
                              <span className="font-bold text-gray-300">{log.source}</span>
                              <span className="text-[10px] text-gray-600 font-mono">{log.timestamp}</span>
                           </div>
                           <p className="text-gray-500 truncate">{log.message}</p>
                        </div>
                     </div>
                  ))}
               </div>
            </TerminalFrame>
         </div>

      </div>
    </div>
  );
};
