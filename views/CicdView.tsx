
import React, { useState, useEffect } from 'react';
import { pipelinesApi, auditApi, AuditEntry, PipelineListItem, PipelineStageView } from '../api';
import type { PipelineRunResult } from '../api';
import { GitBranch, PlayCircle, Clock, CheckCircle, XCircle, Loader2, Terminal, Code, Cpu, Activity, ShieldCheck, RefreshCw, MinusCircle } from 'lucide-react';

interface StageNodeProps {
  stage: PipelineStageView;
  index: number;
}

const stageStatusOf = (s: PipelineStageView): string => s.last_status || 'PENDING';

const stageDurationOf = (s: PipelineStageView): string =>
  s.last_duration_ms != null ? `${s.last_duration_ms}ms` : '—';

const StageNode: React.FC<StageNodeProps> = ({ stage, index }) => {
  const status = stageStatusOf(stage);
  const getStatusStyles = (s: string) => {
    switch (s) {
      case 'SUCCESS': return {
        border: 'border-[#59a86940]',
        text: 'text-[#59a869]',
        bg: 'bg-[#59a86910]',
        glow: 'shadow-[0_0_15px_rgba(89,168,105,0.15)]',
        icon: <CheckCircle size={14} className="text-[#59a869]" />
      };
      case 'FAILED': return {
        border: 'border-[#e06c7540]',
        text: 'text-[#e06c75]',
        bg: 'bg-[#e06c7510]',
        glow: 'shadow-[0_0_15px_rgba(224,108,117,0.15)]',
        icon: <XCircle size={14} className="text-[#e06c75]" />
      };
      case 'RUNNING': return {
        border: 'border-[#f2c55c60]',
        text: 'text-[#f2c55c]',
        bg: 'bg-[#f2c55c05]',
        glow: 'shadow-[0_0_20px_rgba(242,197,92,0.2)]',
        icon: <Loader2 size={14} className="text-[#f2c55c] animate-spin" />
      };
      case 'SKIPPED': return {
        border: 'border-[#43454a]',
        text: 'text-[#6e7073]',
        bg: 'bg-[#2b2d3080]',
        glow: '',
        icon: <MinusCircle size={14} className="text-[#6e7073]" />
      };
      default: return {
        border: 'border-[#43454a]',
        text: 'text-[#6e7073]',
        bg: 'bg-[#2b2d3080]',
        glow: '',
        icon: <Activity size={14} className="text-[#6e7073]" />
      };
    }
  };

  const styles = getStatusStyles(status);

  return (
    <div className="relative group flex flex-col items-center">
      {/* Connection Point Left */}
      <div className="absolute left-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-[#43454a] z-10 border border-[#1e1f22]"></div>

      <div className={`w-40 min-h-[90px] p-3 rounded-lg border-2 ${styles.border} ${styles.bg} ${styles.glow} flex flex-col justify-between items-start gap-2 relative transition-all duration-300 group-hover:scale-[1.02] group-hover:border-opacity-100 backdrop-blur-sm`}>
        <div className="flex justify-between items-center w-full">
          <span className={`text-[10px] font-black uppercase tracking-[0.1em] ${styles.text}`}>
            {stage.name}
          </span>
          {styles.icon}
        </div>

        <div className="w-full space-y-2">
          <div className="flex items-center gap-1.5 text-[9px] font-mono text-[#6e7073]">
            <Clock size={10} />
            {stageDurationOf(stage)}
          </div>
          <div className="text-[9px] font-bold uppercase tracking-widest text-[#6e7073]">
            {status}{stage.allow_failure ? ' · may fail' : ''}
          </div>
        </div>

        {status === 'RUNNING' && (
          <div className="w-full h-1 bg-[#43454a]/30 rounded-full overflow-hidden mt-1">
            <div className="h-full bg-[#f2c55c] animate-[loading-bar_1.5s_infinite_linear] w-1/3 shadow-[0_0_8px_rgba(242,197,92,0.5)]"></div>
          </div>
        )}

        {/* Technical accents */}
        <div className="absolute top-0 right-0 p-1 opacity-20">
          <Cpu size={10} className={styles.text} />
        </div>

        {/* Tooltip */}
        <div className="absolute -bottom-12 left-1/2 -translate-x-1/2 bg-[#2b2d30] border border-[#43454a] text-[9px] font-mono text-[#dfe1e5] px-2 py-1 opacity-0 group-hover:opacity-100 transition-all z-20 whitespace-nowrap rounded shadow-2xl pointer-events-none translate-y-2 group-hover:translate-y-0">
          {stage.name} // STATUS: {status}
        </div>
      </div>

      {/* Connection Point Right */}
      <div className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-[#43454a] z-10 border border-[#1e1f22]"></div>
    </div>
  );
};

const ConnectingLine = ({ isFirst = false }: { isFirst?: boolean }) => {
  return (
    <div className="relative w-16 h-10 flex items-center justify-center">
      <svg width="64" height="40" viewBox="0 0 64 40" fill="none" xmlns="http://www.w3.org/2000/svg" className="absolute inset-0 overflow-visible">
        <path d="M0 20H64" stroke="#43454a" strokeWidth="2" strokeDasharray="4 4" className="opacity-40" />
        <path d="M0 20H64" stroke="url(#flowGradient)" strokeWidth="2" strokeDasharray="20 44" strokeLinecap="round">
          <animate attributeName="stroke-dashoffset" from="64" to="0" dur="2s" repeatCount="indefinite" />
        </path>
        <defs>
          <linearGradient id="flowGradient" x1="0" y1="0" x2="64" y2="0" gradientUnits="userSpaceOnUse">
            <stop stopColor="#e85d26" stopOpacity="0" />
            <stop offset="0.5" stopColor="#e85d26" />
            <stop offset="1" stopColor="#e85d26" stopOpacity="0" />
          </linearGradient>
        </defs>
      </svg>
    </div>
  );
};

const AuditFeed: React.FC<{ entries: AuditEntry[] }> = ({ entries }) => {
  if (entries.length === 0) {
    return (
      <div className="h-full flex items-center justify-center text-[#6e7073] text-[12px] font-mono">
        No executions recorded yet — run a route to populate the audit trail.
      </div>
    );
  }
  return (
    <div className="h-full bg-[#1e1f22] font-mono text-[11px] p-4 overflow-y-auto space-y-1.5 text-[#dfe1e5] border border-[#43454a] rounded-[var(--radius)] shadow-inner custom-scrollbar">
      {entries.map((e, i) => (
        <div key={`${e.route_id}-${e.ts}-${i}`} className="flex gap-3 leading-relaxed">
          <span className="text-[#6e7073] select-none text-[10px] whitespace-nowrap">{e.ts.slice(11, 19)}</span>
          <span className={`font-bold ${e.success ? 'text-[#59a869]' : 'text-[#e06c75]'}`}>
            {e.success ? 'OK ' : 'ERR'}
          </span>
          <span className="text-[#e85d26] font-bold">{e.route_name}</span>
          <span className="text-[#6e7073] truncate">{e.error ? e.error : `${e.execution_time_ms}ms · via ${e.via}`}</span>
        </div>
      ))}
    </div>
  );
};

export const CicdView: React.FC = () => {
  const [pipelines, setPipelines] = useState<PipelineListItem[]>([]);
  const [selectedPipeline, setSelectedPipeline] = useState<PipelineListItem | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [audit, setAudit] = useState<AuditEntry[]>([]);
  const [lastRun, setLastRun] = useState<PipelineRunResult | null>(null);

  const fetchPipelines = async () => {
    try {
      const data = await pipelinesApi.list();
      setPipelines(data);
      setSelectedPipeline(prev => {
        if (!data.length) return null;
        return data.find(p => p.id === prev?.id) || data[0];
      });
      setLoadError(null);
    } catch (err: any) {
      setLoadError(err.message || 'Failed to fetch pipelines');
    } finally {
      setLoading(false);
    }
  };

  const fetchAudit = async () => {
    try {
      const data = await auditApi.recent(50);
      setAudit(data.entries);
    } catch {
      // audit trail is best-effort; pipelines view still works without it
    }
  };

  useEffect(() => {
    fetchPipelines();
    fetchAudit();
    const interval = setInterval(fetchAudit, 5000);
    return () => clearInterval(interval);
  }, []);

  const triggerRun = async () => {
    if (!selectedPipeline || isRunning) return;
    setIsRunning(true);
    try {
      const run = await pipelinesApi.trigger(selectedPipeline.id);
      setLastRun(run);
      await fetchPipelines();
      await fetchAudit();
    } catch (err: any) {
      console.error('Trigger failed:', err);
    } finally {
      setIsRunning(false);
    }
  };

  const lastStatusOf = (p: PipelineListItem): string => {
    if (!p.enabled) return 'DISABLED';
    return p.last_run ? (p.last_run.success ? 'SUCCESS' : 'FAILED') : 'NEVER_RUN';
  };

  return (
    <div className="min-h-full p-6 flex flex-col gap-6 max-w-[1600px] mx-auto">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-3 text-[#dfe1e5] tracking-tight">
            <div className="p-2 rounded-[var(--radius)] bg-[#e85d2615] border border-[#e85d2620]">
              <GitBranch className="text-[#e85d26]" size={20} />
            </div>
            Pipeline Runner
          </h1>
          <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">Continuous Integration & Deployment Matrix</p>
        </div>
        <button
          onClick={triggerRun}
          disabled={!selectedPipeline || isRunning}
          className={`px-5 py-2.5 rounded-[var(--radius)] text-[11px] flex items-center gap-2 font-bold transition-all uppercase tracking-tight shadow-lg border
            ${isRunning || !selectedPipeline
              ? 'bg-[#2b2d30] border-[#43454a] text-[#f2c55c] cursor-not-allowed'
              : 'bg-[#59a869] border-[#59a869] text-white hover:bg-[#59a869e0] active:scale-95'
            }`}
        >
          {isRunning ? <Loader2 size={14} className="animate-spin" /> : <PlayCircle size={14} />}
          {isRunning ? 'Running...' : 'Trigger Pipeline'}
        </button>
      </div>

      <div className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">

        {/* Sidebar: Pipeline List */}
        <div className="lg:col-span-3 flex flex-col gap-3 overflow-y-auto pr-2 custom-scrollbar">
          {loading && (
            <div className="jb-card p-6 flex items-center justify-center gap-2 text-[#6e7073] text-[12px]">
              <Loader2 size={16} className="animate-spin" /> Loading pipelines...
            </div>
          )}
          {!loading && loadError && (
            <div className="jb-card p-4 text-[#e06c75] text-[12px] border-[#e06c7540]">
              {loadError}
              <button onClick={fetchPipelines} className="ml-2 underline hover:text-[#dfe1e5]">Retry</button>
            </div>
          )}
          {!loading && !loadError && pipelines.length === 0 && (
            <div className="jb-card p-6 text-center text-[#6e7073] text-[12px]">
              No pipelines registered on this server.
            </div>
          )}
          {pipelines.map(pipe => {
            const st = lastStatusOf(pipe);
            return (
            <div
              key={pipe.id}
              onClick={() => { setSelectedPipeline(pipe); setLastRun(null); }}
              className={`jb-card p-4 cursor-pointer transition-all group relative overflow-hidden ${selectedPipeline?.id === pipe.id
                ? 'border-[#e85d26] bg-[#e85d2608]'
                : 'hover:border-[#6a6e75] hover:bg-[#393b4040]'
                }`}
            >
              <div className="flex justify-between items-start mb-2">
                <span className={`font-bold text-[13px] truncate transition-colors ${selectedPipeline?.id === pipe.id ? 'text-[#e85d26]' : 'text-[#dfe1e5]'}`}>
                  {pipe.name}
                </span>
                {st === 'SUCCESS' && <div className="w-2 h-2 rounded-full bg-[#59a869] shadow-[0_0_8px_rgba(89,168,105,0.4)]"></div>}
                {st === 'FAILED' && <div className="w-2 h-2 rounded-full bg-[#e06c75] shadow-[0_0_8px_rgba(224,108,117,0.4)]"></div>}
                {st === 'NEVER_RUN' && <div className="w-2 h-2 rounded-full bg-[#43454a]"></div>}
                {st === 'DISABLED' && <div className="w-2 h-2 rounded-full bg-[#6e7073]"></div>}
              </div>
              <div className="flex items-center gap-2 text-[10px] text-[#6e7073] font-bold uppercase mb-3 px-1">
                <GitBranch size={12} className="opacity-50" /> v{pipe.version} · {pipe.stage_count} stages
              </div>
              <div className="text-[10px] font-mono text-[#6e7073] truncate border-t border-[#43454a] pt-2 px-1 italic">
                {pipe.last_run ? `last: ${pipe.last_run.success ? 'ok' : 'failed'} · ${pipe.last_run.total_time_ms}ms` : 'never run'}
              </div>
              {selectedPipeline?.id === pipe.id && (
                <div className="absolute top-0 left-0 w-1 h-full bg-[#e85d26]"></div>
              )}
            </div>
            );
          })}
        </div>

        {/* Main Content */}
        <div className="lg:col-span-9 flex flex-col gap-6">
          {!selectedPipeline ? (
            <div className="jb-card flex-1 min-h-[350px] flex items-center justify-center text-[#6e7073] text-[13px]">
              {loading ? 'Loading...' : 'Select a pipeline to inspect its flow.'}
            </div>
          ) : (
          <>
          {/* Pipeline Visualizer */}
          <div className="jb-card flex flex-col min-h-[350px] shadow-2xl overflow-hidden group">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/80 backdrop-blur-md z-10">
              <div className="flex items-center gap-2">
                <Code size={14} className="text-[#e85d26]" />
                <span className="text-[11px] font-black text-[#dfe1e5] uppercase tracking-widest">Flow Analysis: {selectedPipeline.name}</span>
              </div>
              <div className="flex gap-5 text-[10px] font-bold text-[#6e7073] uppercase tracking-tighter">
                <span className="flex items-center gap-1.5"><Terminal size={12} className="text-[#e85d26] opacity-70" /> v{selectedPipeline.version}</span>
                <span className="flex items-center gap-1.5"><Clock size={12} className="opacity-70" /> {selectedPipeline.last_run ? `${selectedPipeline.last_run.total_time_ms}ms last run` : 'never run'}</span>
              </div>
            </div>

            <div className="flex-1 relative bg-[#1e1f22] overflow-x-auto custom-scrollbar">
              {/* Background Grid Pattern */}
              <div className="absolute inset-0 opacity-[0.03] pointer-events-none"
                style={{ backgroundImage: 'radial-gradient(#dfe1e5 1px, transparent 1px)', backgroundSize: '24px 24px' }}></div>

              <div className="absolute inset-0 flex items-center justify-center min-w-fit p-12">
                <div className="flex items-center">
                  {/* Source Node */}
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-12 h-12 rounded-xl border-2 border-[#e85d2640] bg-[#e85d2610] flex items-center justify-center text-[#e85d26] shadow-[0_0_20px_rgba(232,93,38,0.15)] relative group/source">
                      <GitBranch size={20} />
                      <div className="absolute inset-0 rounded-xl bg-[#e85d2610] animate-pulse pointer-events-none"></div>
                    </div>
                    <span className="text-[9px] font-black text-[#6e7073] uppercase tracking-widest">vcs_trigger</span>
                  </div>

                  <ConnectingLine isFirst={true} />

                  {selectedPipeline.stages.map((stage, idx) => (
                    <React.Fragment key={idx}>
                      <StageNode stage={stage} index={idx} />
                      {idx < selectedPipeline.stages.length - 1 && <ConnectingLine />}
                    </React.Fragment>
                  ))}

                  {/* Output Node */}
                  <ConnectingLine />
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-12 h-12 rounded-xl border-2 border-[#59a86940] bg-[#59a86910] flex items-center justify-center text-[#59a869] shadow-[0_0_20px_rgba(89,168,105,0.15)] relative">
                      <ShieldCheck size={20} />
                    </div>
                    <span className="text-[9px] font-black text-[#6e7073] uppercase tracking-widest">ready_prod</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Last run result */}
          {lastRun && (
            <div className={`jb-card p-4 flex items-center gap-3 border ${lastRun.success ? 'border-[#59a86940]' : 'border-[#e06c7540]'}`}>
              {lastRun.success
                ? <CheckCircle size={18} className="text-[#59a869] flex-shrink-0" />
                : <XCircle size={18} className="text-[#e06c75] flex-shrink-0" />}
              <div className="text-[12px]">
                <span className="font-bold text-[#dfe1e5]">{lastRun.success ? 'Run succeeded' : 'Run failed'}</span>
                <span className="text-[#6e7073] font-mono ml-2">{lastRun.total_time_ms}ms · {lastRun.stages.length} stages · {lastRun.finished_at.slice(11, 19)}</span>
              </div>
            </div>
          )}

          {/* Live Logs */}
          <div className="jb-card flex-1 flex flex-col min-h-[300px] overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
              <div className="flex items-center gap-2">
                <Terminal size={14} className="text-[#e85d26]" />
                <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Execution Audit Trail</span>
              </div>
              <button onClick={fetchAudit} className="flex items-center gap-1.5 text-[10px] font-bold text-[#6e7073] hover:text-[#dfe1e5] uppercase transition-colors">
                <RefreshCw size={12} /> Refresh
              </button>
            </div>
            <div className="flex-1 p-4 bg-[#1e1f22] overflow-hidden flex flex-col">
              <div className="flex-1 min-h-0 relative">
                <div className="absolute inset-0">
                  <AuditFeed entries={audit} />
                </div>
              </div>
            </div>
          </div>
          </>
          )}
        </div>

      </div>

      <style dangerouslySetInnerHTML={{
        __html: `
                @keyframes loading-bar {
                    0% { transform: translateX(-100%); }
                    100% { transform: translateX(300%); }
                }
            `}} />
    </div>
  );
};
