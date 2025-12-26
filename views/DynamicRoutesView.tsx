import React, { useState, useEffect } from 'react';
import {
    Code, Play, Plus, Trash2, CheckCircle, XCircle,
    Zap, FileJson, Activity, Terminal, Copy, RefreshCw, Globe, Clock, Save,
    File, ChevronDown, ChevronRight, Settings, Maximize2, Minimize2,
    Search, GitBranch, Braces, Layout, PanelLeft
} from 'lucide-react';
import Editor from "@monaco-editor/react";
import { CodeEditor } from '../components/SyntaxHighlighter';
import { getApiBaseUrl } from '../api';

const ROUTE_TEMPLATES = {
    yamlSimple: {
        name: "YAML: Hello World",
        json: `name: Hello World API
description: Simple greeting endpoint
path: /api/custom/hello
method: GET
enabled: true
version: 1.0.0
auth_required: false

logic:
  - return:
      value:
        message: Hello from Worpen!
        timestamp: "{{now()}}"
        status: success`
    },
    yamlConditional: {
        name: "YAML: Age Checker",
        json: `name: Age Verification
description: Check if user is adult
path: /api/custom/verify-age
method: POST
enabled: true

parameters:
  - name: age
    param_type: body
    data_type: number
    required: true

logic:
  - if:
      condition: age >= 18
      then:
        - return:
            value:
              status: adult
              message: Access granted
              can_vote: true
      otherwise:
        - return:
            value:
              status: minor
              message: Access denied
              can_vote: false`
    },
    yamlDatabase: {
        name: "YAML: DB Query",
        json: `name: User Lookup
description: Find user by ID
path: /api/custom/user/:id
method: GET
enabled: true

parameters:
  - name: id
    param_type: path
    data_type: number
    required: true

logic:
  - query_db:
      query: SELECT * FROM agents WHERE id = $1 LIMIT 1
      params:
        - "{{path.id}}"
  
  - if:
      condition: db_result.length > 0
      then:
        - return:
            value:
              found: true
              user: "{{db_result[0]}}"
      otherwise:
        - return:
            value:
              found: false
              error: User not found`
    },
    yamlLoop: {
        name: "YAML: Loop Example",
        json: `name: Batch Processor
description: Process multiple items
path: /api/custom/process-batch
method: POST
enabled: true

parameters:
  - name: items
    param_type: body
    data_type: array
    required: true

logic:
  - set:
      var: results
      value: []
  
  - for_each:
      collection: "{{body.items}}"
      operations:
        - set:
            var: processed
            value:
              id: "{{item.id}}"
              status: processed
              timestamp: "{{now()}}"
        
        - set:
            var: results
            value: "{{results + [processed]}}"
  
  - return:
      value:
        total: "{{body.items.length}}"
        processed: "{{results}}"`
    },
    simple: {
        name: "JSON: Simple Echo",
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
    },
    advanced: {
        name: "Advanced Showcase",
        json: JSON.stringify({
            name: "Order Processing API",
            description: "Complex order processing with all features",
            path: "/api/custom/process-order",
            method: "POST",
            logic: [
                {
                    "set": {
                        "var": "user_id",
                        "value": "{{body.user_id}}"
                    }
                },
                {
                    "query_db": {
                        "query": "SELECT balance FROM users WHERE id = $1",
                        "params": ["{{user_id}}"]
                    }
                },
                {
                    "set": {
                        "var": "balance",
                        "value": "{{db_result[0].balance}}"
                    }
                },
                {
                    "if": {
                        "condition": "balance >= body.amount",
                        "then": [
                            {
                                "http_request": {
                                    "url": "https://api.payment.com/charge",
                                    "method": "POST",
                                    "body": {
                                        "user_id": "{{user_id}}",
                                        "amount": "{{body.amount}}"
                                    }
                                }
                            },
                            {
                                "query_db": {
                                    "query": "INSERT INTO orders (user_id, amount, status) VALUES ($1, $2, $3)",
                                    "params": ["{{user_id}}", "{{body.amount}}", "completed"]
                                }
                            },
                            {
                                "return": {
                                    "value": {
                                        "success": true,
                                        "order_id": "{{db_result.id}}",
                                        "message": "Order processed successfully"
                                    }
                                }
                            }
                        ],
                        "otherwise": [
                            {
                                "return": {
                                    "value": {
                                        "success": false,
                                        "error": "Insufficient balance",
                                        "required": "{{body.amount}}",
                                        "available": "{{balance}}"
                                    }
                                }
                            }
                        ]
                    }
                }
            ],
            parameters: [
                { "name": "user_id", "param_type": "body", "data_type": "number", "required": true },
                { "name": "amount", "param_type": "body", "data_type": "number", "required": true }
            ],
            enabled: true,
            version: "1.0.0",
            auth_required: true
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
    const [sidebarWidth, setSidebarWidth] = useState(() => {
        const saved = localStorage.getItem('worpen_builder_sidebar_width');
        return saved !== null ? parseFloat(saved) : 33;
    });
    const [isDragging, setIsDragging] = useState(false);
    const [sidebarVisible, setSidebarVisible] = useState(() => {
        const saved = localStorage.getItem('worpen_builder_sidebar_visible');
        return saved !== null ? JSON.parse(saved) : true;
    });
    const [registrationError, setRegistrationError] = useState<string | null>(null);
    const [activeActivityId, setActiveActivityId] = useState<'explorer' | 'search' | 'git' | 'debug' | 'settings'>('explorer');
    const [expandedFolders, setExpandedFolders] = useState<string[]>(['templates', 'active-routes']);

    useEffect(() => {
        localStorage.setItem('worpen_builder_sidebar_width', sidebarWidth.toString());
    }, [sidebarWidth]);

    useEffect(() => {
        localStorage.setItem('worpen_builder_sidebar_visible', JSON.stringify(sidebarVisible));
    }, [sidebarVisible]);

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
            const response = await fetch(`${getApiBaseUrl()}/api/v1/dynamic-routes`);
            const data = await response.json();
            setRoutes(data);
        } catch (error) {
            console.error('Failed to fetch routes:', error);
        }
    };

    const registerRoute = async () => {
        if (!routeDefinition.trim()) return;

        setLoading(true);
        setRegistrationError(null);
        try {
            // Detect if it's YAML or JSON
            const isYAML = !routeDefinition.trim().startsWith('{') && !routeDefinition.trim().startsWith('[');
            const contentType = isYAML ? 'application/x-yaml' : 'application/json';

            // Use /register endpoint which supports both YAML and JSON
            const response = await fetch(`${getApiBaseUrl()}/api/v1/dynamic-routes/register`, {
                method: 'POST',
                headers: { 'Content-Type': contentType },
                body: routeDefinition,
            });

            if (response.ok) {
                await fetchRoutes();
                setRouteDefinition('');
                setActiveTab('editor');
                setRegistrationError(null);
            } else {
                const errorText = await response.text();
                try {
                    const errorData = JSON.parse(errorText);
                    setRegistrationError(errorData.error || errorData.details || errorData.message || 'Failed to register route');
                } catch {
                    setRegistrationError(errorText || 'Failed to register route');
                }
            }
        } catch (error: any) {
            setRegistrationError(error.message || 'Network error occurred');
            console.error(error);
        } finally {
            setLoading(false);
        }
    };

    const deleteRoute = async (routeId: string) => {
        if (!confirm('Delete this route?')) return;

        try {
            const response = await fetch(`${getApiBaseUrl()}/api/v1/dynamic-routes/${routeId}`, {
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
            const response = await fetch(`${getApiBaseUrl()}/api/v1/dynamic-routes/test`, {
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
        setRegistrationError(null); // Clear any previous errors
    };


    return (
        <div className="flex flex-col h-full w-full bg-[#1e1f22] text-[#dfe1e5] font-sans selection:bg-[#3574f030] overflow-hidden">
            {/* IDE Main Workspace */}
            <div id="routes-container" className="flex flex-1 overflow-hidden">
                {/* 1. Activity Bar (Far Left) */}
                <div className="w-10 bg-[#2b2d30] flex flex-col items-center py-2 gap-2 border-r border-[#43454a]">
                    {[
                        { id: 'explorer', icon: File, label: 'Explorer' },
                        { id: 'search', icon: Search, label: 'Search' },
                        { id: 'git', icon: GitBranch, label: 'Source Control' },
                        { id: 'debug', icon: Play, label: 'Run and Debug' },
                    ].map((item) => (
                        <button
                            key={item.id}
                            onClick={() => {
                                if (activeActivityId === item.id) {
                                    setSidebarVisible(!sidebarVisible);
                                } else {
                                    setActiveActivityId(item.id as any);
                                    setSidebarVisible(true);
                                }
                            }}
                            className={`p-2 transition-all relative group rounded-[var(--radius)] mx-1 ${activeActivityId === item.id && sidebarVisible ? 'text-[#3574f0] bg-[#3574f015]' : 'text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40]'
                                }`}
                            title={item.label}
                        >
                            <item.icon size={20} strokeWidth={activeActivityId === item.id ? 2.5 : 2} />
                            {activeActivityId === item.id && sidebarVisible && (
                                <div className="absolute left-[-4px] top-1/2 -translate-y-1/2 w-1 h-5 bg-[#3574f0] rounded-r-full shadow-[0_0_8px_rgba(53,116,240,0.3)]" />
                            )}
                        </button>
                    ))}
                    <div className="mt-auto flex flex-col gap-2 mb-1">
                        <button className="p-2 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] transition-all rounded-[var(--radius)] mx-1">
                            <Settings size={20} strokeWidth={2} />
                        </button>
                    </div>
                </div>

                {/* 2. Sidebar (Explorer/Search etc.) */}
                {sidebarVisible && (
                    <div
                        className="bg-[#2b2d30] flex flex-col border-r border-[#43454a] select-none group/sidebar relative"
                        style={{ width: `${sidebarWidth}%`, minWidth: '180px' }}
                    >
                        {/* Sidebar Header */}
                        <div className="h-9 px-4 flex items-center justify-between text-[11px] uppercase tracking-wider text-[#6e7073] font-bold border-b border-[#43454a]">
                            <span>{activeActivityId}</span>
                        </div>

                        {/* Sidebar Content */}
                        <div className="flex-1 overflow-y-auto custom-scrollbar flex flex-col py-2">
                            {activeActivityId === 'explorer' && (
                                <>
                                    {/* Templates Section */}
                                    <div className="flex flex-col mb-2">
                                        <button
                                            onClick={() => setExpandedFolders(prev =>
                                                prev.includes('templates') ? prev.filter(f => f !== 'templates') : [...prev, 'templates']
                                            )}
                                            className="h-7 px-2 flex items-center hover:bg-[#393b40] text-[11px] font-bold text-[#dfe1e5] transition-colors"
                                        >
                                            <div className="mr-1 shadow-sm">
                                                {expandedFolders.includes('templates') ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                                            </div>
                                            <span className="uppercase tracking-tight">ROUTE TEMPLATES</span>
                                        </button>
                                        {expandedFolders.includes('templates') && (
                                            <div className="flex flex-col py-1">
                                                {Object.entries(ROUTE_TEMPLATES).map(([key, template]) => (
                                                    <button
                                                        key={key}
                                                        onClick={() => loadTemplate(key)}
                                                        className="px-6 py-1.5 text-[12px] flex items-center gap-2 hover:bg-[#3574f015] hover:text-[#3574f0] text-[#dfe1e5] transition-all group text-left mx-1 rounded-[var(--radius)]"
                                                    >
                                                        <Braces size={14} className="text-[#3574f0] opacity-70" />
                                                        <span className="truncate">{template.name}</span>
                                                        <Plus size={12} className="ml-auto opacity-0 group-hover:opacity-100 text-[#59a869]" />
                                                    </button>
                                                ))}
                                            </div>
                                        )}
                                    </div>

                                    {/* Active Routes Section */}
                                    <div className="flex flex-col">
                                        <button
                                            onClick={() => setExpandedFolders(prev =>
                                                prev.includes('active-routes') ? prev.filter(f => f !== 'active-routes') : [...prev, 'active-routes']
                                            )}
                                            className="h-7 px-2 flex items-center hover:bg-[#393b40] text-[11px] font-bold text-[#dfe1e5] transition-colors"
                                        >
                                            <div className="mr-1 shadow-sm">
                                                {expandedFolders.includes('active-routes') ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                                            </div>
                                            <span className="uppercase tracking-tight">ACTIVE ROUTES ({routes.length})</span>
                                        </button>
                                        {expandedFolders.includes('active-routes') && (
                                            <div className="flex flex-col py-1">
                                                {routes.length === 0 ? (
                                                    <div className="px-8 py-4 text-center text-[11px] text-[#6e7073] italic">
                                                        No dynamic routes found
                                                    </div>
                                                ) : (
                                                    routes.map((route) => (
                                                        <div
                                                            key={route.id}
                                                            className={`group px-6 py-1.5 text-[12px] flex items-center gap-2 cursor-pointer transition-all mx-1 rounded-[var(--radius)] ${selectedRoute?.id === route.id ? 'bg-[#3574f0] text-white shadow-md' : 'hover:bg-[#393b40] text-[#dfe1e5]'
                                                                }`}
                                                            onClick={() => {
                                                                setSelectedRoute(route);
                                                                setRouteDefinition(route.yaml_definition || JSON.stringify(route, null, 2));
                                                                setActiveTab('editor');
                                                            }}
                                                        >
                                                            <Globe size={14} className={selectedRoute?.id === route.id ? "text-white" : (route.enabled ? "text-[#59a869]" : "text-[#6e7073]")} />
                                                            <div className="flex flex-col overflow-hidden">
                                                                <span className="truncate font-medium">{route.name}</span>
                                                                <span className={`truncate text-[10px] ${selectedRoute?.id === route.id ? 'text-white/70' : 'text-[#6e7073]'}`}>{route.path}</span>
                                                            </div>
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    deleteRoute(route.id);
                                                                }}
                                                                className={`ml-auto opacity-0 group-hover:opacity-100 p-1 hover:bg-red-500/20 rounded transition-all ${selectedRoute?.id === route.id ? 'text-white' : 'text-[#e06c75]'}`}
                                                            >
                                                                <Trash2 size={12} />
                                                            </button>
                                                        </div>
                                                    ))
                                                )}
                                            </div>
                                        )}
                                    </div>
                                </>
                            )}
                        </div>

                        {/* Resize Handle */}
                        <div
                            onMouseDown={() => setIsDragging(true)}
                            className={`absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-[#3574f050] transition-colors z-10 ${isDragging ? 'bg-[#3574f0]' : ''}`}
                        />
                    </div>
                )}

                {/* 3. Main Editor Area */}
                <div className="flex-1 flex flex-col bg-[#1e1f22] overflow-hidden m-1.5 rounded-[var(--radius)] border border-[#43454a] shadow-lg">
                    {/* Tabs / Breadcrumbs */}
                    <div className="h-9 bg-[#2b2d30] flex items-center overflow-x-auto no-scrollbar border-b border-[#43454a]">
                        <button
                            onClick={() => setActiveTab('editor')}
                            className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative ${activeTab === 'editor' ? 'bg-[#1e1f22] text-[#dfe1e5]' : 'text-[#6e7073] hover:bg-[#393b40] hover:text-[#dfe1e5]'
                                }`}
                        >
                            <Braces size={14} className="text-[#3574f0] opacity-70" />
                            <span>{selectedRoute ? `${selectedRoute.name}.yaml` : 'new-route.yaml'}</span>
                            {activeTab === 'editor' && (
                                <div className="absolute top-0 inset-x-0 h-[2px] bg-[#3574f0]" />
                            )}
                        </button>
                        <button
                            onClick={() => setActiveTab('test')}
                            className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative border-l border-[#43454a] ${activeTab === 'test' ? 'bg-[#1e1f22] text-[#dfe1e5]' : 'text-[#6e7073] hover:bg-[#393b40] hover:text-[#dfe1e5]'
                                }`}
                        >
                            <Terminal size={14} className="text-[#59a869] opacity-70" />
                            <span>Debug Console</span>
                            {activeTab === 'test' && (
                                <div className="absolute top-0 inset-x-0 h-[2px] bg-[#3574f0]" />
                            )}
                        </button>

                        <div className="ml-auto flex items-center gap-2 px-4">
                            {activeTab === 'editor' && (
                                <button
                                    onClick={registerRoute}
                                    disabled={loading || !routeDefinition.trim()}
                                    className="flex items-center gap-1.5 px-3 py-1 bg-[#3574f0] hover:bg-[#3574f0e0] disabled:bg-[#43454a] text-white text-[11px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                                >
                                    {loading ? <RefreshCw size={12} className="animate-spin" /> : <Save size={12} />}
                                    DEPLOY
                                </button>
                            )}
                        </div>
                    </div>

                    {/* Editor Content */}
                    <div className="flex-1 flex flex-col relative overflow-hidden bg-[#1e1f22]">
                        {activeTab === 'editor' && (
                            <div className="h-full w-full">
                                <Editor
                                    height="100%"
                                    defaultLanguage="yaml"
                                    theme="vs-dark"
                                    value={routeDefinition}
                                    onChange={(val) => setRouteDefinition(val || '')}
                                    options={{
                                        minimap: { enabled: true },
                                        fontSize: 14,
                                        fontFamily: 'JetBrains Mono, Menlo, Monaco, Courier New, monospace',
                                        lineNumbers: 'on',
                                        roundedSelection: true,
                                        scrollBeyondLastLine: false,
                                        readOnly: false,
                                        automaticLayout: true,
                                        padding: { top: 16 },
                                        wordWrap: 'on'
                                    }}
                                />
                                {registrationError && (
                                    <div className="absolute bottom-4 left-4 right-4 bg-[#e06c75] text-white p-4 rounded-[var(--radius)] shadow-2xl animate-fade-in flex items-start gap-4 z-20 border border-white/20">
                                        <XCircle className="mt-0.5 shrink-0" size={20} />
                                        <div className="flex-1">
                                            <p className="text-[12px] font-bold uppercase mb-1 drop-shadow-sm">Deployment Failed</p>
                                            <p className="text-[13px] font-medium leading-relaxed opacity-90">{registrationError}</p>
                                        </div>
                                        <button onClick={() => setRegistrationError(null)} className="hover:bg-white/20 rounded p-1 transition-colors">
                                            <XCircle size={18} />
                                        </button>
                                    </div>
                                )}
                            </div>
                        )}

                        {activeTab === 'test' && (
                            <div className="flex flex-col h-full bg-[#1e1f22]">
                                <div className="flex-1 p-4 flex flex-col gap-4 overflow-y-auto custom-scrollbar">
                                    <div className="flex items-center justify-between border-b border-[#43454a] pb-3">
                                        <div className="flex items-center gap-2">
                                            <Terminal size={18} className="text-[#3574f0]" />
                                            <h3 className="text-[12px] font-bold text-[#dfe1e5] uppercase tracking-wider">Test Environment</h3>
                                        </div>
                                        <button
                                            onClick={testRoute}
                                            disabled={loading || !selectedRoute}
                                            className="flex items-center gap-1.5 px-4 py-1.5 bg-[#59a869] hover:bg-[#59a869e0] disabled:bg-[#43454a] text-white text-[12px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                                        >
                                            {loading ? <RefreshCw size={12} className="animate-spin" /> : <Play size={12} />}
                                            RUN TEST
                                        </button>
                                    </div>

                                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 flex-1">
                                        {/* Payload Input */}
                                        <div className="flex flex-col gap-2">
                                            <div className="flex items-center gap-2 px-3 py-1.5 bg-[#2b2d30] rounded-[var(--radius)] border border-[#43454a]">
                                                <FileJson size={14} className="text-[#6e7073]" />
                                                <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-tight">Request Body</span>
                                            </div>
                                            <div className="flex-1 border border-[#43454a] rounded-[var(--radius)] overflow-hidden bg-[#1e1f22]">
                                                <Editor
                                                    height="100%"
                                                    defaultLanguage="json"
                                                    theme="vs-dark"
                                                    value={testPayload}
                                                    onChange={(val) => setTestPayload(val || '')}
                                                    options={{
                                                        minimap: { enabled: false },
                                                        fontSize: 13,
                                                        lineNumbers: 'on',
                                                        scrollBeyondLastLine: false,
                                                        automaticLayout: true,
                                                        padding: { top: 12 }
                                                    }}
                                                />
                                            </div>
                                        </div>

                                        {/* Test Result Output */}
                                        <div className="flex flex-col gap-2">
                                            <div className="flex items-center justify-between px-3 py-1.5 bg-[#2b2d30] rounded-[var(--radius)] border border-[#43454a]">
                                                <div className="flex items-center gap-2">
                                                    <Activity size={14} className="text-[#6e7073]" />
                                                    <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-tight">System Response</span>
                                                </div>
                                                {testResult && (
                                                    <span className={`text-[11px] font-bold px-2 py-0.5 rounded ${testResult.success ? 'bg-[#59a86920] text-[#59a869]' : 'bg-[#e06c7520] text-[#e06c75]'}`}>
                                                        {testResult.success ? '200 OK' : 'ERROR'}
                                                    </span>
                                                )}
                                            </div>
                                            <div className={`flex-1 border rounded-[var(--radius)] bg-[#1e1f22] font-mono text-[13px] p-4 overflow-auto shadow-inner transition-colors ${testResult?.success ? 'border-[#59a86930] bg-[#59a86905]' : 'border-[#e06c7530] bg-[#e06c7505]'
                                                }`}>
                                                {testResult ? (
                                                    <pre className={`custom-scrollbar ${testResult.success ? 'text-[#59a869]' : 'text-[#e06c75]'}`}>
                                                        {JSON.stringify(testResult, null, 2)}
                                                    </pre>
                                                ) : (
                                                    <div className="h-full flex flex-col items-center justify-center text-[#6e7073] gap-3">
                                                        <Clock size={32} className="opacity-20" />
                                                        <span className="text-[13px] italic">Awaiting test execution...</span>
                                                    </div>
                                                )}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                </div>
            </div>

            {/* 4. Status Bar (Bottom) */}
            <div className="h-6 bg-[#3574f0] text-white flex items-center justify-between text-[11px] font-medium shadow-lg z-20 select-none">
                <div className="flex items-center h-full">
                    <div className="flex items-center gap-2 hover:bg-white/10 px-3 h-full cursor-pointer transition-colors border-r border-white/5">
                        <GitBranch size={12} className="opacity-90" />
                        <span className="translate-y-[0.5px]">main*</span>
                    </div>
                    <div className="flex items-center gap-2 hover:bg-white/10 px-3 h-full cursor-pointer transition-colors">
                        <RefreshCw size={12} className={`${loading ? "animate-spin" : ""} opacity-90`} />
                        <span className="translate-y-[0.5px]">{routes.length} Active Endpoints</span>
                    </div>
                </div>
                <div className="flex items-center h-full">
                    <div className="flex items-center gap-1.5 hover:bg-white/10 px-3 h-full cursor-pointer transition-colors">
                        <span className="translate-y-[0.5px]">Ln {routeDefinition.split('\n').length}, Col 1</span>
                    </div>
                    <div className="flex items-center hover:bg-white/10 px-3 h-full cursor-pointer uppercase transition-colors">
                        <span className="translate-y-[0.5px]">UTF-8</span>
                    </div>
                    <div className="flex items-center gap-2 bg-white/15 px-4 h-full cursor-pointer shadow-inner border-l border-white/10">
                        <Zap size={12} className="fill-white" />
                        <span className="font-bold tracking-tight translate-y-[0.5px]">WORPEN IDE</span>
                    </div>
                </div>
            </div>
        </div>
    );
};
