
import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { LiveLogs } from '../components/LiveLogs';
import { StatsChart } from '../components/StatsChart';
import { MOCK_LOGS, MOCK_AGENTS, MOCK_GOSSIP_LOGS, GossipLog } from '../constants';
import { AgentStatus } from '../types';
import { Activity, Server, Zap, ShieldAlert, Cpu, Network, Share2, ArrowLeftRight, Lock, Fingerprint, ShieldCheck, FileKey, BarChart3 } from 'lucide-react';

// --- VISUAL WIDGETS ---

const SecurityWidget = () => (
  <div className="absolute top-4 right-4 bg-black/90 border border-green-700 p-3 w-48 shadow-[0_0_15px_rgba(0,255,0,0.1)] backdrop-blur-sm z-20">
    <div className="flex items-center justify-between border-b border-green-800 pb-2 mb-2">
      <div className="flex items-center gap-2 text-xs font-bold text-green-400">
        <ShieldCheck size={14} /> SEC_DAEMON
      </div>
      <div className="w-2 h-2 bg-green-500 animate-pulse rounded-full"></div>
    </div>
    <div className="space-y-2 font-mono text-[9px]">
      <div className="flex justify-between text-gray-400">
        <span>ENCRYPTION</span>
        <span className="text-green-300">AES-256-GCM</span>
      </div>
      <div className="flex justify-between text-gray-400">
        <span>FIREWALL</span>
        <span className="text-green-300">HARDENED</span>
      </div>
      <div className="mt-2">
        <div className="flex justify-between text-gray-500 mb-1">
          <span>THREAT_LEVEL</span>
          <span className="text-green-500">LOW</span>
        </div>
        <div className="h-1 w-full bg-gray-800 flex gap-0.5">
           <div className="h-full w-[10%] bg-green-500"></div>
           <div className="h-full w-[10%] bg-green-500"></div>
           <div className="h-full w-[10%] bg-green-500"></div>
           <div className="h-full w-[10%] bg-gray-700"></div>
           <div className="h-full w-[10%] bg-gray-700"></div>
        </div>
      </div>
    </div>
  </div>
);

const LicenseWidget = () => (
  <div className="absolute bottom-4 left-4 bg-black/90 border-l-2 border-green-600 p-2 w-40 backdrop-blur-sm z-20">
     <div className="flex items-center gap-2 text-[10px] font-bold text-green-500 mb-1">
        <FileKey size={12} /> LICENSE_KEY
     </div>
     <div className="font-mono text-[9px] text-gray-500 break-all leading-tight">
        XJ9-2249-KLP-001
        <br/>
        <span className="text-gray-700">SIG: 7f8a91b...</span>
     </div>
     <div className="mt-1 flex items-center gap-1 text-[9px] text-green-800">
        <Fingerprint size={10} /> AUTH_VERIFIED
     </div>
  </div>
);

const SpectrumGraph = () => {
  // Mock bars for a visual spectrum analyzer look
  const bars = Array.from({ length: 20 }).map((_, i) => ({
    height: Math.floor(Math.random() * 80) + 20,
    color: i % 3 === 0 ? '#a855f7' : i % 2 === 0 ? '#3b82f6' : '#eab308' // Purple, Blue, Yellow
  }));

  return (
    <div className="absolute bottom-4 right-4 flex items-end gap-[2px] h-16 opacity-80 z-10 pointer-events-none">
      {bars.map((bar, i) => (
        <div 
          key={i} 
          className="w-1.5 animate-pulse"
          style={{ 
            height: `${bar.height}%`, 
            backgroundColor: bar.color,
            animationDuration: `${Math.random() * 1 + 0.5}s`
          }}
        ></div>
      ))}
    </div>
  );
}

// --- TOPOLOGY MAP ---

