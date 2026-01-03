import React, { useState, useEffect } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { Save, Shield, Globe, Key, Plus, X, Settings, Bell, Hash, Sparkles, CheckCircle, AlertTriangle, Loader2, Puzzle } from 'lucide-react';
import { useNotifications } from '../components/NotificationSystem';
import { getAIConfig, saveAIConfig, testAIConnection, AIConfig, AVAILABLE_MODELS, POPULAR_ENDPOINTS } from '../services/aiConfig';

export const SettingsView: React.FC = () => {
  const { addNotification } = useNotifications();
  const [envVars, setEnvVars] = useState([
    { key: 'RUST_LOG', value: 'info' },
    { key: 'HIVE_REGION', value: 'eu-west-1' },
    { key: 'HEARTBEAT_INTERVAL', value: '5000' }
  ]);

  const [newKey, setNewKey] = useState('');
  const [newValue, setNewValue] = useState('');
  
  // AI Configuration State
  const [aiConfig, setAiConfig] = useState<AIConfig>(() => getAIConfig());
  const [testingConnection, setTestingConnection] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<{ type: 'success' | 'error' | null; message: string }>({ type: null, message: '' });
  
  // 🚀 Custom Operation Schemas State
  const [customSchemas, setCustomSchemas] = useState<Array<{id: string; name: string; schema: any}>>([]);
  const [newSchemaName, setNewSchemaName] = useState('');
  const [newSchemaContent, setNewSchemaContent] = useState('{\n  "required_field": "string",\n  "optional_field": "number"\n}');
  
  // Load custom schemas from localStorage
  useEffect(() => {
    try {
      const stored = localStorage.getItem('worpen_custom_schemas');
      if (stored) {
        setCustomSchemas(JSON.parse(stored));
      }
    } catch (error) {
      console.error('Failed to load custom schemas:', error);
    }
  }, []);
  
  // Save custom schemas to localStorage
  const saveCustomSchemas = (schemas: typeof customSchemas) => {
    try {
      localStorage.setItem('worpen_custom_schemas', JSON.stringify(schemas));
      setCustomSchemas(schemas);
      addNotification('SUCCESS', 'SCHEMAS_SAVED', 'Custom operation schemas saved!', 3000);
    } catch (error: any) {
      addNotification('ERROR', 'SCHEMAS_ERROR', error.message || 'Failed to save schemas');
    }
  };
  
  const addCustomSchema = () => {
    if (!newSchemaName.trim()) {
      addNotification('ERROR', 'SCHEMA_ERROR', 'Schema name is required');
      return;
    }
    
    try {
      const schemaObj = JSON.parse(newSchemaContent);
      const newSchema = {
        id: `${Date.now()}_${newSchemaName.toLowerCase().replace(/\s+/g, '_')}`,
        name: newSchemaName,
        schema: schemaObj
      };
      
      saveCustomSchemas([...customSchemas, newSchema]);
      setNewSchemaName('');
      setNewSchemaContent('{\n  "required_field": "string",\n  "optional_field": "number"\n}');
    } catch (error: any) {
      addNotification('ERROR', 'SCHEMA_PARSE_ERROR', 'Invalid JSON schema');
    }
  };
  
  const removeCustomSchema = (id: string) => {
    saveCustomSchemas(customSchemas.filter(s => s.id !== id));
    addNotification('INFO', 'SCHEMA_REMOVED', 'Custom schema removed');
  };

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

  const handleSaveAIConfig = () => {
    try {
      saveAIConfig(aiConfig);
      addNotification('SUCCESS', 'AI_CONFIG_SAVED', 'AI configuration saved successfully.', 3000);
      setConnectionStatus({ type: null, message: '' });
    } catch (error: any) {
      addNotification('ERROR', 'AI_CONFIG_ERROR', error.message || 'Failed to save AI configuration');
    }
  };

  const handleTestConnection = async () => {
    setTestingConnection(true);
    setConnectionStatus({ type: null, message: '' });

    try {
      const result = await testAIConnection(aiConfig);
      setConnectionStatus({
        type: result.success ? 'success' : 'error',
        message: result.message,
      });

      if (result.success) {
        addNotification('SUCCESS', 'AI_CONNECTED', 'AI connection verified!', 3000);
      } else {
        addNotification('ERROR', 'AI_CONNECTION_FAILED', result.message);
      }
    } catch (error: any) {
      setConnectionStatus({
        type: 'error',
        message: error.message || 'Connection test failed',
      });
    } finally {
      setTestingConnection(false);
    }
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

        {/* AI Integration Section */}
        <div className="w-full">
          <section className="jb-card p-6 space-y-5">
            <div className="flex items-center justify-between pb-4 border-b border-[#43454a]">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-[#3574f0] to-[#c678dd] flex items-center justify-center shadow-lg">
                  <Sparkles size={20} className="text-white" />
                </div>
                <div>
                  <h2 className="text-base font-bold text-[#dfe1e5]">AI Integration</h2>
                  <p className="text-xs text-[#6e7073] mt-0.5">
                    Configure AI provider for Logic Generator
                  </p>
                </div>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input 
                  type="checkbox" 
                  className="sr-only peer"
                  checked={aiConfig.enabled}
                  onChange={(e) => setAiConfig({ ...aiConfig, enabled: e.target.checked })}
                />
                <div className="w-11 h-6 bg-[#43454a] rounded-full peer peer-checked:bg-[#59a869] transition-colors relative">
                  <div className="absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition-transform peer-checked:translate-x-5"></div>
                </div>
                <span className="ml-3 text-sm font-medium text-[#dfe1e5]">
                  {aiConfig.enabled ? 'Enabled' : 'Disabled'}
                </span>
              </label>
            </div>

            <div className="bg-[#3574f010] text-[#3574f0] text-xs px-4 py-3 border-l-4 border-[#3574f0] rounded-[2px] font-medium leading-relaxed">
              <strong>GitHub Models (Free):</strong> Get your free API key from{' '}
              <a 
                href="https://github.com/marketplace/models" 
                target="_blank" 
                rel="noopener noreferrer"
                className="underline hover:text-[#4a8fff] transition-colors"
              >
                github.com/marketplace/models
              </a>
              . No credit card required!
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
              {/* API Endpoint */}
              <div className="space-y-2">
                <label className="text-xs font-bold text-[#6e7073] uppercase px-1">
                  API Endpoint
                </label>
                <select
                  value={aiConfig.endpoint}
                  onChange={(e) => setAiConfig({ ...aiConfig, endpoint: e.target.value })}
                  className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2.5 rounded-[var(--radius)] text-sm focus:border-[#3574f0] outline-none transition-colors"
                >
                  {POPULAR_ENDPOINTS.map((ep) => (
                    <option key={ep.url} value={ep.url}>
                      {ep.name} - {ep.url}
                    </option>
                  ))}
                </select>
                <input
                  type="text"
                  value={aiConfig.endpoint}
                  onChange={(e) => setAiConfig({ ...aiConfig, endpoint: e.target.value })}
                  placeholder="https://models.inference.ai.azure.com"
                  className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none transition-colors mt-2"
                />
              </div>

              {/* Model Name */}
              <div className="space-y-2">
                <label className="text-xs font-bold text-[#6e7073] uppercase px-1">
                  Model Name
                </label>
                <select
                  value={aiConfig.model}
                  onChange={(e) => setAiConfig({ ...aiConfig, model: e.target.value })}
                  className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2.5 rounded-[var(--radius)] text-sm focus:border-[#3574f0] outline-none transition-colors"
                >
                  {AVAILABLE_MODELS.map((model) => (
                    <option key={model.id} value={model.id}>
                      {model.name} ({model.provider})
                    </option>
                  ))}
                </select>
                <input
                  type="text"
                  value={aiConfig.model}
                  onChange={(e) => setAiConfig({ ...aiConfig, model: e.target.value })}
                  placeholder="gpt-4o"
                  className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none transition-colors mt-2"
                />
              </div>
            </div>

            {/* API Key */}
            <div className="space-y-2">
              <label className="text-xs font-bold text-[#6e7073] uppercase px-1">
                API Key
              </label>
              <div className="relative group">
                <input
                  type="password"
                  value={aiConfig.apiKey}
                  onChange={(e) => setAiConfig({ ...aiConfig, apiKey: e.target.value })}
                  placeholder="Enter your API key..."
                  className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2.5 pr-10 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none transition-colors"
                />
                <div className="absolute right-3 top-1/2 -translate-y-1/2 text-[#6e7073]">
                  <Key size={16} />
                </div>
              </div>
              <p className="text-[10px] text-[#6e7073] px-1">
                Your API key is stored locally in your browser and never sent to Worpen servers.
              </p>
            </div>

            {/* Connection Status */}
            {connectionStatus.type && (
              <div className={`flex items-start gap-3 p-4 rounded-[var(--radius)] border ${
                connectionStatus.type === 'success'
                  ? 'bg-[#59a869]/10 border-[#59a869]/30 text-[#59a869]'
                  : 'bg-[#e06c75]/10 border-[#e06c75]/30 text-[#e06c75]'
              }`}>
                {connectionStatus.type === 'success' ? (
                  <CheckCircle size={20} className="flex-shrink-0 mt-0.5" />
                ) : (
                  <AlertTriangle size={20} className="flex-shrink-0 mt-0.5" />
                )}
                <div className="flex-1">
                  <p className="text-sm font-semibold">
                    {connectionStatus.type === 'success' ? 'Connection Successful' : 'Connection Failed'}
                  </p>
                  <p className="text-xs mt-1 opacity-90">
                    {connectionStatus.message}
                  </p>
                </div>
                <button
                  onClick={() => setConnectionStatus({ type: null, message: '' })}
                  className="text-current hover:opacity-70 transition-opacity"
                >
                  <X size={16} />
                </button>
              </div>
            )}

            {/* Action Buttons */}
            <div className="flex items-center gap-3 pt-4 border-t border-[#43454a]">
              <button
                onClick={handleTestConnection}
                disabled={testingConnection || !aiConfig.apiKey || !aiConfig.endpoint}
                className="flex items-center gap-2 px-5 py-2.5 bg-[#2b2d30] hover:bg-[#393b40] disabled:bg-[#2b2d30] disabled:opacity-50 disabled:cursor-not-allowed text-[#dfe1e5] text-sm font-medium border border-[#43454a] rounded-[var(--radius)] transition-all active:scale-95"
              >
                {testingConnection ? (
                  <>
                    <Loader2 size={16} className="animate-spin" />
                    Testing Connection...
                  </>
                ) : (
                  <>
                    <Sparkles size={16} />
                    Test Connection
                  </>
                )}
              </button>
              <button
                onClick={handleSaveAIConfig}
                className="flex items-center gap-2 px-5 py-2.5 bg-[#3574f0] hover:bg-[#2d5fc7] text-white text-sm font-bold rounded-[var(--radius)] transition-all active:scale-95 shadow-lg"
              >
                <Save size={16} />
                Save AI Config
              </button>
            </div>
          </section>
          
          {/* 🚀 Custom Operation Schemas Section */}
          <section className="jb-card p-6 space-y-5">
            <div className="flex items-center gap-2 pb-3 border-b border-[#43454a]">
              <Puzzle size={16} className="text-[#c678dd]" />
              <h2 className="text-[13px] font-bold text-[#dfe1e5]">🧩 Extensions / Custom Operations</h2>
            </div>

            <div className="space-y-4">
              <p className="text-[11px] text-[#6e7073]">
                Define custom operation types that will be automatically recognized by the compiler and Monaco editor.
                Variable scoping will work automatically for any fields containing <code className="px-1 py-0.5 bg-[#1e1f22] rounded text-[#e06c75]">{`{{...}}`}</code> patterns.
              </p>

              {/* Existing Schemas List */}
              {customSchemas.length > 0 && (
                <div className="space-y-2">
                  <h3 className="text-[11px] font-bold text-[#6e7073] uppercase">Registered Custom Operations</h3>
                  <div className="space-y-2">
                    {customSchemas.map((schema) => (
                      <div key={schema.id} className="flex items-center justify-between p-3 bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)]">
                        <div className="flex-1">
                          <div className="text-sm font-mono text-[#dfe1e5]">{schema.name}</div>
                          <div className="text-[10px] text-[#6e7073] font-mono mt-1">
                            {JSON.stringify(schema.schema).substring(0, 80)}...
                          </div>
                        </div>
                        <button
                          onClick={() => removeCustomSchema(schema.id)}
                          className="ml-3 p-1.5 bg-[#e06c75]/20 hover:bg-[#e06c75]/30 border border-[#e06c75]/30 text-[#e06c75] rounded-[var(--radius)] transition-all"
                        >
                          <X size={14} />
                        </button>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Add New Schema Form */}
              <div className="space-y-3 pt-2 border-t border-[#43454a]">
                <h3 className="text-[11px] font-bold text-[#6e7073] uppercase">Add New Operation Type</h3>
                
                <div className="space-y-2">
                  <label className="text-[11px] font-bold text-[#6e7073] uppercase px-1">Operation Name</label>
                  <input
                    type="text"
                    value={newSchemaName}
                    onChange={(e) => setNewSchemaName(e.target.value)}
                    placeholder="e.g., EmailOp, NuclearLaunchOp"
                    className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none"
                  />
                </div>

                <div className="space-y-2">
                  <label className="text-[11px] font-bold text-[#6e7073] uppercase px-1">JSON Schema</label>
                  <textarea
                    value={newSchemaContent}
                    onChange={(e) => setNewSchemaContent(e.target.value)}
                    rows={8}
                    className="w-full bg-[#1e1f22] border border-[#43454a] text-[#dfe1e5] px-3 py-2 rounded-[var(--radius)] text-sm font-mono focus:border-[#3574f0] outline-none resize-none"
                    placeholder='{ "to": "string", "subject": "string", "body": "string" }'
                  />
                </div>

                <button
                  onClick={addCustomSchema}
                  className="flex items-center gap-2 px-4 py-2 bg-[#c678dd]/20 hover:bg-[#c678dd]/30 border border-[#c678dd]/30 text-[#c678dd] text-sm font-bold rounded-[var(--radius)] transition-all active:scale-95"
                >
                  <Plus size={16} />
                  Register Operation
                </button>
              </div>

              <div className="mt-4 p-3 bg-[#61afef]/10 border border-[#61afef]/20 rounded-[var(--radius)] text-[11px] text-[#61afef]">
                <strong>💡 How it works:</strong> Once registered, your custom operations will appear in Monaco autocomplete,
                and the compiler will automatically scan/scope all string fields containing <code className="px-1 bg-[#1e1f22]/50">{`{{variables}}`}</code>.
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
};
