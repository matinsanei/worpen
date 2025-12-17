import React, { useState, useEffect } from 'react';
import { 
    Code, Play, Plus, Trash2, Download, Upload, CheckCircle, XCircle, 
    Zap, FileJson, Activity, Terminal, Rocket, Copy, RefreshCw, Globe, Clock
} from 'lucide-react';

const API_BASE_URL = 'http://127.0.0.1:3000';

const ROUTE_TEMPLATES = {
  simple: {
    name: "Simple Echo",
    icon: Terminal,
    json: JSON.stringify({
      name: "Echo API",
      description: "Returns your input",
      path: "/api/custom/echo",
      method: "POST",
      logic: [{
        "return": { "value": { "message": "{{input}}" } }
      }],
      parameters: [],
      enabled: true,
      version: "1.0.0",
      auth_required: false
    }, null, 2)
  },
  database: {
    name: "Database Query",
    icon: Activity,
    json: JSON.stringify({
      name: "Users API",
      description: "Get users from database",
      path: "/api/custom/users",
      method: "GET",
      logic: [
        { "query_db": { "query": "SELECT * FROM agents LIMIT 10", "params": [] } },
        { "return": { "value": { "users": "{{db_result}}" } } }
      ],
      parameters: [],
      enabled: true,
      version: "1.0.0",
      auth_required: false
    }, null, 2)
  },
  conditional: {
    name: "Conditional Logic",
    icon: Code,
    json: JSON.stringify({
      name: "Age Check API",
      description: "Check if user is adult",
      path: "/api/custom/age-check",
      method: "POST",
      logic: [{
        "if": {
          "condition": "age >= 18",
          "then": [{ "return": { "value": { "status": "adult", "can_vote": true } } }],
          "otherwise": [{ "return": { "value": { "status": "minor", "can_vote": false } } }]
        }
      }],
      parameters: [{ "name": "age", "param_type": "body", "data_type": "number", "required": true }],
      enabled: true,
      version: "1.0.0",
      auth_required: false
    }, null, 2)
  }
};

const StatCard = ({ icon: Icon, value, label, color = "green" }: any) => (
    <div className="relative overflow-hidden rounded-md bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 p-4 group hover:border-green-500/30 transition-all hover:shadow-[0_0_20px_rgba(0,255,65,0.05)]">
        <div className="flex items-center gap-3 relative z-10">
            <div className={`p-2 rounded-lg bg-${color}-500/10 text-${color}-500 group-hover:bg-${color}-500/20 transition-colors`}>
                <Icon size={18} />
            </div>
            <div>
                <div className="text-2xl font-bold text-white tracking-tight">{value}</div>
                <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">{label}</div>
            </div>
        </div>
        <div className={`absolute -right-4 -bottom-4 w-20 h-20 bg-${color}-500/10 rounded-full blur-[40px] group-hover:bg-${color}-500/20 transition-colors`}></div>
    </div>
);

