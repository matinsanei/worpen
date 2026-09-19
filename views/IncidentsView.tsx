
import React, { useState, useEffect } from 'react';
import { AlertTriangle, Search, Filter, Clock, ChevronRight, ShieldAlert, History, MoreHorizontal, Loader2, CheckCircle } from 'lucide-react';
import { incidentsApi, auditApi, AuditEntry } from '../api';
import { Incident } from '../types';
import { useNotifications } from '../components/NotificationSystem';

export const IncidentsView: React.FC = () => {
    const [searchTerm, setSearchTerm] = useState('');
    const [incidents, setIncidents] = useState<Incident[]>([]);
    const [loading, setLoading] = useState(true);
    const [loadError, setLoadError] = useState<string | null>(null);
    const [resolvingId, setResolvingId] = useState<string | null>(null);
    const [failures, setFailures] = useState<AuditEntry[]>([]);
    const { addNotification } = useNotifications();

    const fetchIncidents = async () => {
        try {
            setLoading(true);
            const [incData, auditData] = await Promise.all([
                incidentsApi.list(),
                auditApi.recent(100).catch(() => ({ count: 0, entries: [] as AuditEntry[] })),
            ]);
            setIncidents(incData as Incident[]);
            setFailures(auditData.entries.filter(e => !e.success));
            setLoadError(null);
        } catch (err: any) {
            setLoadError(err.message || 'Failed to fetch incidents');
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchIncidents();
    }, []);

    const handleResolve = async (id: string) => {
        setResolvingId(id);
        try {
            await incidentsApi.resolve(id);
            addNotification('SUCCESS', 'INCIDENT_RESOLVED', `Incident ${id} marked as resolved.`, 3000);
            await fetchIncidents();
        } catch (err: any) {
            addNotification('ERROR', 'RESOLVE_FAILED', err.message || 'Failed to resolve incident');
        } finally {
            setResolvingId(null);
        }
    };

    const filtered = incidents.filter((inc) => {
        const q = searchTerm.toLowerCase();
        return !q || inc.service.toLowerCase().includes(q) || inc.issue.toLowerCase().includes(q) || inc.node.toLowerCase().includes(q);
    });
    const filteredFailures = failures.filter((f) => {
        const q = searchTerm.toLowerCase();
        return !q || f.route_name.toLowerCase().includes(q) || (f.error || '').toLowerCase().includes(q);
    });
    const pendingCount = incidents.filter(i => i.status !== 'RESOLVED').length;

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
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-[#6e7073] transition-colors group-focus-within:text-[#e85d26]" size={14} />
                        <input
                            type="text"
                            placeholder="Search incidents..."
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                            className="bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] pl-9 pr-3 py-1.5 rounded-[var(--radius)] text-[12px] focus:border-[#e85d26] outline-none transition-all w-64 shadow-inner"
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
                            <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-[#e06c75]"></div> Open: {pendingCount}</span>
                            <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-[#59a869]"></div> Total: {incidents.length}</span>
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
                                {loading && (
                                    <tr><td colSpan={5} className="p-8 text-center text-[#6e7073] text-[12px]">
                                        <Loader2 size={16} className="animate-spin inline mr-2" />Loading incidents...
                                    </td></tr>
                                )}
                                {!loading && loadError && (
                                    <tr><td colSpan={5} className="p-8 text-center text-[#e06c75] text-[12px]">
                                        {loadError} <button onClick={fetchIncidents} className="underline ml-2">Retry</button>
                                    </td></tr>
                                )}
                                {!loading && !loadError && filtered.length === 0 && (
                                    <tr><td colSpan={5} className="p-8 text-center text-[#6e7073] text-[12px] italic">
                                        No incidents found. The grid is quiet.
                                    </td></tr>
                                )}
                                {filtered.map((inc) => (
                                    <tr key={inc.id} className="hover:bg-[#e85d2605] group transition-colors cursor-pointer">
                                        <td className="px-4 py-4 text-[#6e7073] font-mono text-[11px] border-r border-[#43454a]">
                                            <div className="flex items-center gap-2">
                                                <Clock size={12} className="opacity-50" />
                                                {inc.time}
                                            </div>
                                        </td>
                                        <td className="px-4 py-4 font-semibold text-[#dfe1e5] text-[12px] border-r border-[#43454a]">
                                            <div className="flex items-center gap-2">
                                                <div className="w-1 h-3 rounded-full bg-[#e85d26]"></div>
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
                                            <div className="flex items-center justify-between gap-2">
                                                <span className={`px-2.5 py-0.5 rounded-full text-[10px] font-bold border ${inc.status === 'RESOLVED'
                                                        ? 'bg-[#59a86910] text-[#59a869] border-[#59a86930]'
                                                        : 'bg-[#e06c7510] text-[#e06c75] border-[#e06c7530] animate-pulse'
                                                    }`}>
                                                    {inc.status}
                                                </span>
                                                {inc.status !== 'RESOLVED' && (
                                                    <button
                                                        onClick={() => handleResolve(inc.id)}
                                                        disabled={resolvingId === inc.id}
                                                        className="flex items-center gap-1 px-2.5 py-1 bg-[#59a86915] hover:bg-[#59a86930] border border-[#59a86930] text-[#59a869] text-[10px] font-bold rounded-[var(--radius)] transition-all disabled:opacity-50"
                                                        title="Mark as resolved"
                                                    >
                                                        {resolvingId === inc.id ? <Loader2 size={11} className="animate-spin" /> : <CheckCircle size={11} />}
                                                        RESOLVE
                                                    </button>
                                                )}
                                                <ChevronRight size={14} className="text-[#6e7073] opacity-0 group-hover:opacity-100 transition-all -translate-x-2 group-hover:translate-x-0" />
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>

                {/* Failed executions (real audit trail) */}
                <div className="jb-card overflow-hidden flex flex-col bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)]">
                    <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]">
                        <div className="flex items-center gap-2">
                            <AlertTriangle size={14} className="text-[#e85d26]" />
                            <span className="text-[12px] font-bold uppercase tracking-tight text-[#dfe1e5]">Failed Executions</span>
                        </div>
                        <span className="text-[11px] text-[#6e7073] font-medium">from live audit trail · {filteredFailures.length} shown</span>
                    </div>
                    <div className="overflow-x-auto">
                        <table className="w-full text-left border-collapse">
                            <thead className="bg-[#1e1f22] text-[#6e7073] text-[10px] font-bold uppercase tracking-widest border-b border-[#43454a] sticky top-0">
                                <tr>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Time</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Route / Pipeline</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Path</th>
                                    <th className="px-4 py-3 border-r border-[#43454a]">Error</th>
                                    <th className="px-4 py-3">Via</th>
                                </tr>
                            </thead>
                            <tbody className="divide-y divide-[#43454a]">
                                {filteredFailures.length === 0 && (
                                    <tr><td colSpan={5} className="p-8 text-center text-[#6e7073] text-[12px] italic">
                                        No failed executions recorded. Clean sheet.
                                    </td></tr>
                                )}
                                {filteredFailures.map((f, i) => (
                                    <tr key={`${f.route_id}-${f.ts}-${i}`} className="hover:bg-[#e85d2605] transition-colors">
                                        <td className="px-4 py-3 text-[#6e7073] font-mono text-[11px] border-r border-[#43454a] whitespace-nowrap">
                                            {f.ts.slice(0, 19).replace('T', ' ')}
                                        </td>
                                        <td className="px-4 py-3 font-semibold text-[#dfe1e5] text-[12px] border-r border-[#43454a]">
                                            <div className="flex items-center gap-2">
                                                <div className="w-1 h-3 rounded-full bg-[#e06c75]"></div>
                                                {f.route_name}
                                            </div>
                                        </td>
                                        <td className="px-4 py-3 text-[#6e7073] text-[12px] font-mono border-r border-[#43454a]">{f.path}</td>
                                        <td className="px-4 py-3 border-r border-[#43454a]">
                                            <span className="text-[12px] font-medium text-[#e06c75]">{f.error || 'unknown error'}</span>
                                        </td>
                                        <td className="px-4 py-4">
                                            <span className="px-2.5 py-0.5 rounded-full text-[10px] font-bold border bg-[#e85d2610] text-[#e85d26] border-[#e85d2630]">
                                                {f.via.toUpperCase()}
                                            </span>
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
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#e85d2610] flex items-center justify-center text-[#e85d26]">
                            <ShieldAlert size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Open Incidents</div>
                            <div className="text-xl font-bold">{pendingCount}</div>
                        </div>
                    </div>
                    <div className="jb-card p-4 flex items-center gap-4 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] shadow-sm">
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#59a86910] flex items-center justify-center text-[#59a869]">
                            <Clock size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Resolved</div>
                            <div className="text-xl font-bold">{incidents.length - pendingCount}</div>
                        </div>
                    </div>
                    <div className="jb-card p-4 flex items-center gap-4 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] shadow-sm">
                        <div className="w-10 h-10 rounded-[var(--radius)] bg-[#e06c7510] flex items-center justify-center text-[#e06c75]">
                            <History size={20} />
                        </div>
                        <div>
                            <div className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider">Total Tracked</div>
                            <div className="text-xl font-bold">{incidents.length}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
