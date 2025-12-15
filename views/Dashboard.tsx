
import React from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import {
   AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, ReferenceLine
} from 'recharts';
import { MOCK_LOGS } from '../constants';
import {
   Server, Zap, ShieldCheck,
   Cpu, Network, Globe, Clock, ArrowUpRight,
   Database, Lock, Wifi, HardDrive, AlertCircle
} from 'lucide-react';

const STATIC_CHART_DATA = [
   { time: '00:00', load: 24, predicted: 20 },
   { time: '02:00', load: 18, predicted: 18 },
   { time: '04:00', load: 15, predicted: 15 },
   { time: '06:00', load: 32, predicted: 30 },
   { time: '08:00', load: 65, predicted: 60 },
   { time: '10:00', load: 88, predicted: 85 },
   { time: '11:00', load: 95, predicted: 92 },
   { time: '12:00', load: 92, predicted: 90 },
   { time: '13:00', load: 85, predicted: 88 },
   { time: '14:00', load: 78, predicted: 80 },
   { time: '16:00', load: 70, predicted: 75 },
   { time: '18:00', load: 96, predicted: 85 },
   { time: '20:00', load: 70, predicted: 75 },
   { time: '22:00', load: 45, predicted: 40 },
   { time: '23:59', load: 30, predicted: 30 },
];

const KPICard = ({ title, value, icon: Icon, trend, color = "green" }: any) => (
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

const ServiceHealthMatrix = () => {
   const services = [
      { name: 'AUTH_GATEWAY', status: 'OPERATIONAL', lat: '12ms', load: 45, icon: Lock },
      { name: 'CORE_DB_SHARD_01', status: 'OPERATIONAL', lat: '4ms', load: 78, icon: Database },
      { name: 'EDGE_ROUTER_EU', status: 'DEGRADED', lat: '145ms', load: 92, icon: Globe },
      { name: 'STORAGE_BLOB_S3', status: 'OPERATIONAL', lat: '24ms', load: 12, icon: HardDrive },
      { name: 'MSG_QUEUE_KAFKA', status: 'OPERATIONAL', lat: '8ms', load: 33, icon: Wifi },
   ];

   return (
      <div className="h-full flex flex-col gap-2 p-2 overflow-y-auto">
         {services.map((svc, i) => (
            <div key={i} className="flex items-center justify-between p-3 bg-white/5 border border-white/5 hover:border-green-500/30 transition-all group rounded-sm">
               <div className="flex items-center gap-3">
                  <div className={`p-2 rounded bg-black border border-white/10 ${svc.status === 'DEGRADED' ? 'text-yellow-500' : 'text-green-500'}`}>
                     <svc.icon size={14} />
                  </div>
                  <div>
                     <div className="text-xs font-bold text-gray-200 group-hover:text-white">{svc.name}</div>
                     <div className="text-[10px] text-gray-500 font-mono flex items-center gap-2">
                        <span className={svc.status === 'OPERATIONAL' ? 'text-green-600' : 'text-yellow-600'}>{svc.status}</span>
                        <span className="text-gray-700">|</span>
                        <span>{svc.lat}</span>
                     </div>
                  </div>
               </div>

               <div className="w-24 flex flex-col gap-1 items-end">
                  <span className="text-[10px] font-mono text-gray-400">{svc.load}% LOAD</span>
                  <div className="w-full h-1 bg-gray-800 rounded-full overflow-hidden">
                     <div
                        className={`h-full ${svc.load > 90 ? 'bg-red-500' : svc.load > 70 ? 'bg-yellow-500' : 'bg-green-500'}`}
                        style={{ width: `${svc.load}%` }}
                     ></div>
                  </div>
               </div>
            </div>
         ))}
         <div className="mt-auto pt-2 border-t border-white/5 text-[10px] text-gray-500 text-center font-mono">
            <span className="animate-pulse text-green-500">●</span> REAL-TIME MESH SYNC ACTIVE
         </div>
      </div>
   );
};

export const Dashboard: React.FC = () => {
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
                     <ResponsiveContainer width="100%" height="100%" debounce={300}>
                        <AreaChart data={STATIC_CHART_DATA}>
                           <defs>
                              <linearGradient id="colorLoad" x1="0" y1="0" x2="0" y2="1">
                                 <stop offset="5%" stopColor="#00ff41" stopOpacity={0.3} />
                                 <stop offset="95%" stopColor="#00ff41" stopOpacity={0} />
                              </linearGradient>
                              <linearGradient id="colorPred" x1="0" y1="0" x2="0" y2="1">
                                 <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.1} />
                                 <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                              </linearGradient>
                           </defs>
                           <CartesianGrid strokeDasharray="3 3" stroke="#333" vertical={false} />
                           <XAxis dataKey="time" stroke="#666" fontSize={10} tickLine={false} axisLine={false} />
                           <YAxis stroke="#666" fontSize={10} tickLine={false} axisLine={false} />
                           <Tooltip
                              contentStyle={{ backgroundColor: '#09090b', border: '1px solid #333', borderRadius: '4px' }}
                              itemStyle={{ fontSize: '12px' }}
                              labelStyle={{ color: '#999', marginBottom: '5px' }}
                           />
                           <Area
                              type="monotone"
                              dataKey="predicted"
                              stroke="#3b82f6"
                              strokeDasharray="5 5"
                              strokeWidth={2}
                              fillOpacity={1}
                              fill="url(#colorPred)"
                              name="AI Prediction"
                           />
                           <Area
                              type="monotone"
                              dataKey="load"
                              stroke="#00ff41"
                              strokeWidth={2}
                              fillOpacity={1}
                              fill="url(#colorLoad)"
                              name="Actual Load"
                           />
                           <ReferenceLine x="18:00" stroke="red" strokeDasharray="3 3" label={{ position: 'top', value: 'SPIKE DETECTED', fill: 'red', fontSize: 10 }} />
                        </AreaChart>
                     </ResponsiveContainer>
                  </div>
               </TerminalFrame>

               <div className="h-1/3 grid grid-cols-2 gap-4">
                  <TerminalFrame title="Network I/O" noPadding>
                     <div className="h-full flex flex-col items-center justify-center text-gray-600 gap-2 p-4">
                        <div className="flex items-end gap-1 w-full h-16 justify-between px-4">
                           {[40, 60, 45, 70, 30, 55, 65, 40, 80, 50].map((h, i) => (
                              <div key={i} className="w-1.5 bg-green-500/20 hover:bg-green-500 transition-colors rounded-sm" style={{ height: `${h}%` }}></div>
                           ))}
                        </div>
                        <div className="flex justify-between w-full px-4 text-xs">
                           <span className="text-gray-500">RX: <span className="text-white">4.2 GB/s</span></span>
                           <span className="text-gray-500">TX: <span className="text-white">1.8 GB/s</span></span>
                        </div>
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

            {/* Right Column */}
            <div className="lg:col-span-1 flex flex-col gap-6">
               {/* Replaced Topology with Service Matrix */}
               <TerminalFrame title="Service Mesh Status" className="h-[250px]" noPadding>
                  <ServiceHealthMatrix />
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
