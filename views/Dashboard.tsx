import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { LiveLogs } from '../components/LiveLogs';
import { StatsChart } from '../components/StatsChart';
import { MOCK_LOGS, MOCK_AGENTS } from '../constants';
import { AgentStatus } from '../types';
import { Activity, Server, Zap, ShieldAlert, Cpu, Network } from 'lucide-react';

// --- Topology Map Component ---
const TopologyMap = ({ agents }: { agents: typeof MOCK_AGENTS }) => {
  return (
    <div className="relative w-full h-full bg-[#050505] overflow-hidden">
      {/* Grid Background */}
      <svg className="absolute inset-0 w-full h-full opacity-20 pointer-events-none">
        <defs>
          <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
            <path d="M 40 0 L 0 0 0 40" fill="none" stroke="#004400" strokeWidth="1"/>
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#grid)" />
      </svg>

      {/* SVG Map */}
      <svg className="w-full h-full" viewBox="0 0 400 300" preserveAspectRatio="xMidYMid meet">
        {/* Center Core */}
        <g transform="translate(200, 150)">
          <circle r="15" fill="#000" stroke="#00ff41" strokeWidth="2" className="animate-pulse" />
          <circle r="8" fill="#00ff41" className="opacity-50" />
          <text x="0" y="28" textAnchor="middle" fill="#00ff41" fontSize="8" fontFamily="monospace">HIVE_CORE</text>
        </g>

        {/* Connections & Nodes */}
        {agents.map((agent, i) => {
          const count = agents.length;
          const angle = (i / count) * Math.PI * 2 - Math.PI / 2;
          const radius = 110;
          const x = 200 + Math.cos(angle) * radius;
          const y = 150 + Math.sin(angle) * radius;
          const isOffline = agent.status === AgentStatus.OFFLINE || agent.status === AgentStatus.CRITICAL;
          const isHealing = agent.status === AgentStatus.HEALING;
          
          let color = "#00ff41";
          if (isOffline) color = "#ef4444";
          if (isHealing) color = "#eab308";

          return (
            <g key={agent.id}>
              {/* Connection Line */}
              <line 
                x1="200" y1="150" 
                x2={x} y2={y} 
                stroke={color} 
                strokeWidth={isHealing ? 2 : 1}
                strokeDasharray={isOffline ? "4,4" : "none"}
                className={isHealing ? "animate-pulse" : "opacity-40"}
              />
              
              {/* Node Dot */}
              <circle cx={x} cy={y} r="6" fill="#000" stroke={color} strokeWidth="2" />
              
              {/* Node Label */}
              <text x={x} y={y + 15} textAnchor="middle" fill={color} fontSize="6" fontFamily="monospace" className="uppercase">
                {agent.name.split('-').pop()}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
};

export const Dashboard: React.FC = () => {
  // --- Stabilized Data Generation ---
  const [chartData, setChartData] = useState(() => {
    const initial = [];
    let load = 45;
    let net = 800;
    for (let i = 0; i < 30; i++) {
      load = Math.max(10, Math.min(90, load + (Math.random() * 10 - 5)));
      net = Math.max(200, Math.min(1200, net + (Math.random() * 200 - 100)));
      initial.push({ time: i, load: Math.floor(load), requests: Math.floor(net) });
    }
    return initial;
  });

  useEffect(() => {
    const interval = setInterval(() => {
      setChartData(prev => {
        const last = prev[prev.length - 1];
        const newLoad = Math.max(10, Math.min(90, last.load + (Math.random() * 8 - 4)));
        const newNet = Math.max(200, Math.min(1200, last.requests + (Math.random() * 150 - 75)));
        
        const newData = [...prev.slice(1), { 
          time: last.time + 1, 
          load: Math.floor(newLoad), 
          requests: Math.floor(newNet) 
        }];
        return newData;
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const onlineCount = MOCK_AGENTS.filter(a => a.status === AgentStatus.ONLINE).length;
  const healingCount = MOCK_AGENTS.filter(a => a.status === AgentStatus.HEALING).length;
  const criticalCount = MOCK_AGENTS.filter(a => a.status === AgentStatus.OFFLINE || a.status === AgentStatus.CRITICAL).length;

  return (
    <div className="flex flex-col gap-4 h-full">
      
      {/* 1. Header Area (ASCII) */}
      <div className="hidden xl:block opacity-60">
        <pre className="text-[10px] text-green-500 leading-none font-bold select-none whitespace-pre overflow-hidden">
{`
██     ██  ██████  ██████  ██████  ███████ ███    ██ 
██     ██ ██    ██ ██   ██ ██   ██ ██      ████   ██ 
██  █  ██ ██    ██ ██████  ██████  █████   ██ ██  ██ 
██ ███ ██ ██    ██ ██   ██ ██      ██      ██  ██ ██ 
 ███ ███   ██████  ██   ██ ██      ███████ ██   ████ 
                                                     
       THE DIGITAL NERVOUS SYSTEM // CORE: ACTIVE
`}
        </pre>
      </div>

      {/* 2. KPI Row */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 flex-shrink-0">
        <TerminalFrame title="ACTIVE_NODES" className="h-24">
          <div className="flex items-center justify-between h-full px-2">
            <div>
               <div className="text-3xl font-bold text-green-400 font-mono">{onlineCount}/{MOCK_AGENTS.length}</div>
               <div className="text-[10px] text-green-700">MESH INTEGRITY: OK</div>
            </div>
            <Server className="w-8 h-8 text-green-800" />
          </div>
        </TerminalFrame>
        
        <TerminalFrame title="HEALING_OPS" className="h-24" flashing={healingCount > 0}>
          <div className="flex items-center justify-between h-full px-2">
             <div className="flex flex-col">
              <span className={`text-3xl font-bold font-mono ${healingCount > 0 ? 'text-yellow-400' : 'text-gray-600'}`}>{healingCount}</span>
              <span className="text-[10px] text-yellow-700">AUTO-FIX</span>
             </div>
            <Activity className={`w-8 h-8 ${healingCount > 0 ? 'text-yellow-500 animate-pulse' : 'text-gray-800'}`} />
          </div>
        </TerminalFrame>

        <TerminalFrame title="THROUGHPUT" className="h-24">
           <div className="flex items-center justify-between h-full px-2">
             <div>
               <div className="text-3xl font-bold text-blue-400 font-mono">1.2<span className="text-sm">k</span></div>
               <div className="text-[10px] text-blue-700">REQ/SEC</div>
             </div>
             <Zap className="w-8 h-8 text-blue-800" />
           </div>
        </TerminalFrame>

        <TerminalFrame title="ALERTS" className="h-24" flashing={criticalCount > 0}>
           <div className="flex items-center justify-between h-full px-2">
            <div>
              <div className={`text-3xl font-bold font-mono ${criticalCount > 0 ? 'text-red-500' : 'text-gray-600'}`}>{criticalCount}</div>
              <div className="text-[10px] text-red-900">CRITICAL</div>
            </div>
            <ShieldAlert className="w-8 h-8 text-red-800" />
          </div>
        </TerminalFrame>
      </div>

      {/* 3. Main Workspace Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 flex-1 min-h-0">
        
        {/* Left Col: Map & Charts */}
        <div className="lg:col-span-2 flex flex-col gap-4 min-h-0">
          
          {/* Map Section */}
          <TerminalFrame title="CLUSTER_TOPOLOGY" className="flex-grow min-h-[300px]">
             <TopologyMap agents={MOCK_AGENTS} />
          </TerminalFrame>
          
          {/* Charts Row */}
          <div className="h-48 grid grid-cols-1 md:grid-cols-2 gap-4 flex-shrink-0">
             <TerminalFrame title="CPU_LOAD_AVG">
               <div className="flex flex-col h-full">
                 <div className="flex items-center gap-2 mb-2 text-xs text-green-700">
                    <Cpu size={12}/> <span>CLUSTER AVERAGE</span>
                 </div>
                 <div className="flex-1 min-h-0">
                   <StatsChart data={chartData} dataKey="load" color="#00ff41" />
                 </div>
               </div>
             </TerminalFrame>
             <TerminalFrame title="NET_IO_TRAFFIC">
               <div className="flex flex-col h-full">
                  <div className="flex items-center gap-2 mb-2 text-xs text-blue-700">
                    <Network size={12}/> <span>INGRESS/EGRESS</span>
                 </div>
                 <div className="flex-1 min-h-0">
                    <StatsChart data={chartData} dataKey="requests" color="#3b82f6" />
                 </div>
               </div>
             </TerminalFrame>
          </div>
        </div>

        {/* Right Col: Logs */}
        <div className="lg:col-span-1 min-h-[400px] lg:min-h-0 flex flex-col">
          <LiveLogs logs={MOCK_LOGS} />
        </div>
      </div>
    </div>
  );
};