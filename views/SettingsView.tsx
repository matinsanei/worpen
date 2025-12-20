
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

  const handleCommit = () => {
      addNotification('SUCCESS', 'SYSTEM_COMMIT', 'Configuration successfully propagated to fleet.', 3000);
  };

  return (
    <div className="h-full p-2 flex flex-col gap-4">
       {/* Header */}
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2">
        <div>
           <h1 className="text-2xl font-bold flex items-center gap-2">
             <Settings className="text-gray-400" /> 
             SYSTEM_CONFIGURATION
           </h1>
           <p className="text-xs text-green-600">CORE SETTINGS // GLOBAL ENV // SECURITY</p>
        </div>
        <button 
          onClick={handleCommit}
          className="px-4 py-1 bg-green-900 border border-green-500 text-white text-xs hover:bg-green-700 flex items-center gap-2 font-bold animate-pulse transition-all"
        >
           <Save size={14} /> COMMIT_CHANGES
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 overflow-y-auto pb-10">
        
        {/* Left Column: Connection & Security */}
        <div className="flex flex-col gap-4">
          <TerminalFrame title="CORE_UPLINK [VPS_CONNECTION]">
            <div className="flex flex-col gap-4 p-2">
              
              <div className="space-y-1">
                <label className="text-[10px] text-green-400 uppercase font-bold flex items-center gap-2">
                  <Globe size={12}/> Controller Endpoint (gRPC)
                </label>
                <div className="flex gap-2">
                  <input 
                    type="text" 
                    defaultValue="grpc.core.worpen.io:50051" 
                    className="flex-1 bg-black border border-green-800 text-green-300 p-2 text-xs font-mono focus:border-green-500 outline-none"
                  />
                  <button 
                    onClick={() => addNotification('INFO', 'PING_SENT', 'Latency: 14ms')}
                    className="px-3 bg-gray-900 border border-gray-700 text-green-500 text-xs hover:text-white hover:border-white hover:bg-green-900"
                  >
                    PING
                  </button>
                </div>
              </div>

              <div className="space-y-1">
                <label className="text-[10px] text-green-400 uppercase font-bold flex items-center gap-2">
                  <Key size={12}/> Master API Key
                </label>
                <input 
                  type="password" 
                  defaultValue="sk_live_99823482394823984" 
                  className="w-full bg-black border border-green-800 text-green-300 p-2 text-xs font-mono focus:border-green-500 outline-none"
                />
              </div>

               <div className="space-y-1">
                <label className="text-[10px] text-green-400 uppercase font-bold flex items-center gap-2">
                  <Shield size={12}/> mTLS Certificate (Client.pem)
                </label>
                <div className="border border-green-900 border-dashed bg-green-900/10 p-4 text-center cursor-pointer hover:bg-green-900/20 group">
                   <div className="text-xs text-gray-500 group-hover:text-green-400">DRAG CERTIFICATE HERE OR CLICK TO UPLOAD</div>
                   <div className="text-[9px] text-gray-700 mt-1">SECURE ENCLAVE UPLOAD</div>
                </div>
              </div>

            </div>
          </TerminalFrame>

          <TerminalFrame title="ALERT_CHANNELS">
             <div className="flex flex-col gap-4 p-2">
                <div className="flex flex-col gap-2">
                    <div className="flex items-center justify-between border-b border-green-900 pb-1">
                       <span className="text-xs flex items-center gap-2"><Hash size={12}/> SLACK_WEBHOOK</span>
                       <div className="w-8 h-4 bg-green-900 rounded-full relative cursor-pointer"><div className="absolute right-0 top-0 h-4 w-4 bg-green-500 rounded-full shadow-[0_0_5px_#00ff00]"></div></div>
                    </div>
                    <input 
                      type="text" 
                      placeholder="https://hooks.slack.com/services/YOUR/WEBHOOK/URL"
                      className="w-full bg-black border border-green-900 text-gray-500 p-2 text-xs font-mono focus:border-green-500 outline-none"
                    />
                </div>
                
                <div className="flex flex-col gap-2">
                    <div className="flex items-center justify-between border-b border-green-900 pb-1">
                       <span className="text-xs flex items-center gap-2"><Bell size={12}/> DISCORD_WEBHOOK</span>
                       <div className="w-8 h-4 bg-gray-800 rounded-full relative cursor-pointer"><div className="absolute left-0 top-0 h-4 w-4 bg-gray-500 rounded-full"></div></div>
                    </div>
                    <input 
                      type="text" 
                      placeholder="https://discord.com/api/webhooks/..."
                      disabled
                      className="w-full bg-black border border-gray-800 text-gray-700 p-2 text-xs font-mono focus:border-gray-500 outline-none cursor-not-allowed"
                    />
                </div>
             </div>
          </TerminalFrame>
        </div>

        {/* Right Column: Environment Variables */}
        <TerminalFrame title="GLOBAL_ENV_VARS [.ENV INJECTION]">
          <div className="flex flex-col h-full min-h-[400px]">
            <div className="bg-yellow-900/20 text-yellow-500 text-[10px] p-2 mb-2 border-l-2 border-yellow-500">
               WARNING: Variables defined here are injected into all connected Agents during the next heartbeat.
            </div>

            <div className="flex-1 overflow-y-auto mb-4 border border-green-900/50 bg-black/50">
               <table className="w-full text-xs text-left border-collapse">
                 <thead className="bg-green-900/20 text-green-300 sticky top-0">
                   <tr>
                     <th className="p-2 border-b border-green-800">KEY</th>
                     <th className="p-2 border-b border-green-800">VALUE</th>
                     <th className="p-2 border-b border-green-800 w-8"></th>
                   </tr>
                 </thead>
                 <tbody>
                   {envVars.map((env, idx) => (
                     <tr key={idx} className="border-b border-green-900/30 font-mono hover:bg-green-900/10 group">
                       <td className="p-2 text-green-400 font-bold">{env.key}</td>
                       <td className="p-2 text-gray-300">{env.value}</td>
                       <td className="p-2 text-center">
                         <button onClick={() => removeEnv(idx)} className="text-gray-600 hover:text-red-500 transition-colors"><X size={14}/></button>
                       </td>
                     </tr>
                   ))}
                   {envVars.length === 0 && (
                     <tr>
                       <td colSpan={3} className="p-4 text-center text-gray-600 italic">NO VARIABLES DEFINED</td>
                     </tr>
                   )}
                 </tbody>
               </table>
            </div>

            <div className="flex gap-2 items-end pt-3 border-t border-green-900">
               <div className="flex-[2]">
                 <label className="text-[9px] text-gray-500 mb-1 block">KEY (UPPERCASE)</label>
                 <input 
                    value={newKey}
                    onChange={(e) => setNewKey(e.target.value.toUpperCase().replace(/\s/g, '_'))}
                    className="w-full bg-black border border-green-800 p-2 text-xs outline-none focus:border-green-500 text-green-300 placeholder-gray-800" 
                    placeholder="E.G. MAX_THREADS"
                  />
               </div>
               <div className="flex-[3]">
                 <label className="text-[9px] text-gray-500 mb-1 block">VALUE</label>
                 <input 
                    value={newValue}
                    onChange={(e) => setNewValue(e.target.value)}
                    className="w-full bg-black border border-green-800 p-2 text-xs outline-none focus:border-green-500 text-green-300 placeholder-gray-800" 
                    placeholder="Value..."
                    onKeyDown={(e) => e.key === 'Enter' && addEnv()}
                  />
               </div>
               <button 
                onClick={addEnv}
                className="bg-green-900 hover:bg-green-700 text-white p-2 border border-green-600 h-[34px] w-[34px] flex items-center justify-center"
               >
                 <Plus size={16}/>
               </button>
            </div>
          </div>
        </TerminalFrame>

      </div>
    </div>
  );
};
