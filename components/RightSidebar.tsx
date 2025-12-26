
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
        fixed inset-y-0 right-0 z-30 w-80 bg-[#2b2d30] border-l border-[#43454a]
        transform transition-all duration-300 ease-in-out shadow-[-4px_0_10px_rgba(0,0,0,0.3)]
        ${isOpen ? 'translate-x-0' : 'translate-x-full'}
        flex flex-col
      `}
      >
         {/* Tool Window Header */}
         <div className="h-9 flex items-center justify-between px-3 border-b border-[#43454a] bg-[#2b2d30] select-none">
            <div className="flex items-center gap-2 text-[#dfe1e5]">
               <div className="p-1 rounded bg-[#3574f015] border border-[#3574f030]">
                  <Activity size={14} className="text-[#3574f0]" />
               </div>
               <span className="text-[11px] font-bold uppercase tracking-wider">Command Center</span>
            </div>
            <div className="flex items-center gap-1.5">
               <div className="flex items-center gap-1.5 text-[10px] text-[#59a869] font-semibold bg-[#59a86915] px-2 py-0.5 rounded-[var(--radius)]">
                  <div className="w-1.5 h-1.5 bg-[#59a869] rounded-full"></div>
                  ONLINE
               </div>
               <button onClick={onClose} className="text-[#6e7073] hover:text-[#dfe1e5] transition-colors p-1 hover:bg-[#393b40] rounded-[var(--radius)]">
                  <ChevronRight size={16} />
               </button>
            </div>
         </div>

         <div className="flex-1 overflow-y-auto p-4 space-y-6 custom-scrollbar bg-[#1e1f22]">

            {/* System Resources */}
            <div className="space-y-3">
               <SectionHeader icon={Cpu} title="RESOURCES" />
               <div className="bg-[#2b2d30] border border-[#43454a] p-3 rounded-[var(--radius)] space-y-4 shadow-sm">
                  <ResourceBar label="CPU Load" value="42%" color="#3574f0" width="42%" />
                  <ResourceBar label="Memory" value="12.4GB" color="#3574f0" width="65%" />
                  <ResourceBar label="Disk I/O" value="2%" color="#59a869" width="2%" />
               </div>
            </div>

            {/* Network Mesh */}
            <div className="space-y-3">
               <SectionHeader icon={Globe} title="NETWORK" />
               <div className="grid grid-cols-2 gap-2">
                  <NetworkStat label="PING" value="24ms" status="good" />
                  <NetworkStat label="UP" value="1.2G" status="good" />
               </div>
            </div>

            {/* Quick Execution */}
            <div className="space-y-3">
               <SectionHeader icon={Zap} title="ACTIONS" />
               <div className="grid grid-cols-2 gap-2">
                  <ActionButton icon={Play} label="RESTART" />
                  <ActionButton icon={ShieldAlert} label="LOCKDOWN" danger />
               </div>
            </div>

            {/* Mini Log */}
            <div className="bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] overflow-hidden flex flex-col shadow-inner">
               <div className="bg-[#2b2d30] px-3 py-1.5 border-b border-[#43454a] flex items-center justify-between">
                  <div className="flex items-center gap-2 text-[#6e7073]">
                     <Terminal size={12} />
                     <span className="text-[10px] font-bold uppercase tracking-tight">System Feed</span>
                  </div>
               </div>
               <div className="p-3 font-mono text-[11px] h-48 overflow-y-auto custom-scrollbar space-y-1 bg-black/20">
                  {logs.map((log, i) => (
                     <div key={i} className="text-[#6e7073] hover:text-[#dfe1e5] transition-colors">
                        <span className="opacity-50 mr-2">$</span>
                        <span className={log.includes('Warning') ? 'text-[#e06c75]' : ''}>{log}</span>
                     </div>
                  ))}
                  <div className="animate-pulse text-[#3574f0]">_</div>
               </div>
            </div>

         </div>

         {/* Footer Status */}
         <div className="h-6 border-t border-[#43454a] bg-[#2b2d30] text-[10px] text-[#6e7073] font-medium flex justify-between items-center px-4 select-none">
            <span className="flex items-center gap-1.5"><Lock size={10} /> ENCRYPTED</span>
            <span className="opacity-50">US-EAST-1</span>
         </div>
      </aside>
   );
};

// Subcomponents
const SectionHeader = ({ icon: Icon, title }: any) => (
   <div className="flex items-center gap-2 px-1">
      <Icon size={12} className="text-[#6e7073]" />
      <h3 className="text-[10px] font-bold text-[#6e7073] uppercase tracking-widest">{title}</h3>
   </div>
);

const ResourceBar = ({ label, value, color, width }: any) => (
   <div className="space-y-1.5">
      <div className="flex justify-between text-[10px] text-[#dfe1e5] font-medium px-0.5">
         <span>{label}</span>
         <span style={{ color }}>{value}</span>
      </div>
      <div className="h-1 w-full bg-[#1e1f22] rounded-full overflow-hidden border border-[#43454a]">
         <div className="h-full transition-all duration-1000 rounded-full" style={{ width, backgroundColor: color }}></div>
      </div>
   </div>
);

const NetworkStat = ({ label, value, status }: any) => (
   <div className="bg-[#2b2d30] border border-[#43454a] p-2 rounded-[var(--radius)] flex flex-col items-center justify-center gap-0.5 shadow-sm">
      <span className="text-[9px] text-[#6e7073] font-bold">{label}</span>
      <span className={`text-[13px] font-bold ${status === 'good' ? 'text-[#59a869]' : 'text-[#e06c75]'}`}>{value}</span>
   </div>
);

const ActionButton = ({ icon: Icon, label, danger }: any) => (
   <button className={`
    flex items-center gap-2 p-2 border rounded-[var(--radius)] transition-all active:scale-95 group relative shadow-sm
    ${danger
         ? 'border-[#e06c7540] bg-[#e06c7510] text-[#e06c75] hover:bg-[#e06c7520] hover:border-[#e06c75]'
         : 'border-[#43454a] bg-[#2b2d30] text-[#6e7073] hover:bg-[#393b40] hover:text-[#dfe1e5] hover:border-[#6a6e75]'}
  `}>
      <Icon size={14} />
      <span className="text-[10px] font-bold tracking-tight">{label}</span>
   </button>
);
