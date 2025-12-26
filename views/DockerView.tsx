import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { InteractiveTerminal } from '../components/InteractiveTerminal';
import { MOCK_CONTAINERS } from '../constants';
import { Box, Play, Square, RotateCw, Trash2, Terminal, Code } from 'lucide-react';
import { containersApi } from '../api';

export const DockerView: React.FC = () => {
  const [containers, setContainers] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTerminal, setActiveTerminal] = useState<{
    id: string;
    name: string;
  } | null>(null);

  useEffect(() => {
    const fetchContainers = async () => {
      try {
        const data = await containersApi.list();
        setContainers(data as any[]);
      } catch (error) {
        console.error('Failed to fetch containers:', error);
        setContainers(MOCK_CONTAINERS);
      } finally {
        setLoading(false);
      }
    };

    fetchContainers();
    const interval = setInterval(fetchContainers, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleStart = async (id: string) => {
    try {
      await containersApi.start(id);
      const data = await containersApi.list();
      setContainers(data as any[]);
    } catch (error) {
      console.error('Failed to start container:', error);
    }
  };

  const handleStop = async (id: string) => {
    try {
      await containersApi.stop(id);
      const data = await containersApi.list();
      setContainers(data as any[]);
    } catch (error) {
      console.error('Failed to stop container:', error);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this container?')) return;
    try {
      await containersApi.delete(id);
      const data = await containersApi.list();
      setContainers(data as any[]);
    } catch (error) {
      console.error('Failed to delete container:', error);
    }
  };

  const handleOpenShell = (id: string, name: string) => {
    setActiveTerminal({ id, name });
  };

  if (activeTerminal) {
    return (
      <div className="h-full p-6">
        <InteractiveTerminal
          type="container"
          targetId={activeTerminal.id}
          targetName={activeTerminal.name}
          onClose={() => setActiveTerminal(null)}
        />
      </div>
    );
  }

  return (
    <div className="h-full p-6 flex flex-col gap-6 max-w-[1800px] mx-auto overflow-hidden">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-3 text-[#dfe1e5] tracking-tight">
            <div className="p-2 rounded-[var(--radius)] bg-[#3574f015] border border-[#3574f020]">
              <Box className="text-[#3574f0]" size={20} />
            </div>
            Docker Orchestration
          </h1>
          <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">Direct Socket Access // Bollard Engine Active</p>
        </div>
        <div className="flex gap-2">
          <button className="px-4 py-2 bg-[#2b2d30] hover:bg-[#393b40] border border-[#43454a] text-[11px] font-bold text-[#dfe1e5] rounded-[var(--radius)] transition-all uppercase tracking-tight">
            PRUNE SYSTEM
          </button>
          <button className="px-4 py-2 bg-[#3574f015] hover:bg-[#3574f025] border border-[#3574f030] text-[11px] font-bold text-[#3574f0] rounded-[var(--radius)] transition-all uppercase tracking-tight">
            PULL IMAGE
          </button>
        </div>
      </div>

      <div className="jb-card flex-1 flex flex-col min-h-0 overflow-hidden shadow-2xl">
        <div className="flex items-center gap-2 px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
          <Code size={14} className="text-[#3574f0]" />
          <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Active Containers ({containers.length})</span>
        </div>
        <div className="flex-1 overflow-auto custom-scrollbar">
          <table className="w-full text-left text-[12px] border-collapse relative">
            <thead className="sticky top-0 bg-[#2b2d30] z-10 border-b border-[#43454a] font-bold uppercase tracking-wider text-[10px] text-[#6e7073]">
              <tr>
                <th className="p-4">ID</th>
                <th className="p-4">Image</th>
                <th className="p-4">Name</th>
                <th className="p-4">Node</th>
                <th className="p-4">State</th>
                <th className="p-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="font-mono">
              {(loading ? MOCK_CONTAINERS : containers).map((container) => (
                <tr key={container.id} className="border-b border-[#43454a]/50 hover:bg-[#3574f005] group transition-colors">
                  <td className="p-4 text-[#6e7073]">{container.id.substring(0, 8)}</td>
                  <td className="p-4 text-[#f2c55c] font-medium">{container.image}</td>
                  <td className="p-4 font-bold text-[#dfe1e5]">{container.name}</td>
                  <td className="p-4 text-[#6e7073] uppercase text-[10px] font-bold">{container.node}</td>
                  <td className="p-4">
                    <span className={`px-2 py-0.5 text-[10px] uppercase font-bold rounded-[4px] border transition-all ${container.state === 'RUNNING'
                        ? 'border-[#59a86930] text-[#59a869] bg-[#59a86910]'
                        : 'border-[#e06c7530] text-[#e06c75] bg-[#e06c7510]'
                      }`}>
                      {container.status}
                    </span>
                  </td>
                  <td className="p-4 text-right">
                    <div className="flex justify-end gap-1.5 opacity-60 group-hover:opacity-100 transition-all">
                      <button
                        onClick={() => handleOpenShell(container.id, container.name)}
                        className="p-1.5 hover:bg-[#3574f015] border border-[#43454a] rounded-[var(--radius)] text-[#6e7073] hover:text-[#3574f0] transition-all"
                        title="Open interactive shell"
                      >
                        <Terminal size={14} />
                      </button>

                      {container.state !== 'RUNNING' ? (
                        <button
                          onClick={() => handleStart(container.id)}
                          className="p-1.5 hover:bg-[#59a86915] border border-[#43454a] rounded-[var(--radius)] text-[#6e7073] hover:text-[#59a869] transition-all"
                          title="Start Container"
                        >
                          <Play size={14} />
                        </button>
                      ) : (
                        <button
                          onClick={() => handleStop(container.id)}
                          className="p-1.5 hover:bg-[#f2c55c15] border border-[#43454a] rounded-[var(--radius)] text-[#6e7073] hover:text-[#f2c55c] transition-all"
                          title="Stop Container"
                        >
                          <Square size={14} />
                        </button>
                      )}

                      <button
                        onClick={() => handleDelete(container.id)}
                        className="p-1.5 hover:bg-[#e06c7515] border border-[#43454a] rounded-[var(--radius)] text-[#6e7073] hover:text-[#e06c75] transition-all"
                        title="Remove Container"
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Modernized Console Preview */}
      <div className="h-40 bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] overflow-hidden flex flex-col shadow-inner">
        <div className="flex items-center gap-2 px-3 py-1.5 bg-[#2b2d30] border-b border-[#43454a]">
          <Terminal size={12} className="text-[#6e7073]" />
          <span className="text-[10px] font-bold text-[#6e7073] uppercase tracking-wider">Engine Output</span>
        </div>
        <div className="flex-1 p-3 font-mono text-[11px] overflow-y-auto custom-scrollbar">
          <div className="text-[#6e7073] mb-1">{'root@worpen-core:~# docker ps --format "table {{.ID}}\t{{.Image}}\t{{.Status}}"'}</div>
          {MOCK_CONTAINERS.map(c => (
            <div key={c.id} className="text-[#59a869] whitespace-pre opacity-90 leading-relaxed">
              {`${c.id.substring(0, 8).padEnd(15)} ${c.image.padEnd(30)} ${c.status}`}
            </div>
          ))}
          <div className="flex gap-1 mt-2 items-center">
            <span className="text-[#3574f0] font-bold">root@worpen-core:~#</span>
            <span className="animate-pulse block w-2 h-4 bg-[#3574f0]"></span>
          </div>
        </div>
      </div>
    </div>
  );
};