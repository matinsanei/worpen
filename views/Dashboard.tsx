
import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { LiveLogs } from '../components/LiveLogs';
import { StatsChart } from '../components/StatsChart';
import { MOCK_LOGS, MOCK_AGENTS, MOCK_GOSSIP_LOGS, GossipLog } from '../constants';
import { AgentStatus } from '../types';
import { Activity, Server, Zap, ShieldAlert, Cpu, Network, Share2, Radio, ArrowLeftRight } from 'lucide-react';

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
        {/* Core Halo */}
        <circle cx="200" cy="150" r="130" fill="none" stroke="#00ff41" strokeWidth="0.5" strokeDasharray="2,2" className="opacity-10 animate-[spin_60s_linear_infinite]" />
        
        {/* MESH NETWORK CONNECTIONS (Side Net) */}
        {agents.map((agent, i) => {
             // Calculate position
             const count = agents.length;
             const angle = (i / count) * Math.PI * 2 - Math.PI / 2;
             const radius = 110;
             const x = 200 + Math.cos(angle) * radius;
             const y = 150 + Math.sin(angle) * radius;

             // Calculate next neighbor position for ring mesh
             const nextAngle = ((i + 1) % count / count) * Math.PI * 2 - Math.PI / 2;
             const nextX = 200 + Math.cos(nextAngle) * radius;
             const nextY = 150 + Math.sin(nextAngle) * radius;
             
             // Animated Packets on mesh
             const packetDuration = 2 + Math.random();

             return (
               <g key={`mesh-${i}`}>
                   <line 
                    x1={x} y1={y}
                    x2={nextX} y2={nextY}
                    stroke="#3b82f6"
                    strokeWidth="0.5"
                    strokeOpacity="0.4"
                    strokeDasharray="2,2"
                   />
                   {/* Moving Packet */}
                   <circle r="1.5" fill="#3b82f6" opacity="0.8">
                      <animateMotion 
                        dur={`${packetDuration}s`} 
                        repeatCount="indefinite"
                        path={`M ${x} ${y} L ${nextX} ${nextY}`}
                      />
                   </circle>
               </g>
             )
        })}

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
              {/* Core Connection Line */}
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
              
              {/* Side Net Activity Indicator (Blue Dot) */}
              {!isOffline && (
                 <circle cx={x + 8} cy={y - 8} r="1.5" fill="#3b82f6" className="animate-ping" />
              )}
              
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

// --- Gossip Log Component ---
const GossipStream = ({ logs }: { logs: GossipLog[] }) => {
    return (
        <TerminalFrame title="GOSSIP_PROTOCOL [SIDE_NET]" className="h-full">
            <div className="flex flex-col gap-2 font-mono text-[10px] md:text-xs">
                {logs.map((log) => (
                    <div key={log.id} className="flex flex-col bg-blue-900/10 border-l-2 border-blue-500 p-2">
                        <div className="flex justify-between items-center text-blue-300 mb-1">
                            <span className="flex items-center gap-1">
                                <ArrowLeftRight size={10} /> 
                                {log.from.split('-')[1]} -> {log.to.split('-')[1]}
                            </span>
                            <span className="text-gray-500">{log.latency}</span>
                        </div>
                        <div className="flex justify-between">
                            <span className="font-bold text-white">{log.type}</span>
                            <span className="text-gray-400">{log.payload}</span>
                        </div>
                    </div>
                ))}
                <div className="text-center text-blue-500 animate-pulse text-[10px] mt-2">
                    /// LISTENING TO ENCRYPTED P2P TRAFFIC...
                </div>
            </div>
        </TerminalFrame>
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
        
        {/* NEW KPI: MESH / SIDE NET */}
        <TerminalFrame title="SIDE_NET_MESH" className="h-24">
          <div className="flex items-center justify-between h-full px-2">
             <div className="flex flex-col">
              <span className="text-3xl font-bold font-mono text-blue-400">ACTIVE</span>
              <span className="text-[10px] text-blue-700">GOSSIP PROTOCOL</span>
             </div>
            <Share2 className="w-8 h-8 text-blue-800 animate-pulse" />
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
          <TerminalFrame title="CLUSTER_TOPOLOGY + MESH OVERLAY" className="flex-grow min-h-[300px]">
             <TopologyMap agents={MOCK_AGENTS} />
             <div className="absolute bottom-2 right-2 flex flex-col items-end gap-1 pointer-events-none">
                <div className="flex items-center gap-1">
                   <div className="w-2 h-0.5 bg-green-500"></div> <span className="text-[9px] text-green-500">CORE_LINK</span>
                </div>
                <div className="flex items-center gap-1">
                   <div className="w-2 h-0.5 bg-blue-500 border border-blue-500 border-dashed"></div> <span className="text-[9px] text-blue-500">SIDE_MESH</span>
                </div>
             </div>
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

        {/* Right Col: Logs & Gossip */}
        <div className="lg:col-span-1 min-h-[400px] lg:min-h-0 flex flex-col gap-4">
           {/* Top Half: Gossip Stream */}
           <div className="flex-1 min-h-0">
              <GossipStream logs={MOCK_GOSSIP_LOGS} />
           </div>
           
           {/* Bottom Half: System Logs */}
           <div className="flex-1 min-h-0">
              <LiveLogs logs={MOCK_LOGS} />
           </div>
        </div>
      </div>
    </div>
  );
};