const TopologyMap = ({ agents }: { agents: typeof MOCK_AGENTS }) => {
  return (
    <div className="relative w-full h-full bg-[#020202] overflow-hidden group">
      
      {/* 1. Cyberpunk Hex Grid Background */}
      <svg className="absolute inset-0 w-full h-full opacity-10 pointer-events-none">
        <defs>
          <pattern id="hex-grid" width="40" height="34.6" patternUnits="userSpaceOnUse" patternTransform="scale(1.5)">
             <path d="M20 0L40 11.5L40 34.6L20 46.1L0 34.6L0 11.5Z" fill="none" stroke="#00ff41" strokeWidth="0.5"/>
          </pattern>
          <radialGradient id="radar-gradient" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
            <stop offset="0%" stopColor="rgba(0, 255, 65, 0)" />
            <stop offset="50%" stopColor="rgba(0, 255, 65, 0.1)" />
            <stop offset="100%" stopColor="rgba(0, 255, 65, 0.4)" />
          </radialGradient>
        </defs>
        <rect width="100%" height="100%" fill="url(#hex-grid)" />
      </svg>

      {/* 2. Radar Sweep Animation */}
      <div className="absolute inset-0 flex items-center justify-center pointer-events-none opacity-30">
         <div className="w-[600px] h-[600px] rounded-full border border-green-900/50 bg-[conic-gradient(from_0deg,transparent_0deg,rgba(0,255,65,0.1)_60deg,rgba(0,255,65,0.5)_90deg,transparent_91deg)] animate-[spin_4s_linear_infinite]"></div>
      </div>

      {/* 3. Overlay Widgets */}
      <SecurityWidget />
      <LicenseWidget />
      <SpectrumGraph />

      {/* 4. Main SVG Network */}
      <svg className="w-full h-full relative z-10" viewBox="0 0 400 300" preserveAspectRatio="xMidYMid meet">
        
        {/* Central Core Reactor */}
        <g transform="translate(200, 150)">
           {/* Outer rotating rings */}
           <circle r="40" fill="none" stroke="#004400" strokeWidth="1" strokeDasharray="10,5" className="animate-[spin_10s_linear_infinite_reverse]" />
           <circle r="50" fill="none" stroke="#003300" strokeWidth="0.5" className="animate-pulse" />
           <circle r="30" fill="none" stroke="#00ff41" strokeWidth="1" strokeDasharray="50,20" className="animate-[spin_3s_linear_infinite]" />
           
           {/* Inner Core */}
           <circle r="15" fill="#000" stroke="#00ff41" strokeWidth="2" className="animate-pulse shadow-[0_0_20px_#00ff41]" />
           <circle r="5" fill="#00ff41" className="animate-ping opacity-75" />
           
           {/* Core Text */}
           <text x="0" y="4" textAnchor="middle" fill="#00ff41" fontSize="6" fontFamily="monospace" fontWeight="bold">CORE</text>
        </g>

        {/* MESH NETWORK CONNECTIONS (Side Net) */}
        {agents.map((agent, i) => {
             // Calculate position
             const count = agents.length;
             const angle = (i / count) * Math.PI * 2 - Math.PI / 2;
             const radius = 100;
             const x = 200 + Math.cos(angle) * radius;
             const y = 150 + Math.sin(angle) * radius;

             // Next neighbor for ring mesh
             const nextAngle = ((i + 1) % count / count) * Math.PI * 2 - Math.PI / 2;
             const nextX = 200 + Math.cos(nextAngle) * radius;
             const nextY = 150 + Math.sin(nextAngle) * radius;
             
             // Animated Packets
             const packetDuration = 2 + Math.random();

             return (
               <g key={`mesh-${i}`}>
                   {/* Ring Connection */}
                   <line 
                    x1={x} y1={y}
                    x2={nextX} y2={nextY}
                    stroke="#3b82f6"
                    strokeWidth="0.5"
                    strokeOpacity="0.3"
                   />
                   {/* Fast Data Packet on Ring */}
                   <circle r="1" fill="#fff">
                      <animateMotion 
                        dur={`${packetDuration}s`} 
                        repeatCount="indefinite"
                        path={`M ${x} ${y} L ${nextX} ${nextY}`}
                      />
                   </circle>
               </g>
             )
        })}

        {/* Nodes & Connections to Core */}
        {agents.map((agent, i) => {
          const count = agents.length;
          const angle = (i / count) * Math.PI * 2 - Math.PI / 2;
          const radius = 100;
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
                strokeWidth={isHealing ? 2 : 0.5}
                strokeOpacity="0.5"
              />
              
              {/* Data Packet to Core */}
              {!isOffline && (
                 <circle r="1.5" fill={color}>
                    <animateMotion 
                       dur={`${1 + Math.random()}s`}
                       repeatCount="indefinite"
                       path={`M ${x} ${y} L 200 150`}
                       keyPoints="0;1"
                       keyTimes="0;1"
                    />
                 </circle>
              )}
              
              {/* Node Graphics */}
              <g transform={`translate(${x}, ${y})`}>
                 {/* Rotating Satellite Ring */}
                 {!isOffline && (
                    <circle r="12" fill="none" stroke={color} strokeWidth="0.5" strokeDasharray="4,4" className="animate-[spin_4s_linear_infinite]" opacity="0.5"/>
                 )}
                 
                 {/* Main Node Circle */}
                 <circle r="6" fill="#000" stroke={color} strokeWidth="1.5" />
                 
                 {/* Inner Status Indicator */}
                 <circle r="3" fill={color} className={isHealing ? "animate-pulse" : ""} />

                 {/* Text Label Background */}
                 <rect x="-20" y="10" width="40" height="10" fill="#000" opacity="0.7" rx="2" />
                 <text x="0" y="18" textAnchor="middle" fill={color} fontSize="6" fontFamily="monospace" fontWeight="bold">
                    {agent.name.split('-').pop()?.toUpperCase()}
                 </text>
              </g>
            </g>
          );
        })}
      </svg>
      
      {/* Decorative Corner Lines */}
      <div className="absolute top-0 left-0 w-8 h-8 border-t border-l border-green-500 opacity-50"></div>
      <div className="absolute top-0 right-0 w-8 h-8 border-t border-r border-green-500 opacity-50"></div>
      <div className="absolute bottom-0 left-0 w-8 h-8 border-b border-l border-green-500 opacity-50"></div>
      <div className="absolute bottom-0 right-0 w-8 h-8 border-b border-r border-green-500 opacity-50"></div>
    </div>
  );
};

