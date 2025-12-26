

import React, { useEffect, useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import {
   AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, ReferenceLine
} from 'recharts';
import { MOCK_LOGS } from '../constants';
import {
   Server, Zap, ShieldCheck,
   Cpu, Globe, Clock, ArrowUpRight,
   Database, Lock, Wifi, HardDrive,
   Layers, Activity, Bell
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

const KPICard = ({ title, value, icon: Icon, trend, color = "blue" }: any) => {
   const colorMap: Record<string, string> = {
      blue: 'text-[#3574f0] bg-[#3574f015]',
      green: 'text-[#59a869] bg-[#59a86915]',
      yellow: 'text-[#f2c55c] bg-[#f2c55c15]',
      purple: 'text-[#a682e6] bg-[#a682e615]',
      red: 'text-[#e06c75] bg-[#e06c7515]'
   };

   return (
      <div className="jb-card p-5 group relative overflow-hidden">
         <div className="flex justify-between items-start mb-4">
            <div className={`p-2.5 rounded-[var(--radius)] ${colorMap[color] || colorMap.blue} transition-colors`}>
               <Icon size={20} />
            </div>
            {trend && (
               <div className="flex items-center gap-1 text-[10px] font-bold text-[#59a869] bg-[#59a86910] px-2 py-0.5 rounded-full border border-[#59a86920] uppercase tracking-wider">
                  <ArrowUpRight size={10} /> {trend}
               </div>
            )}
         </div>
         <div className="relative z-10">
            <div className="text-3xl font-bold text-[#dfe1e5] tracking-tight font-sans">{value}</div>
            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">{title}</div>
         </div>
         <div className={`absolute -right-6 -bottom-6 w-24 h-24 rounded-full blur-[40px] opacity-0 group-hover:opacity-20 transition-opacity bg-current ${colorMap[color]?.split(' ')[0]}`}></div>
      </div>
   );
};

const ServiceHealthMatrix = () => {
   const services = [
      { name: 'AUTH_GATEWAY', status: 'OPERATIONAL', lat: '12ms', load: 45, icon: Lock },
      { name: 'CORE_DB_SHARD_01', status: 'OPERATIONAL', lat: '4ms', load: 78, icon: Database },
      { name: 'EDGE_ROUTER_EU', status: 'DEGRADED', lat: '145ms', load: 92, icon: Globe },
      { name: 'STORAGE_BLOB_S3', status: 'OPERATIONAL', lat: '24ms', load: 12, icon: HardDrive },
      { name: 'MSG_QUEUE_KAFKA', status: 'OPERATIONAL', lat: '8ms', load: 33, icon: Wifi },
   ];

   return (
      <div className="h-full flex flex-col gap-2 overflow-y-auto pr-2 custom-scrollbar">
         {services.map((svc, i) => (
            <div key={i} className="flex items-center justify-between p-3 rounded-[var(--radius)] bg-[#1e1f22]/50 border border-[#43454a] hover:border-[#6a6e75] hover:bg-[#393b4040] transition-all group">
               <div className="flex items-center gap-3">
                  <div className={`p-2 rounded-[var(--radius)] bg-[#2b2d30] border border-[#43454a] ${svc.status === 'DEGRADED' ? 'text-[#f2c55c]' : 'text-[#59a869]'}`}>
                     <svc.icon size={14} />
                  </div>
                  <div>
                     <div className="text-[12px] font-bold text-[#dfe1e5] group-hover:text-[#3574f0] transition-colors">{svc.name}</div>
                     <div className="text-[10px] text-[#6e7073] font-bold flex items-center gap-2 uppercase">
                        <span className={svc.status === 'OPERATIONAL' ? 'text-[#59a869]' : 'text-[#f2c55c]'}>{svc.status}</span>
                        <span className="opacity-30">|</span>
                        <span>{svc.lat}</span>
                     </div>
                  </div>
               </div>

               <div className="w-24 flex flex-col gap-1.5 items-end">
                  <div className="w-full h-1.5 bg-[#43454a] rounded-full overflow-hidden">
                     <div
                        className={`h-full transition-all duration-500 ${svc.load > 90 ? 'bg-[#e06c75]' : svc.load > 70 ? 'bg-[#f2c55c]' : 'bg-[#59a869]'}`}
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
         <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
            <div>
               <h1 className="text-2xl font-bold text-[#dfe1e5] tracking-tight mb-1">Command Center</h1>
               <div className="flex items-center gap-6 text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">
                  <span className="flex items-center gap-2">
                     <span className="w-2 h-2 bg-[#59a869] rounded-full shadow-[0_0_8px_rgba(89,168,105,0.4)]"></span>
                     GRID: {loading ? 'CONNECTING...' : 'OPERATIONAL'}
                  </span>
                  <span className="flex items-center gap-2">
                     <span className="w-2 h-2 bg-[#3574f0] rounded-full shadow-[0_0_8px_rgba(53,116,240,0.4)]"></span>
                     SYNC: VERIFIED
                  </span>
                  <span>LATENCY: 12ms</span>
               </div>
            </div>

            <div className="flex gap-2">
               <button className="px-4 py-2 bg-[#2b2d30] hover:bg-[#393b40] border border-[#43454a] text-[11px] font-bold text-[#dfe1e5] rounded-[var(--radius)] transition-all uppercase tracking-tight">
                  GENERATE REPORT
               </button>
               <button className="px-4 py-2 bg-[#3574f015] hover:bg-[#3574f025] border border-[#3574f030] text-[11px] font-bold text-[#3574f0] rounded-[var(--radius)] transition-all uppercase tracking-tight">
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
            <div className="jb-card flex flex-col h-[300px] xl:h-full">
               <div className="flex items-center gap-2 px-4 py-3 border-b border-[#43454a]">
                  <Layers size={14} className="text-[#3574f0]" />
                  <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Service Mesh Topology</span>
               </div>
               <div className="p-4 flex-1 overflow-hidden">
                  <ServiceHealthMatrix />
               </div>
            </div>

            {/* Col 2: Resource Monitor */}
            <div className="flex flex-col gap-4 h-full">
               <div className="jb-card flex-1 flex flex-col">
                  <div className="flex items-center gap-2 px-4 py-2 border-b border-[#43454a]">
                     <Activity size={12} className="text-[#59a869]" />
                     <span className="text-[10px] font-bold text-[#dfe1e5] uppercase tracking-wider">Network Traffic</span>
                  </div>
                  <div className="flex-1 flex flex-col items-center justify-center p-5">
                     <div className="flex items-end gap-1.5 w-full h-20 justify-between mb-4">
                        {[40, 60, 45, 70, 30, 55, 65, 40, 80, 50, 60, 75, 45, 85, 55].map((h, i) => (
                           <div key={i} className="flex-1 bg-[#59a86920] hover:bg-[#59a869] transition-all rounded-[1px]" style={{ height: `${h}%` }}></div>
                        ))}
                     </div>
                     <div className="flex justify-between w-full text-[11px] font-bold font-mono">
                        <div className="flex flex-col uppercase tracking-tighter">
                           <span className="text-[#6e7073] mb-0.5">INBOUND</span>
                           <span className="text-[#dfe1e5] text-lg">4.2 GB/s</span>
                        </div>
                        <div className="flex flex-col items-end uppercase tracking-tighter">
                           <span className="text-[#6e7073] mb-0.5">OUTBOUND</span>
                           <span className="text-[#dfe1e5] text-lg">1.8 GB/s</span>
                        </div>
                     </div>
                  </div>
               </div>

               <div className="jb-card flex-1 flex flex-col">
                  <div className="flex items-center gap-2 px-4 py-2 border-b border-[#43454a]">
                     <HardDrive size={12} className="text-[#a682e6]" />
                     <span className="text-[10px] font-bold text-[#dfe1e5] uppercase tracking-wider">Storage Clusters</span>
                  </div>
                  <div className="p-5 space-y-4">
                     <div className="space-y-2">
                        <div className="flex justify-between text-[10px] font-bold text-[#6e7073] uppercase tracking-wider"><span>NVMe_POOL_A</span> <span className="text-[#3574f0]">78%</span></div>
                        <div className="h-2 w-full bg-[#1e1f22] rounded-full overflow-hidden border border-[#43454a]">
                           <div className="h-full bg-[#3574f0] shadow-[0_0_12px_rgba(53,116,240,0.4)] w-[78%] transition-all duration-1000"></div>
                        </div>
                     </div>
                     <div className="space-y-2">
                        <div className="flex justify-between text-[10px] font-bold text-[#6e7073] uppercase tracking-wider"><span>ARCHIVE_COLD</span> <span className="text-[#a682e6]">22%</span></div>
                        <div className="h-2 w-full bg-[#1e1f22] rounded-full overflow-hidden border border-[#43454a]">
                           <div className="h-full bg-[#a682e6] shadow-[0_0_12px_rgba(166,130,230,0.4)] w-[22%] transition-all duration-1000"></div>
                        </div>
                     </div>
                  </div>
               </div>
            </div>

            {/* Col 3: System Events */}
            <div className="jb-card flex flex-col h-full overflow-hidden">
               <div className="flex items-center gap-2 px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
                  <Bell size={14} className="text-[#f2c55c]" />
                  <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Live Event Log</span>
               </div>
               <div className="flex-1 overflow-y-auto custom-scrollbar">
                  {MOCK_LOGS.map((log) => (
                     <div key={log.id} className="flex gap-3 p-3 border-b border-[#43454a] hover:bg-[#3574f005] transition-colors text-[12px] group">
                        <div className="mt-0.5 text-[#6e7073] group-hover:text-[#3574f0] transition-colors">
                           <Clock size={12} />
                        </div>
                        <div className="flex-1 min-w-0">
                           <div className="flex justify-between mb-0.5">
                              <span className="font-bold text-[#dfe1e5] group-hover:text-[#3574f0]">{log.source}</span>
                              <span className="text-[10px] text-[#6e7073] font-bold uppercase">{log.timestamp}</span>
                           </div>
                           <p className="text-[#6e7073] group-hover:text-[#dfe1e5] transition-colors truncate">{log.message}</p>
                        </div>
                     </div>
                  ))}
               </div>
            </div>

         </div>

         {/* Bottom Section: Main Chart */}
         <div className="h-[380px]">
            <div className="jb-card h-full flex flex-col">
               <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a]">
                  <div className="flex items-center gap-2">
                     <Zap size={14} className="text-[#f2c55c]" />
                     <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Cluster Load Distribution (24h)</span>
                  </div>
                  <div className="flex gap-4 text-[10px] font-bold uppercase tracking-tighter">
                     <div className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-[#59a869]"></div><span className="text-[#6e7073]">Real Data</span></div>
                     <div className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full border border-[#3574f0]"></div><span className="text-[#6e7073]">Prediction</span></div>
                  </div>
               </div>
               <div className="flex-1 p-4" style={{ contain: 'layout paint' }}>
                  <ResponsiveContainer width="100%" height="100%" debounce={200}>
                     <AreaChart data={STATIC_CHART_DATA} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
                        <defs>
                           <linearGradient id="colorLoad" x1="0" y1="0" x2="0" y2="1">
                              <stop offset="5%" stopColor="#59a869" stopOpacity={0.2} />
                              <stop offset="95%" stopColor="#59a869" stopOpacity={0} />
                           </linearGradient>
                           <linearGradient id="colorPred" x1="0" y1="0" x2="0" y2="1">
                              <stop offset="5%" stopColor="#3574f0" stopOpacity={0.1} />
                              <stop offset="95%" stopColor="#3574f0" stopOpacity={0} />
                           </linearGradient>
                        </defs>
                        <CartesianGrid strokeDasharray="3 3" stroke="#43454a30" vertical={false} />
                        <XAxis
                           dataKey="time"
                           stroke="#43454a"
                           fontSize={10}
                           tickLine={false}
                           axisLine={false}
                           tick={{ fill: '#6e7073', fontWeight: 600 }}
                           dy={10}
                        />
                        <YAxis
                           stroke="#43454a"
                           fontSize={10}
                           tickLine={false}
                           axisLine={false}
                           tick={{ fill: '#6e7073', fontWeight: 600 }}
                        />
                        <Tooltip
                           contentStyle={{
                              backgroundColor: '#2b2d30',
                              border: '1px solid #43454a',
                              borderRadius: '4px',
                              boxShadow: '0 8px 30px rgba(0,0,0,0.5)',
                              padding: '8px 12px'
                           }}
                           itemStyle={{ fontSize: '12px', fontWeight: 700, padding: 0 }}
                           labelStyle={{ color: '#dfe1e5', marginBottom: '4px', fontSize: '10px', fontWeight: 800, textTransform: 'uppercase' }}
                           cursor={{ stroke: '#3574f0', strokeWidth: 1 }}
                        />
                        <Area
                           type="monotone"
                           dataKey="predicted"
                           stroke="#3574f0"
                           strokeDasharray="4 4"
                           strokeWidth={2}
                           fillOpacity={1}
                           fill="url(#colorPred)"
                           name="PREDICTION"
                        />
                        <Area
                           type="monotone"
                           dataKey="load"
                           stroke="#59a869"
                           strokeWidth={2}
                           fillOpacity={1}
                           fill="url(#colorLoad)"
                           name="SYSTEM LOAD"
                        />
                        <ReferenceLine
                           x="18:00"
                           stroke="#e06c75"
                           strokeDasharray="3 3"
                           label={{
                              position: 'top',
                              value: 'ANOMALY',
                              fill: '#e06c75',
                              fontSize: 9,
                              fontWeight: 900
                           }}
                        />
                     </AreaChart>
                  </ResponsiveContainer>
               </div>
            </div>
         </div>
      </div>
   );
};
