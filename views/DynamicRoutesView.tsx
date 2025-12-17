import React, { useState, useEffect } from 'react';
import { 
    Code, Play, Plus, Trash2, CheckCircle, XCircle, 
    Zap, FileJson, Activity, Terminal, Copy, RefreshCw, Globe, Clock, Save
} from 'lucide-react';

const API_BASE_URL = 'http://127.0.0.1:3000';

const ROUTE_TEMPLATES = {
  simple: {
    name: "Simple Echo",
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
    const [activeTab, setActiveTab] = useState<'editor' | 'test'>('editor');

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
                setRouteDefinition('');
                setActiveTab('editor');
            }
        } catch (error: any) {
            console.error(error);
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
    };

    return (
        <div className="p-6 space-y-6 max-w-[1800px] mx-auto pb-20">
            {/* Header */}
            <div className="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <h1 className="text-3xl font-bold text-white tracking-tight mb-2">Dynamic Routes</h1>
                    <div className="flex items-center gap-6 text-sm text-gray-500 font-mono">
                        <span className="flex items-center gap-2">
                            <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                            ENGINE: ACTIVE
                        </span>
                        <span className="flex items-center gap-2">
                            <Globe size={14} />
                            JSON-DRIVEN API
                        </span>
                        <span className="flex items-center gap-2">
                            <Clock size={14} />
                            0ms EXECUTION
                        </span>
                    </div>
                </div>

                <div className="flex gap-2">
                    <button 
                        onClick={fetchRoutes}
                        className="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 text-xs font-mono text-gray-300 rounded hover:text-white transition-colors flex items-center gap-2"
                    >
                        <RefreshCw size={14} />
                        REFRESH
                    </button>
                    <button 
                        onClick={() => setActiveTab('editor')}
                        className="px-4 py-2 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-xs font-mono text-green-400 rounded hover:text-green-300 transition-colors flex items-center gap-2"
                    >
                        <Plus size={14} />
                        NEW ROUTE
                    </button>
                </div>
            </div>

            {/* Stats Grid */}
            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <StatCard icon={Activity} value={routes.length} label="Active Routes" color="green" />
                <StatCard icon={Zap} value="0ms" label="Execution" color="blue" />
                <StatCard icon={Terminal} value="11" label="Logic Ops" color="purple" />
            </div>

            {/* Main Content */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                {/* Templates + Routes List */}
                <div className="lg:col-span-1 space-y-4">
                    {/* Quick Start Templates */}
                    <div className="bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 rounded-lg p-4">
                        <div className="flex items-center gap-2 mb-4">
                            <FileJson size={16} className="text-green-500" />
                            <h3 className="text-sm font-bold text-white uppercase tracking-wide">Quick Start</h3>
                        </div>
                        <div className="space-y-2">
                            {Object.entries(ROUTE_TEMPLATES).map(([key, template]) => (
                                <button
                                    key={key}
                                    onClick={() => loadTemplate(key)}
                                    className="w-full flex items-center gap-3 p-3 bg-white/[0.02] border border-white/5 hover:bg-white/[0.04] hover:border-green-500/20 transition-all group rounded text-left"
                                >
                                    <div className="p-2 rounded bg-black/50 border border-white/10 text-green-500">
                                        <Code size={14} />
                                    </div>
                                    <span className="text-xs font-bold text-gray-300 group-hover:text-white">{template.name}</span>
                                </button>
                            ))}
                        </div>
                    </div>

                    {/* Your Routes */}
                    <div className="bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 rounded-lg p-4">
                        <div className="flex items-center justify-between mb-4">
                            <div className="flex items-center gap-2">
                                <Terminal size={16} className="text-green-500" />
                                <h3 className="text-sm font-bold text-white uppercase tracking-wide">Your Routes</h3>
                            </div>
                            <span className="text-xs font-mono text-gray-500 bg-white/5 px-2 py-1 rounded">{routes.length}</span>
                        </div>
                        <div className="space-y-2 max-h-[400px] overflow-y-auto pr-2">
                            {routes.map((route) => (
                                <div
                                    key={route.id}
                                    onClick={() => setSelectedRoute(route)}
                                    className={`flex items-center justify-between p-3 border rounded transition-all cursor-pointer ${
                                        selectedRoute?.id === route.id
                                            ? 'bg-green-500/10 border-green-500/30'
                                            : 'bg-white/[0.02] border-white/5 hover:bg-white/[0.04] hover:border-green-500/20'
                                    }`}
                                >
                                    <div className="flex items-center gap-3 flex-1 min-w-0">
                                        <span className="text-[10px] font-mono px-2 py-1 bg-green-500/10 text-green-400 rounded border border-green-500/20">
                                            {route.method}
                                        </span>
                                        <div className="flex-1 min-w-0">
                                            <div className="text-xs font-bold text-white truncate">{route.name}</div>
                                            <div className="text-[10px] text-gray-500 font-mono truncate">{route.path}</div>
                                        </div>
                                    </div>
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            deleteRoute(route.id);
                                        }}
                                        className="p-1.5 text-gray-500 hover:text-red-400 hover:bg-red-500/10 rounded transition-colors"
                                    >
                                        <Trash2 size={12} />
                                    </button>
                                </div>
                            ))}
                            {routes.length === 0 && (
                                <div className="text-center py-8 text-gray-600 text-xs">
                                    No routes yet. Create one using templates above.
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* Editor / Test Panel */}
                <div className="lg:col-span-2">
                    <div className="bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 rounded-lg overflow-hidden">
                        {/* Tabs */}
                        <div className="flex border-b border-white/5 bg-black/20">
                            <button
                                onClick={() => setActiveTab('editor')}
                                className={`flex items-center gap-2 px-4 py-3 text-xs font-mono transition-colors ${
                                    activeTab === 'editor'
                                        ? 'bg-green-500/10 text-green-400 border-b-2 border-green-500'
                                        : 'text-gray-500 hover:text-gray-300 hover:bg-white/5'
                                }`}
                            >
                                <Code size={14} />
                                Editor
                            </button>
                            <button
                                onClick={() => setActiveTab('test')}
                                className={`flex items-center gap-2 px-4 py-3 text-xs font-mono transition-colors ${
                                    activeTab === 'test'
                                        ? 'bg-green-500/10 text-green-400 border-b-2 border-green-500'
                                        : 'text-gray-500 hover:text-gray-300 hover:bg-white/5'
                                }`}
                                disabled={!selectedRoute}
                            >
                                <Play size={14} />
                                Test
                            </button>
                        </div>

                        {/* Editor Tab */}
                        {activeTab === 'editor' && (
                            <div className="p-4 space-y-4">
                                <div className="flex items-center justify-between">
                                    <div className="flex items-center gap-2 text-xs text-gray-500 font-mono">
                                        <Terminal size={14} />
                                        JSON EDITOR
                                    </div>
                                    <button
                                        onClick={registerRoute}
                                        disabled={loading || !routeDefinition.trim()}
                                        className="px-3 py-1.5 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-xs font-mono text-green-400 rounded hover:text-green-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                    >
                                        {loading ? <RefreshCw size={12} className="animate-spin" /> : <Save size={12} />}
                                        {loading ? 'REGISTERING...' : 'REGISTER ROUTE'}
                                    </button>
                                </div>
                                <textarea
                                    value={routeDefinition}
                                    onChange={(e) => setRouteDefinition(e.target.value)}
                                    placeholder="Paste your JSON route definition here..."
                                    className="w-full h-[500px] bg-black/50 border border-white/10 rounded p-4 text-xs font-mono text-gray-300 focus:outline-none focus:border-green-500/30 focus:ring-1 focus:ring-green-500/20 resize-none"
                                />
                            </div>
                        )}

                        {/* Test Tab */}
                        {activeTab === 'test' && selectedRoute && (
                            <div className="p-4 space-y-4">
                                <div className="bg-black/30 border border-white/10 rounded p-4 space-y-2">
                                    <div className="flex items-center justify-between">
                                        <div className="text-xs font-mono text-gray-500">SELECTED ROUTE</div>
                                        <span className="text-[10px] font-mono px-2 py-1 bg-green-500/10 text-green-400 rounded border border-green-500/20">
                                            {selectedRoute.method}
                                        </span>
                                    </div>
                                    <div className="text-sm font-bold text-white">{selectedRoute.name}</div>
                                    <div className="text-xs text-gray-500 font-mono">{selectedRoute.path}</div>
                                    {selectedRoute.description && (
                                        <div className="text-xs text-gray-400 pt-2 border-t border-white/5">
                                            {selectedRoute.description}
                                        </div>
                                    )}
                                </div>

                                <div>
                                    <div className="flex items-center justify-between mb-2">
                                        <label className="text-xs font-mono text-gray-500">TEST PAYLOAD</label>
                                        <button
                                            onClick={testRoute}
                                            disabled={loading}
                                            className="px-3 py-1.5 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-xs font-mono text-green-400 rounded hover:text-green-300 transition-colors disabled:opacity-50 flex items-center gap-2"
                                        >
                                            {loading ? <RefreshCw size={12} className="animate-spin" /> : <Play size={12} />}
                                            {loading ? 'TESTING...' : 'RUN TEST'}
                                        </button>
                                    </div>
                                    <textarea
                                        value={testPayload}
                                        onChange={(e) => setTestPayload(e.target.value)}
                                        placeholder='{"key": "value"}'
                                        className="w-full h-32 bg-black/50 border border-white/10 rounded p-4 text-xs font-mono text-gray-300 focus:outline-none focus:border-green-500/30 focus:ring-1 focus:ring-green-500/20 resize-none"
                                    />
                                </div>

                                {testResult && (
                                    <div>
                                        <div className="flex items-center gap-2 mb-2">
                                            {testResult.success ? (
                                                <CheckCircle size={14} className="text-green-500" />
                                            ) : (
                                                <XCircle size={14} className="text-red-500" />
                                            )}
                                            <label className="text-xs font-mono text-gray-500">RESULT</label>
                                        </div>
                                        <pre className={`w-full h-48 bg-black/50 border rounded p-4 text-xs font-mono overflow-auto ${
                                            testResult.success ? 'border-green-500/30 text-green-400' : 'border-red-500/30 text-red-400'
                                        }`}>
                                            {JSON.stringify(testResult, null, 2)}
                                        </pre>
                                    </div>
                                )}
                            </div>
                        )}

                        {activeTab === 'test' && !selectedRoute && (
                            <div className="p-8 text-center text-gray-600 text-sm">
                                Select a route from the list to test it
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};
