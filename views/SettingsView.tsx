
import React, { useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { Save, Shield, Globe, Key, Plus, X, Settings, Bell, Hash } from 'lucide-react';
import { useNotifications } from '../components/NotificationSystem';

export const SettingsView: React.FC = () => {
  const { addNotification } = useNotifications();
  const [envVars, setEnvVars] = useState([
    { key: 'RUST_LOG', value: 'info' },
    { key: 'HIVE_REGION', value: 'eu-west-1' },
    { key: 'HEARTBEAT_INTERVAL', value: '5000' }
  ]);

  const [newKey, setNewKey] = useState('');
  const [newValue, setNewValue] = useState('');

  const addEnv = () => {
    if (newKey && newValue) {
      setEnvVars([...envVars, { key: newKey, value: newValue }]);
      setNewKey('');
      setNewValue('');
      addNotification('SUCCESS', 'ENV_VAR_ADDED', `Variable ${newKey} injected into pool.`);
    }
  };

  const removeEnv = (index: number) => {
    setEnvVars(envVars.filter((_, i) => i !== index));
    addNotification('INFO', 'ENV_VAR_REMOVED', 'Environment variable removed.');
  };

  const [apiUrl, setApiUrl] = useState(localStorage.getItem('WORPEN_API_URL') || 'http://127.0.0.1:3000');

  const handleCommit = () => {
    localStorage.setItem('WORPEN_API_URL', apiUrl);
    addNotification('SUCCESS', 'SYSTEM_COMMIT', 'Configuration successfully propagated to fleet.', 3000);
  };

  return (
    <div className="h-full flex flex-col bg-[#1e1f22] text-[#dfe1e5]">
      {/* Header Overlay */}
      <div className="flex justify-between items-center px-6 py-4 border-b border-[#43454a] bg-[#2b2d30] shadow-sm">
        <div>
          <h1 className="text-xl font-bold flex items-center gap-3">
            <div className="p-2 rounded-[var(--radius)] bg-[#3574f015] border border-[#3574f020]">
              <Settings className="text-[#3574f0]" size={20} />
            </div>
            Settings
          </h1>
          <p className="text-[11px] text-[#6e7073] mt-1 font-medium tracking-wide uppercase">Core System & Global Configuration</p>
        </div>
        <button
          onClick={handleCommit}
          className="px-5 py-2 bg-[#3574f0] text-white text-[12px] font-bold rounded-[var(--radius)] hover:bg-[#3574f0e0] flex items-center gap-2 shadow-lg transition-all active:scale-95"
        >
          <Save size={16} /> SAVE SETTINGS
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-6 flex flex-col gap-6 custom-scrollbar pb-12">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">

          {/* Connection & Security */}
          <div className="space-y-6">
            <section className="jb-card p-5 space-y-5">
              <div className="flex items-center gap-2 pb-3 border-b border-[#43454a]">
                <Globe size={16} className="text-[#3574f0]" />
                <h2 className="text-[13px] font-bold text-[#dfe1e5]">System Uplink</h2>
              </div>

              <div className="grid gap-5">
                <div className="space-y-2">
                  <label className="text-[11px] font-bold text-[#6e7073] uppercase px-1">API Endpoint</label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={apiUrl}
                      onChange={(e) => setApiUrl(e.target.value)}
                      placeholder="http://127.0.0.1:3000"
                      className="flex-1 bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none transition-colors shadow-inner"
                    />
                    <button
                      onClick={() => {
                        fetch(`${apiUrl}/health`).then(() => addNotification('SUCCESS', 'PONG', 'Backend is online')).catch(() => addNotification('ERROR', 'OFFLINE', 'Backend unreachable'));
                      }}
                      className="px-4 bg-[#2b2d30] border border-[#43454a] text-[#dfe1e5] text-[11px] font-bold rounded-[var(--radius)] hover:bg-[#393b40] hover:border-[#6a6e75] transition-all"
                    >
                      TEST
                    </button>
                  </div>
                </div>

                <div className="space-y-2">
                  <label className="text-[11px] font-bold text-[#6e7073] uppercase px-1">Master API Key</label>
                  <div className="relative group">
                    <input
                      type="password"
                      defaultValue="sk_live_99823482394823984"
                      className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none transition-colors"
                    />
                    <div className="absolute right-3 top-1/2 -translate-y-1/2 text-[#6e7073]">
                      <Key size={14} />
                    </div>
                  </div>
                </div>

                <div className="space-y-2 pt-2">
                  <label className="text-[11px] font-bold text-[#6e7073] uppercase px-1">Security Certificate</label>
                  <div className="border-2 border-[#43454a] border-dashed bg-[#1e1f22]/50 p-6 rounded-[var(--radius)] text-center cursor-pointer hover:border-[#3574f040] hover:bg-[#3574f005] transition-all group">
                    <Shield size={24} className="mx-auto text-[#6e7073] group-hover:text-[#3574f060] transition-colors mb-2" />
                    <div className="text-[12px] font-medium text-[#6e7073] group-hover:text-[#dfe1e5]">Drop mTLS Certificate here</div>
                    <div className="text-[10px] text-[#6e7073] mt-1 opacity-60">PFX, PEM, or CRT files supported</div>
                  </div>
                </div>
              </div>
            </section>

            <section className="jb-card p-5 space-y-5">
              <div className="flex items-center gap-2 pb-3 border-b border-[#43454a]">
                <Bell size={16} className="text-[#59a869]" />
                <h2 className="text-[13px] font-bold text-[#dfe1e5]">Notifications</h2>
              </div>

              <div className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-bold text-[#6e7073] uppercase">Slack Webhook</span>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input type="checkbox" className="sr-only peer" defaultChecked />
                      <div className="w-8 h-4 bg-[#43454a] rounded-full peer peer-checked:bg-[#59a869] transition-colors relative">
                        <div className="absolute left-0.5 top-0.5 w-3 h-3 bg-white rounded-full transition-transform peer-checked:translate-x-4"></div>
                      </div>
                    </label>
                  </div>
                  <input
                    type="text"
                    placeholder="https://hooks.slack.com/services/..."
                    className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none"
                  />
                </div>

                <div className="space-y-2 opacity-50">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-bold text-[#6e7073] uppercase">Discord Webhook</span>
                    <label className="relative inline-flex items-center cursor-not-allowed">
                      <div className="w-8 h-4 bg-gray-700/50 rounded-full relative">
                        <div className="absolute left-0.5 top-0.5 w-3 h-3 bg-gray-600 rounded-full"></div>
                      </div>
                    </label>
                  </div>
                  <input
                    type="text"
                    disabled
                    placeholder="Discord integration coming soon..."
                    className="w-full bg-[#1e1f22]/50 border border-[#43454a] text-[#6e7073] px-3 py-2 rounded-[var(--radius)] text-sm font-mono cursor-not-allowed"
                  />
                </div>
              </div>
            </section>
          </div>

          {/* Environment Variables */}
          <div className="h-full">
            <section className="jb-card p-5 h-full flex flex-col space-y-4">
              <div className="flex items-center gap-2 pb-3 border-b border-[#43454a]">
                <Hash size={16} className="text-[#a682e6]" />
                <h2 className="text-[13px] font-bold text-[#dfe1e5]">Environment Variables</h2>
              </div>

              <div className="bg-[#a682e610] text-[#a682e6] text-[11px] px-3 py-2 border-l-4 border-[#a682e6] rounded-[2px] font-medium leading-relaxed">
                Variables defined here are globally injected into all connected agents during the next heartbeat cycle.
              </div>

              <div className="flex-1 overflow-hidden border border-[#43454a] rounded-[var(--radius)] bg-[#1e1f22]">
                <table className="w-full text-[12px] text-left border-collapse">
                  <thead className="bg-[#2b2d30] text-[#6e7073] sticky top-0 border-b border-[#43454a] font-bold uppercase tracking-wider text-[10px]">
                    <tr>
                      <th className="p-3 border-r border-[#43454a]">Key</th>
                      <th className="p-3 border-r border-[#43454a]">Value</th>
                      <th className="p-3 w-10"></th>
                    </tr>
                  </thead>
                  <tbody>
                    {envVars.map((env, idx) => (
                      <tr key={idx} className="border-b border-[#43454a] font-mono hover:bg-[#3574f005] transition-colors group">
                        <td className="p-3 text-[#dfe1e5] font-semibold border-r border-[#43454a]">{env.key}</td>
                        <td className="p-3 text-[#6e7073] group-hover:text-[#dfe1e5] transition-colors border-r border-[#43454a]">{env.value}</td>
                        <td className="p-3 text-center">
                          <button onClick={() => removeEnv(idx)} className="text-[#6e7073] hover:text-[#e06c75] transition-colors">
                            <X size={14} />
                          </button>
                        </td>
                      </tr>
                    ))}
                    {envVars.length === 0 && (
                      <tr>
                        <td colSpan={3} className="p-8 text-center text-[#6e7073] italic">No environment variables defined.</td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>

              <div className="pt-4 border-t border-[#43454a] flex gap-3">
                <div className="flex-[2] space-y-1.5">
                  <label className="text-[10px] text-[#6e7073] font-bold uppercase ml-1">Key Name</label>
                  <input
                    value={newKey}
                    onChange={(e) => setNewKey(e.target.value.toUpperCase().replace(/\s/g, '_'))}
                    className="w-full bg-[#1e1f22] border border-[#43454a] p-2 rounded-[var(--radius)] text-xs outline-none focus:border-[#3574f0] text-[#dfe1e5] font-mono"
                    placeholder="E.G. API_TOKEN"
                  />
                </div>
                <div className="flex-[3] space-y-1.5">
                  <label className="text-[10px] text-[#6e7073] font-bold uppercase ml-1">Variable Value</label>
                  <input
                    value={newValue}
                    onChange={(e) => setNewValue(e.target.value)}
                    className="w-full bg-[#1e1f22] border border-[#43454a] p-2 rounded-[var(--radius)] text-xs outline-none focus:border-[#3574f0] text-[#dfe1e5] font-mono"
                    placeholder="Value..."
                    onKeyDown={(e) => e.key === 'Enter' && addEnv()}
                  />
                </div>
                <button
                  onClick={addEnv}
                  className="bg-[#2b2d30] hover:bg-[#3574f0] text-[#dfe1e5] border border-[#43454a] rounded-[var(--radius)] h-8 w-8 mt-auto flex items-center justify-center shadow-md active:scale-90 transition-all"
                >
                  <Plus size={16} />
                </button>
              </div>
            </section>
          </div>
        </div>
      </div>
    </div>
  );
};
