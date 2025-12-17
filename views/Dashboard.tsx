

import React, { useEffect, useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import {
   AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, ReferenceLine
} from 'recharts';
import { MOCK_LOGS } from '../constants';
import {
   Server, Zap, ShieldCheck,
   Cpu, Globe, Clock, ArrowUpRight,
   Database, Lock, Wifi, HardDrive
} from 'lucide-react';
import { dashboardApi } from '../api';

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
   <div className="relative overflow-hidden rounded-md bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 p-5 group hover:border-green-500/30 transition-all hover:shadow-[0_0_20px_rgba(0,255,65,0.05)]">
      <div className="flex justify-between items-start mb-4 relative z-10">
         <div className={`p-2 rounded-lg bg-${color}-500/10 text-${color}-500 group-hover:bg-${color}-500/20 transition-colors`}>
            <Icon size={20} />
         </div>
         {trend && (
            <div className="flex items-center gap-1 text-[10px] font-mono text-green-400 bg-green-500/10 px-2 py-1 rounded-full border border-green-500/20">
               <ArrowUpRight size={10} /> {trend}
            </div>
         )}
      </div>
      <div className="relative z-10">
         <div className="text-3xl font-bold text-white mb-1 tracking-tight font-sans">{value}</div>
         <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">{title}</div>
      </div>
      {/* Background Glow */}
      <div className={`absolute -right-6 -bottom-6 w-24 h-24 bg-${color}-500/10 rounded-full blur-[40px] group-hover:bg-${color}-500/20 transition-colors`}></div>
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
      <div className="h-full flex flex-col gap-2 overflow-y-auto pr-2">
         {services.map((svc, i) => (
            <div key={i} className="flex items-center justify-between p-3 bg-white/[0.02] border border-white/5 hover:bg-white/[0.04] hover:border-green-500/20 transition-all group rounded-md">
               <div className="flex items-center gap-3">
                  <div className={`p-2 rounded bg-black/50 border border-white/10 ${svc.status === 'DEGRADED' ? 'text-yellow-500' : 'text-green-500'}`}>
                     <svc.icon size={14} />
                  </div>
                  <div>
                     <div className="text-xs font-bold text-gray-200 group-hover:text-white transition-colors">{svc.name}</div>
                     <div className="text-[10px] text-gray-500 font-mono flex items-center gap-2">
                        <span className={svc.status === 'OPERATIONAL' ? 'text-green-500' : 'text-yellow-500'}>{svc.status}</span>
                        <span className="text-gray-800">|</span>
                        <span>{svc.lat}</span>
                     </div>
                  </div>
               </div>

               <div className="w-24 flex flex-col gap-1 items-end">
                  <div className="w-full h-1 bg-gray-800 rounded-full overflow-hidden">
                     <div
                        className={`h-full ${svc.load > 90 ? 'bg-red-500' : svc.load > 70 ? 'bg-yellow-500' : 'bg-green-500'}`}
                        style={{ width: `${svc.load}%` }}
                     ></div>
                  </div>
               </div>
            </div>
         ))}
      </div>
   );
};

