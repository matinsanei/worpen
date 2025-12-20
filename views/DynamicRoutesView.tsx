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
    const [sidebarWidth, setSidebarWidth] = useState(33); // Percentage
    const [isDragging, setIsDragging] = useState(false);
    const [sidebarVisible, setSidebarVisible] = useState(true);

    useEffect(() => {
        fetchRoutes();
        const interval = setInterval(fetchRoutes, 5000);
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        const handleMouseMove = (e: MouseEvent) => {
            if (!isDragging || !sidebarVisible) return;
            const container = document.getElementById('routes-container');
            if (!container) return;
            const containerRect = container.getBoundingClientRect();
            const newWidth = ((e.clientX - containerRect.left) / containerRect.width) * 100;
            setSidebarWidth(Math.min(Math.max(newWidth, 20), 60)); // Between 20% and 60%
        };

        const handleMouseUp = () => {
            setIsDragging(false);
        };

        if (isDragging) {
            document.addEventListener('mousemove', handleMouseMove);
            document.addEventListener('mouseup', handleMouseUp);
        }

        return () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };
    }, [isDragging, sidebarVisible]);

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
        <div className="max-w-[1800px] mx-auto h-full overflow-hidden flex flex-col relative">
            {/* Toggle Sidebar Button - Fixed at top */}
            <button
                onClick={() => setSidebarVisible(!sidebarVisible)}
                className="absolute left-4 top-4 z-50 p-2.5 bg-green-500/10 hover:bg-green-500/20 border-2 border-green-500/30 hover:border-green-500/50 text-green-400 rounded-lg transition-all duration-300 shadow-lg hover:shadow-green-500/20 hover:scale-110 active:scale-95"
                title={sidebarVisible ? "Hide Sidebar" : "Show Sidebar"}
            >
                <svg 
                    width="18" 
                    height="18" 
                    viewBox="0 0 24 24" 
                    fill="none" 
                    stroke="currentColor" 
                    strokeWidth="2.5"
                    className="transition-transform duration-300"
                    style={{ transform: sidebarVisible ? 'rotate(0deg)' : 'rotate(180deg)' }}
                >
                    <path d="M15 18l-6-6 6-6" />
                </svg>
            </button>

            {/* Main Content */}
            <div id="routes-container" className="flex gap-0 h-full relative pt-16">
                {/* Templates + Routes List */}
                {sidebarVisible && (
                    <div 
                        className="space-y-4 overflow-y-auto custom-scrollbar pr-4" 
                        style={{ 
                            width: `${sidebarWidth}%`,
                            transition: isDragging ? 'none' : 'width 0.5s ease-in-out'
                        }}
                    >
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
                    <div className="bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 rounded-lg p-4 flex-1 flex flex-col overflow-hidden">
                        <div className="flex items-center justify-between mb-4">
                            <div className="flex items-center gap-2">
                                <Terminal size={16} className="text-green-500" />
                                <h3 className="text-sm font-bold text-white uppercase tracking-wide">Routes</h3>
                            </div>
                            <div className="flex items-center gap-2">
                                <button 
                                    onClick={fetchRoutes}
                                    className="p-1.5 bg-white/5 hover:bg-white/10 border border-white/10 text-gray-400 rounded hover:text-white transition-colors"
                                    title="Refresh"
                                >
                                    <RefreshCw size={12} />
                                </button>
                                <button 
                                    onClick={() => setActiveTab('editor')}
                                    className="p-1.5 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-green-400 rounded hover:text-green-300 transition-colors"
                                    title="New Route"
                                >
                                    <Plus size={12} />
                                </button>
                                <span className="text-xs font-mono text-gray-500 bg-white/5 px-2 py-1 rounded">{routes.length}</span>
                            </div>
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
                )}

                {/* Resize Handle */}
                {sidebarVisible && (
                    <div 
                        className={`w-1 cursor-col-resize relative group ${
                            isDragging 
                                ? 'bg-green-500/50 w-2 shadow-[0_0_10px_rgba(34,197,94,0.5)]' 
                                : 'bg-white/5 hover:bg-green-500/30'
                        }`}
                        style={{ transition: isDragging ? 'none' : 'all 0.3s' }}
                        onMouseDown={(e) => {
                            e.preventDefault();
                            setIsDragging(true);
                        }}
                    >
                        <div className="absolute inset-y-0 -left-2 -right-2 flex items-center justify-center">
                            <div className={`w-1.5 h-16 rounded-full transition-all duration-300 ${
                                isDragging 
                                    ? 'bg-green-500/80 h-24 shadow-[0_0_15px_rgba(34,197,94,0.6)]' 
                                    : 'bg-green-500/0 group-hover:bg-green-500/60 group-hover:h-20'
                            }`}></div>
                        </div>
                    </div>
                )}

                {/* Editor / Test Panel */}
                <div 
                    className="overflow-hidden flex-1"
                    style={{ transition: isDragging ? 'none' : 'all 0.5s ease-in-out' }}
                >
                    <div className="bg-[#0a0a0a]/50 backdrop-blur-sm border border-white/5 rounded-lg overflow-hidden h-full flex flex-col">
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
                            <div className="p-4 space-y-4 flex-1 flex flex-col overflow-hidden">
                                <div className="flex items-center justify-between">
                                    <div className="flex items-center gap-2 text-xs text-gray-500 font-mono">
                                        <Terminal size={14} />
                                        JSON/YAML EDITOR
                                    </div>
                                    <div className="flex items-center gap-2">
                                        <button
                                            onClick={() => window.open('https://github.com/matinsanei/worpen/blob/main/DYNAMIC_ROUTES_GUIDE.md', '_blank')}
                                            className="px-3 py-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/20 text-xs font-mono text-blue-400 rounded hover:text-blue-300 transition-colors flex items-center gap-2"
                                        >
                                            <FileJson size={12} />
                                            DOCUMENTATION
                                        </button>
                                        <button
                                            onClick={registerRoute}
                                            disabled={loading || !routeDefinition.trim()}
                                            className="px-3 py-1.5 bg-green-500/10 hover:bg-green-500/20 border border-green-500/20 text-xs font-mono text-green-400 rounded hover:text-green-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                        >
                                            {loading ? <RefreshCw size={12} className="animate-spin" /> : <Save size={12} />}
                                            {loading ? 'REGISTERING...' : 'REGISTER ROUTE'}
                                        </button>
                                    </div>
                                </div>
                                
                                {/* Enhanced Code Editor with Line Numbers */}
                                <div className="relative group flex-1 overflow-hidden">
                                    {/* Line Numbers */}
                                    <div className="absolute left-0 top-0 bottom-0 w-12 bg-black/80 border-r border-white/10 flex flex-col pt-4 pb-4 text-[10px] font-mono text-gray-600 select-none overflow-hidden">
                                        {routeDefinition.split('\n').map((_, i) => (
                                            <div key={i} className="h-[18px] px-2 text-right leading-[18px]">
                                                {i + 1}
                                            </div>
                                        ))}
                                    </div>
                                    
                                    {/* Editor */}
                                    <textarea
                                        value={routeDefinition}
                                        onChange={(e) => setRouteDefinition(e.target.value)}
                                        placeholder={`Paste your JSON route definition here...\n\nExample:\n{\n  "name": "Hello API",\n  "path": "/api/hello",\n  "method": "GET",\n  "logic": [\n    { "return": { "value": { "message": "Hello World" } } }\n  ]\n}`}
                                        className="w-full h-full bg-black/50 border border-white/10 rounded p-4 pl-16 text-xs font-mono text-gray-300 focus:outline-none focus:border-green-500/30 focus:ring-2 focus:ring-green-500/10 resize-none leading-[18px] group-hover:border-green-500/20 transition-colors"
                                        style={{ 
                                            tabSize: 2,
                                            fontFamily: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace"
                                        }}
                                        spellCheck={false}
                                    />
                                    
                                    {/* Syntax Hints */}
                                    <div className="absolute top-4 right-4 flex gap-2">
                                        <div className="px-2 py-1 bg-green-500/10 border border-green-500/20 rounded text-[10px] font-mono text-green-400">
                                            JSON
                                        </div>
                                        <div className="px-2 py-1 bg-blue-500/10 border border-blue-500/20 rounded text-[10px] font-mono text-blue-400">
                                            YAML
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}

                        {/* Test Tab */}
                        {activeTab === 'test' && selectedRoute && (
                            <div className="p-4 space-y-4 flex-1 overflow-y-auto custom-scrollbar">
                                <div className="bg-black/30 border border-white/10 rounded p-0 space-y-2">
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
