
import React from 'react';
import { Activity, Cpu, Zap, Play, RotateCw, ShieldAlert, Wifi, Terminal } from 'lucide-react';

export const RightSidebar: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
  return (
    <aside 
      className={`
        fixed inset-y-0 right-0 z-30 w-80 bg-[#050505]/95 backdrop-blur-md border-l border-white/5 
        transform transition-transform duration-300 ease-in-out
        ${isOpen ? 'translate-x-0' : 'translate-x-full'}
        flex flex-col shadow-[-10px_0_30px_rgba(0,0,0,0.5)]
      `}
    >
      {/* Header */}
      <div className="h-16 flex items-center justify-between px-5 border-b border-white/5 bg-[#080808]">
         <div className="flex items-center gap-2 text-green-500">
            <Activity size={18} />
            <span className="font-bold tracking-wider text-sm">CMD_CENTER</span>
         </div>
         <div className="flex items-center gap-2">
            <div className="flex items-center gap-1 text-[10px] text-gray-500 font-mono">
               <div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"></div>
               ONLINE
            </div>
         </div>
      </div>

      <div className="flex-1 overflow-y-auto p-5 space-y-8 custom-scrollbar">
         
         {/* System Resources */}
         <div className="space-y-3">
            <h3 className="text-[10px] font-mono text-gray-500 uppercase tracking-widest mb-2 flex items-center gap-2">
                <Cpu size={12}/> Resource_Monitor
            </h3>
            
            <div className="bg-[#0a0a0a] border border-white/5 p-4 rounded-sm space-y-4 shadow-inner">
               <div className="space-y-1">
                  <div className="flex justify-between text-xs text-gray-400">
                     <span className="font-mono">V-CORE LOAD</span>
                     <span className="text-white font-mono font-bold">42%</span>
                  </div>
                  <div className="h-1.5 w-full bg-gray-800 rounded-full overflow-hidden border border-white/5">
                     <div className="h-full bg-blue-500 w-[42%] shadow-[0_0_10px_rgba(59,130,246,0.5)]"></div>
                  </div>
               </div>

               <div className="space-y-1">
                  <div className="flex justify-between text-xs text-gray-400">
                     <span className="font-mono">MEMORY ALLOC</span>
                     <span className="text-white font-mono font-bold">12.4GB</span>
                  </div>
                  <div className="h-1.5 w-full bg-gray-800 rounded-full overflow-hidden border border-white/5">
                     <div className="h-full bg-purple-500 w-[65%] shadow-[0_0_10px_rgba(168,85,247,0.5)]"></div>
                  </div>
               </div>
               
               <div className="space-y-1">
                  <div className="flex justify-between text-xs text-gray-400">
                     <span className="font-mono">SWAP USAGE</span>
                     <span className="text-white font-mono font-bold">2%</span>
                  </div>
                  <div className="h-1.5 w-full bg-gray-800 rounded-full overflow-hidden border border-white/5">
                     <div className="h-full bg-yellow-500 w-[2%]"></div>
                  </div>
               </div>
            </div>
         </div>

         {/* Active Tasks */}
         <div className="space-y-3">
            <h3 className="text-[10px] font-mono text-gray-500 uppercase tracking-widest mb-2 flex items-center gap-2">
                <RotateCw size={12}/> Background_Tasks
            </h3>
            
            <div className="space-y-2">
               <TaskItem title="Compiling Assets" progress={75} status="PROCESSING" />
               <TaskItem title="Gossip Sync" progress={30} status="SYNCING" color="blue" />
               <TaskItem title="Backup Routine" progress={100} status="COMPLETED" color="green" />
            </div>
         </div>

         {/* Quick Actions */}
         <div className="space-y-3">
             <h3 className="text-[10px] font-mono text-gray-500 uppercase tracking-widest mb-2 flex items-center gap-2">
                <Zap size={12}/> Quick_Execution
             </h3>
             <div className="grid grid-cols-2 gap-2">
                <ActionButton icon={Play} label="START_ALL" />
                <ActionButton icon={RotateCw} label="FLUSH_DNS" />
                <ActionButton icon={ShieldAlert} label="LOCKDOWN" danger />
                <ActionButton icon={Wifi} label="RECONNECT" />
             </div>
         </div>

         {/* Mini Log */}
         <div className="flex-1 min-h-[150px] bg-black border border-green-900/30 p-3 font-mono text-[10px] text-gray-500 overflow-hidden flex flex-col rounded-sm">
            <div className="text-green-500 border-b border-green-900/30 pb-2 mb-2 flex items-center gap-2">
                <Terminal size={10} /> SYSTEM_FEED
            </div>
            <div className="flex-1 overflow-y-auto space-y-1.5 custom-scrollbar">
               <div className="text-gray-500"><span className="text-gray-700">10:42:01</span> > Initializing neural link...</div>
               <div className="text-gray-400"><span className="text-gray-700">10:42:02</span> > Handshake verified (TLS 1.3)</div>
               <div className="text-blue-400"><span className="text-gray-700">10:42:05</span> > Uplink established to [Core]</div>
               <div className="text-gray-500"><span className="text-gray-700">10:42:10</span> > Daemon [k8s-watcher] started</div>
               <div className="text-yellow-600"><span className="text-gray-700">10:42:15</span> > Warning: High latency on shard-02</div>
               <div className="animate-pulse text-green-500 mt-2">_</div>
            </div>
         </div>

      </div>
      
      {/* Footer Status */}
      <div className="p-2 border-t border-white/5 bg-black text-[9px] text-gray-600 font-mono text-center">
        SECURE CHANNEL // ENCRYPTED
      </div>
    </aside>
  );
};

const TaskItem = ({ title, progress, status, color = "purple" }: any) => (
  <div className="bg-[#0a0a0a] border border-white/5 p-3 rounded-sm group hover:border-white/10 transition-colors cursor-pointer">
     <div className="flex justify-between items-center mb-2">
        <span className="text-xs font-bold text-gray-300 group-hover:text-white transition-colors">{title}</span>
        <span className={`text-[9px] font-mono bg-${color}-500/10 text-${color}-400 px-1.5 py-0.5 rounded border border-${color}-500/20`}>{status}</span>
     </div>
     <div className="h-1 w-full bg-gray-800 overflow-hidden rounded-full">
        <div className={`h-full bg-${color}-500 transition-all duration-1000`} style={{ width: `${progress}%` }}></div>
     </div>
  </div>
);

const ActionButton = ({ icon: Icon, label, danger }: any) => (
  <button className={`
    flex flex-col items-center justify-center gap-1.5 p-3 border rounded-sm transition-all active:scale-95 group
    ${danger 
      ? 'border-red-900/30 bg-red-900/5 text-red-500 hover:bg-red-900/20 hover:border-red-500 hover:shadow-[0_0_10px_rgba(239,68,68,0.2)]' 
      : 'border-white/5 bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white hover:border-white/20'}
  `}>
     <Icon size={16} className={danger ? "text-red-500" : "text-gray-400 group-hover:text-green-400 transition-colors"}/>
     <span className="text-[9px] font-bold tracking-widest">{label}</span>
  </button>
);