// --- GOSSIP STREAM ---
const GossipStream = ({ logs }: { logs: GossipLog[] }) => {
    return (
        <TerminalFrame title="GOSSIP_PROTOCOL [SIDE_NET]" className="h-full">
            <div className="flex flex-col gap-2 font-mono text-[10px] md:text-xs">
                {logs.map((log) => (
                    <div key={log.id} className="flex flex-col bg-blue-900/10 border-l-2 border-blue-500 p-2 group hover:bg-blue-900/20 transition-colors">
                        <div className="flex justify-between items-center text-blue-300 mb-1">
                            <span className="flex items-center gap-1 group-hover:text-white">
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
                <div className="text-center text-blue-500 animate-pulse text-[10px] mt-2 border-t border-blue-900/30 pt-2">
                    /// ENCRYPTED P2P CHANNEL OPEN ///
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
          <div className="flex items-center justify-between h-full px-2 relative overflow-hidden">
            <div className="relative z-10">
               <div className="text-3xl font-bold text-green-400 font-mono">{onlineCount}/{MOCK_AGENTS.length}</div>
               <div className="text-[10px] text-green-700">MESH INTEGRITY: OK</div>
            </div>
            <Server className="w-8 h-8 text-green-800 relative z-10" />
            <div className="absolute right-0 top-0 w-16 h-full bg-gradient-to-l from-green-900/20 to-transparent"></div>
          </div>
        </TerminalFrame>
        
        <TerminalFrame title="SIDE_NET_MESH" className="h-24">
          <div className="flex items-center justify-between h-full px-2 relative overflow-hidden">
             <div className="flex flex-col relative z-10">
              <span className="text-3xl font-bold font-mono text-blue-400">ACTIVE</span>
              <span className="text-[10px] text-blue-700">GOSSIP PROTOCOL</span>
             </div>
            <Share2 className="w-8 h-8 text-blue-800 animate-pulse relative z-10" />
            <div className="absolute right-0 top-0 w-16 h-full bg-gradient-to-l from-blue-900/20 to-transparent"></div>
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
          <TerminalFrame title="CLUSTER_TOPOLOGY + MESH OVERLAY" className="flex-grow min-h-[350px]">
             <TopologyMap agents={MOCK_AGENTS} />
             
             {/* Map Legend */}
             <div className="absolute bottom-2 right-2 flex flex-col items-end gap-1 pointer-events-none z-20 bg-black/50 p-1 backdrop-blur-sm border border-green-900/30">
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
                 <div className="flex items-center justify-between mb-2 text-xs text-green-700">
                    <span className="flex items-center gap-2"><Cpu size={12}/> CLUSTER AVERAGE</span>
                    <BarChart3 size={12} />
                 </div>
                 <div className="flex-1 min-h-0">
                   <StatsChart data={chartData} dataKey="load" color="#00ff41" />
                 </div>
               </div>
             </TerminalFrame>
             <TerminalFrame title="NET_IO_TRAFFIC">
               <div className="flex flex-col h-full">
                  <div className="flex items-center justify-between mb-2 text-xs text-blue-700">
                    <span className="flex items-center gap-2"><Network size={12}/> INGRESS/EGRESS</span>
                    <Activity size={12} />
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
