
import React, { useState, useEffect } from 'react';
import { Activity, Cpu, Zap, Play, RotateCw, ShieldAlert, Wifi, Terminal, ChevronRight, Lock, Database, Globe } from 'lucide-react';

export const RightSidebar: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
   const [logs, setLogs] = useState<string[]>([
      "10:42:01 > Initializing neural link...",
      "10:42:02 > Handshake verified (TLS 1.3)",
      "10:42:05 > Uplink established to [Core]",
      "10:42:10 > Daemon [k8s-watcher] started",
      "10:42:15 > Warning: High latency on shard-02"
   ]);

   // Simulate incoming logs
   useEffect(() => {
      if (!isOpen) return;
      const interval = setInterval(() => {
         const msgs = [
            "Keeping alive...",
            "Packet trace complete",
            "Garbage collection triggered",
            "Syncing headers with peer",
            "Optimizing V8 heap",
            "Checking integrity..."
         ];
         const randomMsg = msgs[Math.floor(Math.random() * msgs.length)];
         const time = new Date().toLocaleTimeString('en-US', { hour12: false });
         setLogs(prev => [...prev.slice(-10), `${time} > ${randomMsg}`]);
      }, 3000);
      return () => clearInterval(interval);
   }, [isOpen]);

   return (
      <aside
         className={`
        fixed inset-y-0 right-0 z-30 w-96 bg-[#030303]/95 backdrop-blur-xl border-l border-white/5 
        transform transition-transform duration-500 cubic-bezier(0.23, 1, 0.32, 1)
        ${isOpen ? 'translate-x-0' : 'translate-x-full'}
        flex flex-col shadow-[-20px_0_50px_rgba(0,0,0,0.7)]
      `}
      >
         {/* Glow Effect */}
         <div className="absolute top-0 bottom-0 left-0 w-[1px] bg-gradient-to-b from-transparent via-green-500/50 to-transparent opacity-50"></div>

         {/* Header */}
         <div className="h-16 flex items-center justify-between px-6 border-b border-white/5 bg-[#050505]">
            <div className="flex items-center gap-3 text-green-500">
               <div className="p-1.5 rounded bg-green-900/20 border border-green-500/20">
                  <Activity size={16} className="animate-pulse" />
               </div>
               <div className="flex flex-col">
                  <span className="font-bold tracking-widest text-xs font-mono">COMMAND_CENTER</span>
                  <span className="text-[9px] text-gray-500 font-mono tracking-widest">SUB_SYSTEM_V.2</span>
               </div>
            </div>
            <div className="flex items-center gap-3">
               <div className="flex items-center gap-1.5 text-[10px] text-green-400/80 font-mono bg-green-900/10 px-2 py-1 rounded border border-green-900/20">
                  <div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-ping"></div>
                  ONLINE
               </div>
               <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors p-1 hover:bg-white/10 rounded">
                  <ChevronRight size={18} />
               </button>
            </div>
         </div>

         <div className="flex-1 overflow-y-auto p-6 space-y-8 custom-scrollbar">

            {/* System Resources */}
            <div className="space-y-4">
               <SectionHeader icon={Cpu} title="CORE_RESOURCES" />

               <div className="bg-[#080808] border border-white/5 p-4 rounded-lg space-y-5 relative overflow-hidden group">
                  {/* Background Grid */}
                  <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:20px_20px]"></div>

                  <ResourceBar label="V-CORE LOAD" value="42%" color="blue" width="42%" />
                  <ResourceBar label="MEMORY ALLOC" value="12.4GB" color="purple" width="65%" />
                  <ResourceBar label="SWAP USAGE" value="2%" color="yellow" width="2%" />
               </div>
            </div>

            {/* Network Status */}
            <div className="space-y-4">
               <SectionHeader icon={Globe} title="NETWORK_MESH" />
               <div className="grid grid-cols-2 gap-3">
                  <NetworkStat label="PING" value="24ms" status="good" />
                  <NetworkStat label="UP" value="1.2Gbps" status="good" />
                  <NetworkStat label="DOWN" value="850Mbps" status="warn" />
                  <NetworkStat label="PKT LOSS" value="0.01%" status="good" />
               </div>
            </div>

            {/* Active Tasks */}
            <div className="space-y-4">
               <SectionHeader icon={RotateCw} title="BACKGROUND_PROCESSES" />
               <div className="space-y-2">
                  <TaskItem title="Asset Compilation" progress={75} status="PROCESSING" />
                  <TaskItem title="Gossip Protocol Sync" progress={30} status="SYNCING" color="blue" />
                  <TaskItem title="Database Backup" progress={100} status="DONE" color="green" />
               </div>
            </div>

            {/* Quick Actions */}
            <div className="space-y-4">
               <SectionHeader icon={Zap} title="QUICK_EXECUTION" />
               <div className="grid grid-cols-2 gap-3">
                  <ActionButton icon={Play} label="START_ALL" />
                  <ActionButton icon={RotateCw} label="FLUSH_DNS" />
                  <ActionButton icon={ShieldAlert} label="LOCKDOWN" danger />
                  <ActionButton icon={Wifi} label="RECONNECT" />
               </div>
            </div>

            {/* Mini Log */}
            <div className="flex-1 min-h-[200px] bg-black border border-green-900/20 p-4 font-mono text-[11px] text-gray-500 overflow-hidden flex flex-col rounded-lg relative">
               <div className="absolute top-0 right-0 p-2 text-[9px] text-green-900/50 select-none">TERMINAL_OUT</div>
               <div className="text-green-500 border-b border-green-900/20 pb-2 mb-2 flex items-center gap-2">
                  <Terminal size={12} /> SYSTEM_FEED
               </div>
               <div className="flex-1 overflow-y-auto space-y-1.5 custom-scrollbar">
                  {logs.map((log, i) => (
                     <div key={i} className="hover:bg-white/5 px-1 rounded transition-colors break-all">
                        <span className="text-gray-600 mr-2">$</span>
                        <span dangerouslySetInnerHTML={{ __html: log.replace(/>/g, '&gt;') }}></span>
                     </div>
                  ))}
                  <div className="animate-pulse text-green-500 mt-2">_</div>
               </div>
            </div>

         </div>

         {/* Footer Status */}
         <div className="p-3 border-t border-white/5 bg-[#020202] text-[10px] text-gray-600 font-mono flex justify-between items-center px-6">
            <span className="flex items-center gap-1.5"><Lock size={10} /> ENCRYPTED: AES-256</span>
            <span className="opacity-50">SHARD: US-EAST-1</span>
         </div>
      </aside>
   );
};