export const DynamicRoutesView: React.FC = () => {
    const [routes, setRoutes] = useState<any[]>([]);
    const [selectedRoute, setSelectedRoute] = useState<any>(null);
    const [routeDefinition, setRouteDefinition] = useState('');
    const [testPayload, setTestPayload] = useState('{}');
    const [testResult, setTestResult] = useState<any>(null);
    const [loading, setLoading] = useState(false);
    const [activeTab, setActiveTab] = useState<'editor' | 'test' | 'preview'>('editor');

    useEffect(() => {
        fetchRoutes();
        const interval = setInterval(fetchRoutes, 5000);
        return () => clearInterval(interval);
    }, []);

    const fetchRoutes = async () => {
        try {
            const response = await fetch(`${API_BASE_URL}/api/v1/dynamic-routes`);
            const data = await response.json();
            setRoutes(data);
        } catch (error) {
            console.error('Failed to fetch routes:', error);
        }
    };

    const registerRoute = async () => {
        if (!routeDefinition.trim()) return;

        setLoading(true);
        try {
            const response = await fetch(`${API_BASE_URL}/api/v1/dynamic-routes`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: routeDefinition,
            });
            
            if (response.ok) {
                await fetchRoutes();
                alert('✅ Route registered successfully!');
                setRouteDefinition('');
            } else {
                const error = await response.text();
                alert('❌ Failed to register route: ' + error);
            }
        } catch (error: any) {
            alert('❌ Error: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const deleteRoute = async (routeId: string) => {
        if (!confirm('Delete this route?')) return;

        try {
            const response = await fetch(`${API_BASE_URL}/api/v1/dynamic-routes/${routeId}`, {
                method: 'DELETE',
            });
            
            if (response.ok) {
                await fetchRoutes();
                if (selectedRoute?.id === routeId) {
                    setSelectedRoute(null);
                }
            }
        } catch (error) {
            console.error('Failed to delete route:', error);
        }
    };

    const testRoute = async () => {
        if (!selectedRoute) return;

        setLoading(true);
        try {
            const response = await fetch(`${API_BASE_URL}/api/v1/dynamic-routes/test`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    route_id: selectedRoute.id,
                    test_payload: JSON.parse(testPayload || '{}'),
                    test_params: {}
                }),
            });
            
            const data = await response.json();
            setTestResult(data);
        } catch (error: any) {
            setTestResult({ success: false, error: error.message });
        } finally {
            setLoading(false);
        }
    };

    const loadTemplate = (key: string) => {
        const template = ROUTE_TEMPLATES[key as keyof typeof ROUTE_TEMPLATES];
        setRouteDefinition(template.json);
        setActiveTab('editor');
    };

    return (
        <div className="dynamic-routes-container">
            {/* Hero Header */}
            <div className="dr-hero">
                <div className="dr-hero-badge">
                    <Sparkles size={14} />
                    <span>Revolutionary JSON-Driven API Engine</span>
                </div>
                <h1 className="dr-hero-title">
                    <Rocket size={36} />
                    Dynamic Routes
                </h1>
                <p className="dr-hero-subtitle">
                    Build powerful APIs with pure JSON - No code deployment needed
                </p>
                <div className="dr-stats">
                    <div className="dr-stat">
                        <Activity size={18} />
                        <div>
                            <div className="dr-stat-value">{routes.length}</div>
                            <div className="dr-stat-label">Active Routes</div>
                        </div>
                    </div>
                    <div className="dr-stat">
                        <Zap size={18} />
                        <div>
                            <div className="dr-stat-value">0ms</div>
                            <div className="dr-stat-label">Execution</div>
                        </div>
                    </div>
                    <div className="dr-stat">
                        <Terminal size={18} />
                        <div>
                            <div className="dr-stat-value">11</div>
                            <div className="dr-stat-label">Logic Ops</div>
                        </div>
                    </div>
                </div>
            </div>

            {/* Main Content */}
            <div className="dr-content">
                {/* Sidebar */}
                <div className="dr-sidebar">
                    <div className="dr-sidebar-section">
                        <h3><FileJson size={18} /> Quick Start</h3>
                        <div className="dr-templates">
                            {Object.entries(ROUTE_TEMPLATES).map(([key, template]) => (
                                <div key={key} className="dr-template-card" onClick={() => loadTemplate(key)}>
                                    <Code size={20} />
                                    <span>{template.name}</span>
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className="dr-sidebar-section">
                        <h3>
                            <span>Your Routes</span>
                            <span className="dr-badge">{routes.length}</span>
                        </h3>
                        <div className="dr-routes-list">
                            {routes.map((route) => (
                                <div
                                    key={route.id}
                                    className={`dr-route-item ${selectedRoute?.id === route.id ? 'active' : ''}`}
                                    onClick={() => setSelectedRoute(route)}
                                >
                                    <div className="dr-route-method">{route.method}</div>
                                    <div className="dr-route-info">
                                        <div className="dr-route-name">{route.name}</div>
                                        <div className="dr-route-path">{route.path}</div>
                                    </div>
                                    <button
                                        className="dr-delete-btn"
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            deleteRoute(route.id);
                                        }}
                                    >
                                        <Trash2 size={14} />
                                    </button>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* Main Editor */}
                <div className="dr-main">
                    <div className="dr-tabs">
                        <button
                            className={`dr-tab ${activeTab === 'editor' ? 'active' : ''}`}
                            onClick={() => setActiveTab('editor')}
                        >
                            <Code size={16} />
                            <span>Editor</span>
                        </button>
                        <button
                            className={`dr-tab ${activeTab === 'test' ? 'active' : ''}`}
                            onClick={() => setActiveTab('test')}
                        >
                            <Play size={16} />
                            <span>Test</span>
                        </button>
                        <button
                            className={`dr-tab ${activeTab === 'preview' ? 'active' : ''}`}
                            onClick={() => setActiveTab('preview')}
                        >
                            <Eye size={16} />
                            <span>Preview</span>
                        </button>
                    </div>

                    {activeTab === 'editor' && (
                        <div className="dr-editor-panel">
                            <div className="dr-editor-header">
                                <span><Terminal size={18} /> JSON Editor</span>
                                <button className="dr-icon-btn" onClick={() => setRouteDefinition('')}>
                                    <RefreshCw size={14} />
                                </button>
                            </div>
                            <textarea
                                className="dr-code-editor"
                                value={routeDefinition}
                                onChange={(e) => setRouteDefinition(e.target.value)}
                                placeholder='{\n  "name": "My API",\n  "path": "/api/custom/endpoint",\n  "method": "POST",\n  "logic": [{"return": {"value": {"message": "Hello"}}}]\n}'
                                spellCheck={false}
                            />
                            <div className="dr-editor-footer">
                                <button className="dr-btn-primary" onClick={registerRoute} disabled={loading}>
                                    {loading ? <RefreshCw size={16} className="spin" /> : <Plus size={16} />}
                                    <span>{loading ? 'Registering...' : 'Register Route'}</span>
                                </button>
                            </div>
                        </div>
                    )}

                    {activeTab === 'test' && (
                        <div className="dr-test-panel">
                            <div className="dr-test-input">
                                <label>Test Payload (JSON)</label>
                                <textarea
                                    value={testPayload}
                                    onChange={(e) => setTestPayload(e.target.value)}
                                    placeholder='{"param": "value"}'
                                />
                            </div>
                            <button className="dr-btn-test" onClick={testRoute} disabled={loading || !selectedRoute}>
                                {loading ? <RefreshCw size={16} className="spin" /> : <Play size={16} />}
                                <span>{loading ? 'Testing...' : 'Run Test'}</span>
                            </button>

                            {testResult && (
                                <div className="dr-test-result">
                                    <div className={`dr-result-badge ${testResult.success ? 'success' : 'error'}`}>
                                        {testResult.success ? <CheckCircle size={18} /> : <XCircle size={18} />}
                                        <span>{testResult.success ? 'Success' : 'Failed'}</span>
                                    </div>
                                    <div className="dr-result-metrics">
                                        <div className="dr-metric">
                                            <Zap size={14} />
                                            <span>{testResult.execution_time_ms}ms</span>
                                        </div>
                                        <div className="dr-metric">
                                            <Activity size={14} />
                                            <span>{testResult.steps_executed?.length || 0} steps</span>
                                        </div>
                                    </div>
                                    {testResult.result && (
                                        <div className="dr-result-data">
                                            <h4>Response</h4>
                                            <pre>{JSON.stringify(testResult.result, null, 2)}</pre>
                                        </div>
                                    )}
                                    {testResult.steps_executed && (
                                        <div className="dr-steps">
                                            <h4>Execution Steps</h4>
                                            {testResult.steps_executed.map((step: string, idx: number) => (
                                                <div key={idx} className="dr-step">
                                                    <span className="dr-step-num">{idx + 1}</span>
                                                    <span>{step}</span>
                                                </div>
                                            ))}
                                        </div>
                                    )}
                                </div>
                            )}
                        </div>
                    )}

                    {activeTab === 'preview' && selectedRoute && (
                        <div className="dr-preview-panel">
                            <div className="dr-preview-section">
                                <h4>Basic Info</h4>
                                <div className="dr-info-grid">
                                    <div><label>Name</label><div>{selectedRoute.name}</div></div>
                                    <div><label>Method</label><span className="dr-method-badge">{selectedRoute.method}</span></div>
                                    <div><label>Path</label><code>{selectedRoute.path}</code></div>
                                    <div><label>Status</label><span className={selectedRoute.enabled ? 'status-enabled' : 'status-disabled'}>{selectedRoute.enabled ? 'Enabled' : 'Disabled'}</span></div>
                                </div>
                            </div>
                            <div className="dr-preview-section">
                                <h4>Logic Operations</h4>
                                <pre>{JSON.stringify(selectedRoute.logic, null, 2)}</pre>
                            </div>
                        </div>
                    )}
                </div>
            </div>

            <style>{`
                .dynamic-routes-container {
                    height: 100vh;
                    background: linear-gradient(135deg, #0f0f23 0%, #1a1a2e 100%);
                    color: #fff;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                }

                .dr-hero {
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    padding: 40px;
                    text-align: center;
                }

                .dr-hero-badge {
                    display: inline-flex;
                    align-items: center;
                    gap: 6px;
                    background: rgba(255,255,255,0.2);
                    padding: 6px 16px;
                    border-radius: 20px;
                    font-size: 12px;
                    margin-bottom: 16px;
                }

                .dr-hero-title {
                    font-size: 36px;
                    margin: 0 0 12px 0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    gap: 12px;
                }

                .dr-hero-subtitle {
                    font-size: 16px;
                    opacity: 0.9;
                    margin-bottom: 24px;
                }

                .dr-stats {
                    display: flex;
                    gap: 20px;
                    justify-content: center;
                }

                .dr-stat {
                    background: rgba(255,255,255,0.1);
                    padding: 16px 24px;
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                }

                .dr-stat-value {
                    font-size: 24px;
                    font-weight: 700;
                }

                .dr-stat-label {
                    font-size: 11px;
                    opacity: 0.8;
                    text-transform: uppercase;
                }

                .dr-content {
                    flex: 1;
                    display: grid;
                    grid-template-columns: 280px 1fr;
                    overflow: hidden;
                }

                .dr-sidebar {
                    background: #16213e;
                    border-right: 1px solid #2a3f5f;
                    overflow-y: auto;
                }

                .dr-sidebar-section {
                    padding: 20px;
                    border-bottom: 1px solid #2a3f5f;
                }

                .dr-sidebar-section h3 {
                    margin: 0 0 16px 0;
                    font-size: 14px;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    justify-content: space-between;
                }

                .dr-badge {
                    background: #667eea;
                    padding: 2px 8px;
                    border-radius: 10px;
                    font-size: 11px;
                }

                .dr-templates {
                    display: flex;
                    flex-direction: column;
                    gap: 8px;
                }

                .dr-template-card {
                    background: linear-gradient(135deg, #667eea, #764ba2);
                    padding: 12px;
                    border-radius: 8px;
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    cursor: pointer;
                    transition: transform 0.2s;
                }

                .dr-template-card:hover {
                    transform: translateX(4px);
                }

                .dr-routes-list {
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                }

                .dr-route-item {
                    padding: 12px;
                    border-radius: 8px;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    transition: background 0.2s;
                }

                .dr-route-item:hover {
                    background: #1a2940;
                }

                .dr-route-item.active {
                    background: #667eea;
                }

                .dr-route-method {
                    background: #764ba2;
                    padding: 4px 8px;
                    border-radius: 4px;
                    font-size: 10px;
                    font-weight: 700;
                    min-width: 45px;
                    text-align: center;
                }

                .dr-route-info {
                    flex: 1;
                    min-width: 0;
                }

                .dr-route-name {
                    font-size: 13px;
                    font-weight: 600;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }

                .dr-route-path {
                    font-size: 11px;
                    opacity: 0.6;
                    font-family: monospace;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }

                .dr-delete-btn {
                    background: transparent;
                    border: none;
                    color: #ff4757;
                    cursor: pointer;
                    padding: 6px;
                    border-radius: 4px;
                    opacity: 0;
                }

                .dr-route-item:hover .dr-delete-btn {
                    opacity: 1;
                }

                .dr-main {
                    display: flex;
                    flex-direction: column;
                    overflow: hidden;
                }

                .dr-tabs {
                    display: flex;
                    background: #16213e;
                    border-bottom: 1px solid #2a3f5f;
                }

                .dr-tab {
                    padding: 14px 20px;
                    background: transparent;
                    border: none;
                    color: #8899a6;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    font-size: 13px;
                    transition: all 0.2s;
                    border-bottom: 2px solid transparent;
                }

                .dr-tab:hover {
                    color: #fff;
                }

                .dr-tab.active {
                    color: #fff;
                    border-bottom-color: #667eea;
                }

                .dr-editor-panel, .dr-test-panel, .dr-preview-panel {
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    overflow: hidden;
                }

                .dr-editor-header {
                    padding: 16px 20px;
                    border-bottom: 1px solid #2a3f5f;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                }

                .dr-icon-btn {
                    background: transparent;
                    border: 1px solid #2a3f5f;
                    color: #8899a6;
                    padding: 6px;
                    border-radius: 6px;
                    cursor: pointer;
                    transition: all 0.2s;
                }

                .dr-icon-btn:hover {
                    background: #667eea;
                    border-color: #667eea;
                    color: white;
                }

                .dr-code-editor {
                    flex: 1;
                    width: 100%;
                    background: #0a0e14;
                    color: #e6e1dc;
                    border: none;
                    padding: 20px;
                    font-family: 'Fira Code', monospace;
                    font-size: 13px;
                    line-height: 1.6;
                    resize: none;
                    outline: none;
                }

                .dr-editor-footer {
                    padding: 16px 20px;
                    border-top: 1px solid #2a3f5f;
                }

                .dr-btn-primary, .dr-btn-test {
                    background: linear-gradient(135deg, #667eea, #764ba2);
                    color: white;
                    border: none;
                    padding: 12px 24px;
                    border-radius: 8px;
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: 600;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    transition: transform 0.2s;
                }

                .dr-btn-primary:hover, .dr-btn-test:hover {
                    transform: translateY(-2px);
                }

                .dr-btn-primary:disabled, .dr-btn-test:disabled {
                    opacity: 0.5;
                    cursor: not-allowed;
                }

                .spin {
                    animation: spin 1s linear infinite;
                }

                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }

                .dr-test-panel {
                    padding: 20px;
                    overflow-y: auto;
                }

                .dr-test-input {
                    margin-bottom: 16px;
                }

                .dr-test-input label {
                    display: block;
                    margin-bottom: 8px;
                    font-size: 13px;
                    font-weight: 600;
                }

                .dr-test-input textarea {
                    width: 100%;
                    background: #0a0e14;
                    color: #e6e1dc;
                    border: 1px solid #2a3f5f;
                    border-radius: 8px;
                    padding: 12px;
                    font-family: monospace;
                    font-size: 13px;
                    resize: vertical;
                    min-height: 120px;
                }

                .dr-test-result {
                    margin-top: 24px;
                    background: #16213e;
                    border-radius: 12px;
                    padding: 20px;
                }

                .dr-result-badge {
                    display: inline-flex;
                    align-items: center;
                    gap: 8px;
                    padding: 10px 16px;
                    border-radius: 8px;
                    font-weight: 600;
                    font-size: 13px;
                    margin-bottom: 16px;
                }

                .dr-result-badge.success {
                    background: linear-gradient(135deg, #11998e, #38ef7d);
                }

                .dr-result-badge.error {
                    background: linear-gradient(135deg, #eb3349, #f45c43);
                }

                .dr-result-metrics {
                    display: flex;
                    gap: 12px;
                    margin-bottom: 16px;
                }

                .dr-metric {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    padding: 6px 12px;
                    background: rgba(102,126,234,0.1);
                    border-radius: 6px;
                    font-size: 12px;
                    color: #667eea;
                }

                .dr-result-data, .dr-steps {
                    margin-top: 16px;
                }

                .dr-result-data h4, .dr-steps h4 {
                    margin: 0 0 12px 0;
                    font-size: 12px;
                    text-transform: uppercase;
                    color: #8899a6;
                }

                .dr-result-data pre {
                    background: #0a0e14;
                    padding: 12px;
                    border-radius: 6px;
                    font-size: 12px;
                    overflow-x: auto;
                }

                .dr-step {
                    display: flex;
                    align-items: flex-start;
                    gap: 10px;
                    padding: 10px;
                    background: #0a0e14;
                    border-radius: 6px;
                    margin-bottom: 6px;
                    font-size: 12px;
                }

                .dr-step-num {
                    background: #667eea;
                    color: white;
                    width: 20px;
                    height: 20px;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 10px;
                    font-weight: 700;
                    flex-shrink: 0;
                }

                .dr-preview-panel {
                    padding: 20px;
                    overflow-y: auto;
                }

                .dr-preview-section {
                    background: #16213e;
                    border-radius: 12px;
                    padding: 20px;
                    margin-bottom: 16px;
                }

                .dr-preview-section h4 {
                    margin: 0 0 16px 0;
                    font-size: 14px;
                    color: #667eea;
                }

                .dr-info-grid {
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    gap: 16px;
                }

                .dr-info-grid label {
                    display: block;
                    font-size: 11px;
                    text-transform: uppercase;
                    color: #8899a6;
                    margin-bottom: 6px;
                }

                .dr-info-grid code {
                    background: #0a0e14;
                    padding: 6px 10px;
                    border-radius: 4px;
                    font-size: 12px;
                    display: block;
                }

                .dr-method-badge {
                    background: #764ba2;
                    padding: 4px 10px;
                    border-radius: 4px;
                    font-size: 11px;
                    font-weight: 700;
                    display: inline-block;
                }

                .status-enabled {
                    color: #38ef7d;
                }

                .status-disabled {
                    color: #ff4757;
                }

                .dr-preview-section pre {
                    background: #0a0e14;
                    padding: 16px;
                    border-radius: 8px;
                    overflow-x: auto;
                    font-size: 12px;
                    line-height: 1.6;
                }
            `}</style>
        </div>
    );
};