export const Dashboard: React.FC = () => {
   const [stats, setStats] = useState<any>(null);
   const [loading, setLoading] = useState(true);

   useEffect(() => {
      const fetchStats = async () => {
         try {
            const data = await dashboardApi.getStats();
            setStats(data);
         } catch (error) {
            console.error('Failed to fetch stats:', error);
         } finally {
            setLoading(false);
         }
      };

      fetchStats();
      const interval = setInterval(fetchStats, 5000); // Refresh every 5 seconds
      return () => clearInterval(interval);
   }, []);

   return (
      <div className="p-6 space-y-6 max-w-[1800px] mx-auto pb-20">

         {/* Header Info */}
         <div className="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-white/5 pb-6">
            <div>
               <h1 className="text-3xl font-bold text-white tracking-tight mb-2">Command Center</h1>
               <div className="flex items-center gap-6 text-sm text-gray-500 font-mono">
                  <span className="flex items-center gap-2">
                     <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                     GRID: {loading ? 'CONNECTING...' : 'ONLINE'}
                  </span>
                  <span className="flex items-center gap-2">
                     <span className="w-2 h-2 bg-blue-500 rounded-full"></span>
                     SYNC: ACTIVE
                  </span>
                  <span>LATENCY: 12ms</span>
               </div>
            </div>

            <div className="flex gap-2">
               <button className="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 text-xs font-mono text-gray-300 rounded hover:text-white transition-colors">
                  GENERATE REPORT
               </button>
               <button className="px-4 py-2 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-xs font-mono text-green-400 rounded hover:text-green-300 transition-colors">
                  SYSTEM DIAGNOSTICS
               </button>
            </div>
         </div>

         {/* KPI Grid */}
         <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4">
            <KPICard 
               title="Active Nodes" 
               value={stats?.active_nodes || '...'} 
               icon={Server} 
               trend="+2 New" 
               color="blue" 
            />
            <KPICard 
               title="Throughput" 
               value={stats?.throughput || '...'} 
               icon={Zap} 
               trend="+12%" 
               color="yellow" 
            />
            <KPICard 
               title="Avg Load" 
               value={stats?.avg_load || '...'} 
               icon={Cpu} 
               color="purple" 
            />
            <KPICard 
               title="Security Score" 
               value={stats?.security_score || '...'} 
               icon={ShieldCheck} 
               color="green" 
            />
         </div>

         {/* Middle Section: Services, Resources, Events */}
         <div className="grid grid-cols-1 xl:grid-cols-3 gap-6 h-auto xl:h-[500px]">

            {/* Col 1: Service Mesh */}
            <TerminalFrame title="Service Mesh Topology" className="h-[300px] xl:h-full">
               <ServiceHealthMatrix />
            </TerminalFrame>

            {/* Col 2: Resource Monitor */}
            <div className="flex flex-col gap-4 h-full">
               <TerminalFrame title="Network Traffic" className="flex-1" noPadding>
                  <div className="h-full flex flex-col items-center justify-center p-4">
                     <div className="flex items-end gap-1 w-full h-20 justify-between">
                        {[40, 60, 45, 70, 30, 55, 65, 40, 80, 50, 60, 75, 45, 85, 55].map((h, i) => (
                           <div key={i} className="flex-1 bg-green-500/20 hover:bg-green-500 transition-all rounded-[1px]" style={{ height: `${h}%` }}></div>
                        ))}
                     </div>
                     <div className="flex justify-between w-full mt-2 text-xs font-mono">
                        <div className="flex flex-col">
                           <span className="text-gray-500">INBOUND</span>
                           <span className="text-white text-lg">4.2 GB/s</span>
                        </div>
                        <div className="flex flex-col items-end">
                           <span className="text-gray-500">OUTBOUND</span>
                           <span className="text-white text-lg">1.8 GB/s</span>
                        </div>
                     </div>
                  </div>
               </TerminalFrame>

               <TerminalFrame title="Storage Clusters" className="flex-1" noPadding>
                  <div className="p-6 space-y-4">
                     <div className="space-y-2">
                        <div className="flex justify-between text-xs font-mono text-gray-400"><span>NVMe_POOL_A</span> <span className="text-blue-400">78%</span></div>
                        <div className="h-2 w-full bg-gray-900 rounded-full overflow-hidden border border-white/5">
                           <div className="h-full bg-blue-500 shadow-[0_0_10px_rgba(59,130,246,0.5)] w-[78%]"></div>
                        </div>
                     </div>
                     <div className="space-y-2">
                        <div className="flex justify-between text-xs font-mono text-gray-400"><span>ARCHIVE_COLD</span> <span className="text-purple-400">22%</span></div>
                        <div className="h-2 w-full bg-gray-900 rounded-full overflow-hidden border border-white/5">
                           <div className="h-full bg-purple-500 shadow-[0_0_10px_rgba(168,85,247,0.5)] w-[22%]"></div>
                        </div>
                     </div>
                  </div>
               </TerminalFrame>
            </div>

            {/* Col 3: System Events */}
            <TerminalFrame title="Live Event Log" className="h-full" noPadding>
               <div className="overflow-y-auto h-full p-0 scrollbar-thin scrollbar-thumb-white/10 scrollbar-track-transparent">
                  {MOCK_LOGS.map((log) => (
                     <div key={log.id} className="flex gap-3 p-3 border-b border-white/5 hover:bg-white/5 transition-colors text-xs group">
                        <div className="mt-0.5 text-gray-600 group-hover:text-green-500 transition-colors">
                           <Clock size={12} />
                        </div>
                        <div className="flex-1 min-w-0">
                           <div className="flex justify-between mb-0.5">
                              <span className="font-bold text-gray-300 group-hover:text-white">{log.source}</span>
                              <span className="text-[10px] text-gray-600 font-mono">{log.timestamp}</span>
                           </div>
                           <p className="text-gray-500 truncate group-hover:text-gray-400 transition-colors">{log.message}</p>
                        </div>
                     </div>
                  ))}
               </div>
            </TerminalFrame>

         </div>

         {/* Bottom Section: Main Chart */}
         <div className="h-[350px]">
            <TerminalFrame title="Cluster Load Distribution (24h)" className="h-full">
               <div className="h-full w-full p-2" style={{ contain: 'layout paint' }}>
                  <ResponsiveContainer width="100%" height="100%" debounce={200}>
                     <AreaChart data={STATIC_CHART_DATA} margin={{ top: 20, right: 30, left: 20, bottom: 30 }}>
                        <defs>
                           <linearGradient id="colorLoad" x1="0" y1="0" x2="0" y2="1">
                              <stop offset="5%" stopColor="#22c55e" stopOpacity={0.3} />
                              <stop offset="95%" stopColor="#22c55e" stopOpacity={0} />
                           </linearGradient>
                           <linearGradient id="colorPred" x1="0" y1="0" x2="0" y2="1">
                              <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.1} />
                              <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                           </linearGradient>
                        </defs>
                        <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" vertical={false} />
                        <XAxis
                           dataKey="time"
                           stroke="#666"
                           fontSize={11}
                           tickLine={false}
                           axisLine={false}
                           tick={{ fill: '#666' }}
                           dy={10}
                        />
                        <YAxis
                           stroke="#666"
                           fontSize={11}
                           tickLine={false}
                           axisLine={false}
                           tick={{ fill: '#666' }}
                           dx={-10}
                        />
                        <Tooltip
                           contentStyle={{
                              backgroundColor: 'rgba(9, 9, 11, 0.9)',
                              border: '1px solid rgba(255,255,255,0.1)',
                              borderRadius: '8px',
                              boxShadow: '0 4px 20px rgba(0,0,0,0.5)',
                              backdropFilter: 'blur(8px)'
                           }}
                           itemStyle={{ fontSize: '12px', fontWeight: 600 }}
                           labelStyle={{ color: '#999', marginBottom: '8px', fontFamily: 'monospace' }}
                           cursor={{ stroke: 'rgba(255,255,255,0.1)', strokeWidth: 1 }}
                        />
                        <Area
                           type="monotone"
                           dataKey="predicted"
                           stroke="#3b82f6"
                           strokeDasharray="4 4"
                           strokeWidth={2}
                           fillOpacity={1}
                           fill="url(#colorPred)"
                           name="AI Prediction"
                        />
                        <Area
                           type="monotone"
                           dataKey="load"
                           stroke="#22c55e"
                           strokeWidth={2}
                           fillOpacity={1}
                           fill="url(#colorLoad)"
                           name="System Load"
                        />
                        <ReferenceLine
                           x="18:00"
                           stroke="#ef4444"
                           strokeDasharray="3 3"
                           label={{
                              position: 'top',
                              value: 'ANOMALY DETECTED',
                              fill: '#ef4444',
                              fontSize: 10,
                              fontWeight: 'bold'
                           }}
                        />
                     </AreaChart>
                  </ResponsiveContainer>
               </div>
            </TerminalFrame>
         </div>
      </div>
   );
};
