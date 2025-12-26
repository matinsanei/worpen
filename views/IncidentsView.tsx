
import React, { useState } from 'react';
import { AlertTriangle, Search, Filter, Clock, ChevronRight, ShieldAlert, History, MoreHorizontal } from 'lucide-react';
import { MOCK_INCIDENTS } from '../constants';

export const IncidentsView: React.FC = () => {
    const [searchTerm, setSearchTerm] = useState('');

    return (
        <div className="h-full flex flex-col bg-[#1e1f22] text-[#dfe1e5]">
            {/* Header / Toolbar */}
            <div className="flex justify-between items-center px-6 py-4 border-b border-[#43454a] bg-[#2b2d30] shadow-sm select-none">
                <div>
                    <h1 className="text-xl font-bold flex items-center gap-3">
                        <div className="p-2 rounded-[var(--radius)] bg-[#e06c7515] border border-[#e06c7520]">
                            <AlertTriangle className="text-[#e06c75]" size={20} />
                        </div>
                        Incidents
                    </h1>
                    <p className="text-[11px] text-[#6e7073] mt-1 font-medium tracking-wide uppercase">Critical Alerts & System Failures</p>
                </div>

                <div className="flex items-center gap-3">
                    <div className="relative group">
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-[#6e7073] transition-colors group-focus-within:text-[#3574f0]" size={14} />
                        <input
                            type="text"
                            placeholder="Search incidents..."
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                            className="bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] pl-9 pr-3 py-1.5 rounded-[var(--radius)] text-[12px] focus:border-[#3574f0] outline-none transition-all w-64 shadow-inner"
                        />
                    </div>
                    <button className="p-2 bg-[#2b2d30] border border-[#43454a] text-[#dfe1e5] rounded-[var(--radius)] hover:bg-[#393b40] transition-all">
                        <Filter size={16} />
                    </button>
                    <button className="flex items-center gap-2 px-4 py-2 bg-[#2b2d30] border border-[#43454a] text-[#dfe1e5] text-[11px] font-bold rounded-[var(--radius)] hover:bg-[#393b40] transition-all shadow-sm">
                        <History size={14} />
                        REPORT_LOGS
                    </button>
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-y-auto p-6 flex flex-col gap-6 custom-scrollbar pb-12">
                <div className="jb-card overflow-hidden flex flex-col bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)]">
                    <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]">
                        <div className="flex items-center gap-2">
                            <ShieldAlert size={14} className="text-[#e06c75]" />
                            <span className="text-[12px] font-bold uppercase tracking-tight text-[#dfe1e5]">Active Breach & Failure Log</span>
                        </div>
                        <div className="flex items-center gap-4 text-[11px] text-[#6e7073] font-medium">
                            <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-[#e06c75]"></div> Critical: 2</span>
                            <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-[#d19a66]"></div> Warning: 1</span>
                        </div>
                    </div>

                    <div className="overflow-x-auto">
                        <table className="w-full text-left border-collapse">
                            <thead className="bg-[#1e1f22] text-[#6e7073] text-[10px] font-bold uppercase tracking-widest border-b border-[#43454a] sticky top-0">
                                <tr>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Timestamp</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Service Cluster</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Affected Node</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Anomaly Detail</th>
                                    <th className="px-4 py-3">Resolution Status</th>
                                </tr>
                            </thead>
                            <tbody className="divide-y divide-[#43454a]">
                                {MOCK_INCIDENTS.map((inc) => (
                                    <tr key={inc.id} className="hover:bg-[#3574f005] group transition-colors cursor-pointer">
                                        <td className="px-4 py-4 text-[#6e7073] font-mono text-[11px] border-r border-[#43454a]">
                                            <div className="flex items-center gap-2">
                                                <Clock size={12} className="opacity-50" />
                                                {inc.time}
                                            </div>
                                        </td>
                                        <td className="px-4 py-4 font-semibold text-[#dfe1e5] text-[12px] border-r border-[#43454a]">
                                            <div className="flex items-center gap-2">
                                                <div className="w-1 h-3 rounded-full bg-[#3574f0]"></div>
                                                {inc.service}
                                            </div>
                                        </td>
                                        <td className="px-4 py-4 text-[#6e7073] text-[12px] font-mono border-r border-[#43454a]">
                                            {inc.node}
                                        </td>
                                        <td className="px-4 py-4 border-r border-[#43454a]">
                                            <div className="flex flex-col gap-1">
                                                <span className="text-[12px] font-bold text-[#e06c75]">{inc.issue}</span>
                                                <div className="flex items-center gap-1 text-[10px] text-[#6e7073] font-medium uppercase italic">
                                                    Action: {inc.action}
                                                </div>
                                            </div>
                                        </td>
                                        <td className="px-4 py-4">
                                            <div className="flex items-center justify-between">
                                                <span className={`px-2.5 py-0.5 rounded-full text-[10px] font-bold border ${inc.status === 'RESOLVED'
                                                        ? 'bg-[#59a86910] text-[#59a869] border-[#59a86930]'
                                                        : 'bg-[#e06c7510] text-[#e06c75] border-[#e06c7530] animate-pulse'
                                                    }`}>
                                                    {inc.status}
                                                </span>
                                                <ChevronRight size={14} className="text-[#6e7073] opacity-0 group-hover:opacity-100 transition-all -translate-x-2 group-hover:translate-x-0" />
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>

                {/* Additional Stats Section */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="jb-card p-4 flex items-center gap-4 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] shadow-sm">
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#3574f010] flex items-center justify-center text-[#3574f0]">
                            <ShieldAlert size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Health Score</div>
                            <div className="text-xl font-bold">98.2%</div>
                        </div>
                    </div>
                    <div className="jb-card p-4 flex items-center gap-4 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] shadow-sm">
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#59a86910] flex items-center justify-center text-[#59a869]">
                            <Clock size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Avg Resolution</div>
                            <div className="text-xl font-bold">14m 32s</div>
                        </div>
                    </div>
                    <div className="jb-card p-4 flex items-center gap-4 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] shadow-sm">
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#e06c7510] flex items-center justify-center text-[#e06c75]">
                            <History size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Uptime Today</div>
                            <div className="text-xl font-bold">23h 58m</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