// Subcomponents
const SectionHeader = ({ icon: Icon, title }: any) => (
   <h3 className="text-[10px] font-mono text-gray-500 uppercase tracking-[0.2em] flex items-center gap-2 pl-1 border-l-2 border-green-500/50">
      <Icon size={12} className="text-green-500/80" /> {title}
   </h3>
);

const ResourceBar = ({ label, value, color, width }: any) => (
   <div className="space-y-1.5 relative z-10">
      <div className="flex justify-between text-[10px] text-gray-400 font-medium">
         <span className="font-mono tracking-wider">{label}</span>
         <span className={`text-${color}-400 font-mono`}>{value}</span>
      </div>
      <div className="h-1.5 w-full bg-gray-800/50 rounded-full overflow-hidden border border-white/5">
         <div className={`h-full bg-${color}-500 shadow-[0_0_10px_currentColor] transition-all duration-1000`} style={{ width }}></div>
      </div>
   </div>
);

const NetworkStat = ({ label, value, status }: any) => (
   <div className="bg-[#0a0a0a] border border-white/5 p-2 rounded flex flex-col items-center justify-center gap-1">
      <span className="text-[9px] text-gray-600 font-mono">{label}</span>
      <span className={`text-sm font-mono font-bold ${status === 'good' ? 'text-green-400' : 'text-yellow-400'}`}>{value}</span>
   </div>
);

const TaskItem = ({ title, progress, status, color = "purple" }: any) => (
   <div className="bg-[#0a0a0a] border border-white/5 p-3 rounded group hover:border-white/10 transition-colors cursor-pointer relative overflow-hidden">
      <div className="absolute top-0 left-0 w-[2px] h-full bg-gradient-to-b from-transparent via-white/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
      <div className="flex justify-between items-center mb-2">
         <span className="text-xs font-bold text-gray-400 group-hover:text-green-200 transition-colors">{title}</span>
         <span className={`text-[9px] font-mono bg-${color}-500/10 text-${color}-400 px-1.5 py-0.5 rounded border border-${color}-500/20`}>{status}</span>
      </div>
      <div className="h-1 w-full bg-gray-800 overflow-hidden rounded-full">
         <div className={`h-full bg-${color}-500 transition-all duration-1000`} style={{ width: `${progress}%` }}></div>
      </div>
   </div>
);

const ActionButton = ({ icon: Icon, label, danger }: any) => (
   <button className={`
    flex flex-col items-center justify-center gap-2 p-4 border rounded transition-all active:scale-95 group relative overflow-hidden
    ${danger
         ? 'border-red-900/30 bg-red-900/5 text-red-500 hover:bg-red-900/20 hover:border-red-500 hover:shadow-[0_0_20px_rgba(239,68,68,0.2)]'
         : 'border-white/5 bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white hover:border-green-500/30'}
  `}>
      <Icon size={18} className={danger ? "text-red-500" : "text-gray-400 group-hover:text-green-400 transition-colors"} />
      <span className="text-[9px] font-bold tracking-widest z-10">{label}</span>
   </button>
);
