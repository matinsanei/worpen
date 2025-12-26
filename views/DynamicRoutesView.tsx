import React, { useState, useEffect } from 'react';
import {
    Code, Play, Plus, Trash2, CheckCircle, XCircle,
    Zap, FileJson, Activity, Terminal, Copy, RefreshCw, Globe, Clock, Save,
    File, ChevronDown, ChevronRight, Settings, Maximize2, Minimize2,
    Search, GitBranch, Braces, Layout, PanelLeft,
    X
} from 'lucide-react';
import Editor, { loader } from "@monaco-editor/react";
import { CodeEditor } from '../components/SyntaxHighlighter';
import { getApiBaseUrl } from '../api';
import { ALL_THEMES, IDETheme, worpenDark } from '../src/themes/ide';

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
    const [currentTheme, setCurrentTheme] = useState<IDETheme>(() => {
        const saved = localStorage.getItem('worpen_ide_theme');
        if (saved) {
            const theme = ALL_THEMES.find(t => t.id === saved);
            if (theme) return theme;
        }
        return worpenDark;
    });

    const AVAILABLE_FONTS = [
        { name: 'JetBrains Mono', family: '"JetBrains Mono", monospace' },
        { name: 'Fira Code', family: '"Fira Code", monospace' },
        { name: 'IBM Plex Mono', family: '"IBM Plex Mono", monospace' },
        { name: 'Roboto Mono', family: '"Roboto Mono", monospace' },
        { name: 'Source Code Pro', family: '"Source Code Pro", monospace' },
        { name: 'Inconsolata', family: '"Inconsolata", monospace' },
        { name: 'Courier Prime', family: '"Courier Prime", monospace' },
        { name: 'Ubuntu Mono', family: '"Ubuntu Mono", monospace' },
        { name: 'Anonymous Pro', family: '"Anonymous Pro", monospace' },
        { name: 'Space Mono', family: '"Space Mono", monospace' },
        { name: 'Nanum Gothic Coding', family: '"Nanum Gothic Coding", monospace' },
        { name: 'Overpass Mono', family: '"Overpass Mono", monospace' },
        { name: 'Cutive Mono', family: '"Cutive Mono", monospace' },
        { name: 'Nova Mono', family: '"Nova Mono", monospace' },
    ];

    const [currentFont, setCurrentFont] = useState(() => {
        return localStorage.getItem('worpen_ide_font') || '"JetBrains Mono", monospace';
    });

    const [showSettingsModal, setShowSettingsModal] = useState(false);

    useEffect(() => {
        localStorage.setItem('worpen_ide_font', currentFont);
    }, [currentFont]);

    useEffect(() => {
        loader.init().then(monaco => {
            ALL_THEMES.forEach(theme => {
                monaco.editor.defineTheme(theme.editor.monacoTheme, {
                    base: theme.editor.monacoConfig.base,
                    inherit: theme.editor.monacoConfig.inherit,
                    rules: theme.editor.monacoConfig.rules as any,
                    colors: theme.editor.monacoConfig.colors,
                });
            });
        });
    }, []);

    useEffect(() => {
        localStorage.setItem('worpen_ide_theme', currentTheme.id);
    }, [currentTheme]);

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
        <div className="flex flex-col h-full w-full bg-[#1e1f22] text-[#dfe1e5] font-sans selection:bg-[#3574f030] overflow-hidden" style={{ backgroundColor: currentTheme.editor.bg }}>
            {/* IDE Main Workspace */}
            <div id="routes-container" className="flex flex-1 overflow-hidden">
                {/* 1. Activity Bar (Far Left) */}
                <div className="w-10 flex flex-col items-center py-2 gap-2 border-r" style={{ backgroundColor: currentTheme.activityBar.bg, borderColor: currentTheme.activityBar.border }}>
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
                            className={`p-2 transition-all relative group rounded-[var(--radius)] mx-1`}
                            style={{
                                color: activeActivityId === item.id && sidebarVisible ? currentTheme.activityBar.activeIcon : currentTheme.activityBar.inactiveIcon,
                                backgroundColor: activeActivityId === item.id && sidebarVisible ? currentTheme.activityBar.activeBg : 'transparent'
                            }}
                            onMouseEnter={(e) => {
                                if (!(activeActivityId === item.id && sidebarVisible)) {
                                    e.currentTarget.style.backgroundColor = currentTheme.activityBar.hoverBg;
                                    e.currentTarget.style.color = currentTheme.activityBar.activeIcon;
                                }
                            }}
                            onMouseLeave={(e) => {
                                if (!(activeActivityId === item.id && sidebarVisible)) {
                                    e.currentTarget.style.backgroundColor = 'transparent';
                                    e.currentTarget.style.color = currentTheme.activityBar.inactiveIcon;
                                }
                            }}
                            title={item.label}
                        >
                            <item.icon size={20} strokeWidth={activeActivityId === item.id ? 2.5 : 2} />
                            {activeActivityId === item.id && sidebarVisible && (
                                <div className="absolute left-[-4px] top-1/2 -translate-y-1/2 w-1 h-5 rounded-r-full shadow-[0_0_8px_rgba(53,116,240,0.3)]" style={{ backgroundColor: currentTheme.activityBar.indicator }} />
                            )}
                        </button>
                    ))}
                    <div className="mt-auto flex flex-col gap-2 mb-1 relative">
                        <button
                            onClick={() => setShowSettingsModal(true)}
                            className="p-2 transition-all rounded-[var(--radius)] mx-1"
                            style={{
                                color: currentTheme.activityBar.inactiveIcon,
                            }}
                            onMouseEnter={(e) => {
                                e.currentTarget.style.backgroundColor = currentTheme.activityBar.hoverBg;
                                e.currentTarget.style.color = currentTheme.activityBar.activeIcon;
                            }}
                            onMouseLeave={(e) => {
                                e.currentTarget.style.backgroundColor = 'transparent';
                                e.currentTarget.style.color = currentTheme.activityBar.inactiveIcon;
                            }}
                        >
                            <Settings size={20} strokeWidth={2} />
                        </button>

                        {/* Settings Modal Overlay */}
                        {showSettingsModal && (
                            <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-md animate-in fade-in duration-500">
                                <div
                                    className="w-[850px] h-[650px] rounded-2xl shadow-[0_32px_64px_-16px_rgba(0,0,0,0.6)] border flex overflow-hidden animate-blur-in"
                                    style={{ backgroundColor: currentTheme.sidebar.bg, borderColor: currentTheme.sidebar.border }}
                                >
                                    {/* Modal Sidebar */}
                                    <div className="w-60 border-r flex flex-col animate-in fade-in duration-700 delay-150" style={{ borderColor: currentTheme.sidebar.border, backgroundColor: currentTheme.activityBar.bg }}>
                                        <div className="p-8 border-b" style={{ borderColor: currentTheme.sidebar.border }}>
                                            <h2 className="text-xl font-black flex items-center gap-3 tracking-tighter" style={{ color: currentTheme.sidebar.headerText }}>
                                                <Settings size={24} className="opacity-80" /> Settings
                                            </h2>
                                        </div>
                                        <div className="flex-1 py-6 px-3">
                                            <div
                                                className="px-5 py-3 flex items-center gap-4 text-[14px] font-bold rounded-xl transition-all"
                                                style={{
                                                    color: currentTheme.sidebar.activeItemText,
                                                    backgroundColor: currentTheme.sidebar.activeItemBg,
                                                }}
                                            >
                                                <Layout size={20} /> Appearance
                                            </div>
                                            <div className="mt-2 px-5 py-3 flex items-center gap-4 text-[14px] font-medium opacity-30 cursor-not-allowed" style={{ color: currentTheme.sidebar.itemText }}>
                                                <Braces size={20} /> Editor Config
                                            </div>
                                            <div className="px-5 py-3 flex items-center gap-4 text-[14px] font-medium opacity-30 cursor-not-allowed" style={{ color: currentTheme.sidebar.itemText }}>
                                                <GitBranch size={20} /> Version Control
                                            </div>
                                        </div>
                                        <div className="p-8 text-[11px] font-bold opacity-30 tracking-widest" style={{ color: currentTheme.sidebar.itemText }}>
                                            WORPEN CORE v1.0.4.5
                                        </div>
                                    </div>

                                    {/* Modal Main Content */}
                                    <div className="flex-1 flex flex-col overflow-hidden animate-in fade-in duration-700 delay-300" style={{ backgroundColor: currentTheme.editor.bg }}>
                                        <div className="h-16 border-b flex items-center justify-between px-10" style={{ borderColor: currentTheme.sidebar.border }}>
                                            <div className="flex flex-col">
                                                <span className="text-xs font-black uppercase tracking-[3px] opacity-40" style={{ color: currentTheme.sidebar.headerText }}>Configuration</span>
                                                <span className="text-sm font-bold" style={{ color: currentTheme.sidebar.headerText }}>Interface & Branding</span>
                                            </div>
                                            <button
                                                onClick={() => setShowSettingsModal(false)}
                                                className="hover:scale-110 transition-transform"
                                                style={{ color: currentTheme.sidebar.headerText }}
                                            >
                                                <XCircle size={28} />
                                            </button>
                                        </div>

                                        <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
                                            {/* Theme Selection */}
                                            <section className="mb-10">
                                                <h3 className="text-[12px] font-black uppercase tracking-widest mb-4 opacity-70" style={{ color: currentTheme.activityBar.indicator }}>Color Theme</h3>
                                                <div className="grid grid-cols-2 lg:grid-cols-3 gap-3">
                                                    {ALL_THEMES.map(theme => (
                                                        <button
                                                            key={theme.id}
                                                            onClick={() => setCurrentTheme(theme)}
                                                            className="flex flex-col gap-3 p-4 rounded-lg border-2 transition-all hover:scale-[1.02]"
                                                            style={{
                                                                backgroundColor: currentTheme.id === theme.id ? theme.activityBar.bg : theme.activityBar.bg + '50',
                                                                borderColor: currentTheme.id === theme.id ? currentTheme.activityBar.indicator : 'transparent'
                                                            }}
                                                        >
                                                            <div className="flex items-center justify-between w-full">
                                                                <span className="text-[13px] font-bold" style={{ color: theme.sidebar.activeItemText }}>{theme.name}</span>
                                                                <div className="w-3 h-3 rounded-full" style={{ backgroundColor: theme.activityBar.indicator }} />
                                                            </div>
                                                            <div className="flex gap-1">
                                                                <div className="w-4 h-4 rounded" style={{ backgroundColor: theme.editor.bg }} />
                                                                <div className="w-4 h-4 rounded" style={{ backgroundColor: theme.sidebar.bg }} />
                                                                <div className="w-4 h-4 rounded" style={{ backgroundColor: theme.statusBar.bg }} />
                                                            </div>
                                                        </button>
                                                    ))}
                                                </div>
                                            </section>

                                            <div className="h-[1px] mb-10 opacity-10" style={{ backgroundColor: currentTheme.sidebar.headerText }} />

                                            {/* Font Selection */}
                                            <section>
                                                <h3 className="text-[12px] font-black uppercase tracking-widest mb-4 opacity-70" style={{ color: currentTheme.activityBar.indicator }}>Editor Font</h3>
                                                <div className="grid grid-cols-2 gap-3">
                                                    {AVAILABLE_FONTS.map(font => (
                                                        <button
                                                            key={font.family}
                                                            onClick={() => setCurrentFont(font.family)}
                                                            className="p-4 rounded-lg border-2 text-left transition-all hover:scale-[1.02]"
                                                            style={{
                                                                backgroundColor: currentFont === font.family ? currentTheme.activityBar.bg : currentTheme.activityBar.bg + '50',
                                                                borderColor: currentFont === font.family ? currentTheme.activityBar.indicator : 'transparent',
                                                                fontFamily: font.family
                                                            }}
                                                        >
                                                            <div className="text-[14px] mb-1" style={{ color: currentTheme.sidebar.activeItemText }}>{font.name}</div>
                                                            <div className="text-[11px] opacity-40 truncate" style={{ color: currentTheme.sidebar.itemText }}>
                                                                const worpen = () =&gt; "IDE Experience";
                                                            </div>
                                                        </button>
                                                    ))}
                                                </div>
                                            </section>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                </div>

                {/* 2. Sidebar (Explorer/Search etc.) */}
                {sidebarVisible && (
                    <div
                        className="flex flex-col border-r select-none group/sidebar relative"
                        style={{ width: `${sidebarWidth}%`, minWidth: '180px', backgroundColor: currentTheme.sidebar.bg, borderColor: currentTheme.sidebar.border }}
                    >
                        {/* Sidebar Header */}
                        <div className="h-9 px-4 flex items-center justify-between text-[11px] uppercase tracking-wider font-bold border-b"
                            style={{ color: currentTheme.sidebar.headerText, borderColor: currentTheme.sidebar.border }}>
                            <span>{activeActivityId}</span>
                        </div>

                        {/* Sidebar Content */}
                        <div className="flex-1 overflow-y-auto custom-scrollbar flex flex-col py-2">
                            {activeActivityId === 'explorer' && (
                                <>
                                    {/* Templates Section */}
                                    <button
                                        onClick={() => setExpandedFolders(prev =>
                                            prev.includes('templates') ? prev.filter(f => f !== 'templates') : [...prev, 'templates']
                                        )}
                                        className="h-7 px-2 flex items-center text-[11px] font-bold transition-colors"
                                        style={{ color: currentTheme.sidebar.folderText }}
                                        onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg}
                                        onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}
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
                                                    className="px-6 py-1.5 text-[12px] flex items-center gap-2 transition-all group text-left mx-1 rounded-[var(--radius)]"
                                                    style={{ color: currentTheme.sidebar.itemText }}
                                                    onMouseEnter={(e) => {
                                                        e.currentTarget.style.backgroundColor = currentTheme.sidebar.itemHoverBg;
                                                        e.currentTarget.querySelector('span')!.style.color = currentTheme.activityBar.indicator;
                                                    }}
                                                    onMouseLeave={(e) => {
                                                        e.currentTarget.style.backgroundColor = 'transparent';
                                                        e.currentTarget.querySelector('span')!.style.color = currentTheme.sidebar.itemText;
                                                    }}
                                                >
                                                    <Braces size={14} style={{ color: currentTheme.activityBar.activeIcon, opacity: 0.7 }} />
                                                    <span className="truncate">{template.name}</span>
                                                    <Plus size={12} className="ml-auto opacity-0 group-hover:opacity-100" style={{ color: currentTheme.sidebar.iconEnabled }} />
                                                </button>
                                            ))}
                                        </div>
                                    )}

                                    {/* Active Routes Section */}
                                    <div className="flex flex-col">
                                        <button
                                            onClick={() => setExpandedFolders(prev =>
                                                prev.includes('active-routes') ? prev.filter(f => f !== 'active-routes') : [...prev, 'active-routes']
                                            )}
                                            className="h-7 px-2 flex items-center text-[11px] font-bold transition-colors"
                                            style={{ color: currentTheme.sidebar.folderText }}
                                            onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg}
                                            onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}
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
                                                            className={`group px-6 py-1.5 text-[12px] flex items-center gap-2 cursor-pointer transition-all mx-1 rounded-[var(--radius)]`}
                                                            style={{
                                                                backgroundColor: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemBg : 'transparent',
                                                                color: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemText : currentTheme.sidebar.itemText
                                                            }}
                                                            onMouseEnter={(e: any) => {
                                                                if (selectedRoute?.id !== route.id) {
                                                                    e.currentTarget.style.backgroundColor = currentTheme.sidebar.itemHoverBg;
                                                                }
                                                            }}
                                                            onMouseLeave={(e: any) => {
                                                                if (selectedRoute?.id !== route.id) {
                                                                    e.currentTarget.style.backgroundColor = 'transparent';
                                                                }
                                                            }}
                                                            onClick={() => {
                                                                setSelectedRoute(route);
                                                                setRouteDefinition(route.yaml_definition || JSON.stringify(route, null, 2));
                                                                setActiveTab('editor');
                                                            }}
                                                        >
                                                            <Globe size={14} style={{ color: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemText : (route.enabled ? currentTheme.sidebar.iconEnabled : currentTheme.sidebar.iconDisabled) }} />
                                                            <div className="flex flex-col overflow-hidden">
                                                                <span className="truncate font-medium">{route.name}</span>
                                                                <span className="truncate text-[10px]" style={{ color: selectedRoute?.id === route.id ? 'rgba(255,255,255,0.7)' : currentTheme.sidebar.headerText }}>{route.path}</span>
                                                            </div>
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    deleteRoute(route.id);
                                                                }}
                                                                className={`ml-auto opacity-0 group-hover:opacity-100 p-1 hover:bg-red-500/20 rounded transition-all`}
                                                                style={{ color: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemText : '#e06c75' }}
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
                            className={`absolute right-0 top-0 bottom-0 w-1 cursor-col-resize transition-colors z-10`}
                            style={{ backgroundColor: isDragging ? currentTheme.activityBar.indicator : 'transparent' }}
                            onMouseEnter={(e: any) => { if (!isDragging) e.currentTarget.style.backgroundColor = `${currentTheme.activityBar.indicator}50` }}
                            onMouseLeave={(e: any) => { if (!isDragging) e.currentTarget.style.backgroundColor = 'transparent' }}
                        />
                    </div>
                )}

                {/* 3. Main Editor Area */}
                <div className="flex-1 flex flex-col overflow-hidden m-1.5 rounded-[var(--radius)] border shadow-lg"
                    style={{ backgroundColor: currentTheme.editor.bg, borderColor: currentTheme.editor.border }}>
                    {/* Tabs / Breadcrumbs */}
                    <div className="h-9 flex items-center overflow-x-auto no-scrollbar border-b"
                        style={{ backgroundColor: currentTheme.editor.tabBg, borderColor: currentTheme.editor.border }}>
                        <button
                            onClick={() => setActiveTab('editor')}
                            className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative`}
                            style={{
                                backgroundColor: activeTab === 'editor' ? currentTheme.editor.tabActiveBg : 'transparent',
                                color: activeTab === 'editor' ? currentTheme.editor.tabActiveText : currentTheme.editor.tabInactiveText
                            }}
                            onMouseEnter={(e: any) => {
                                if (activeTab !== 'editor') {
                                    e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg;
                                }
                            }}
                            onMouseLeave={(e: any) => {
                                if (activeTab !== 'editor') {
                                    e.currentTarget.style.backgroundColor = 'transparent';
                                }
                            }}
                        >
                            <Braces size={14} style={{ color: currentTheme.activityBar.activeIcon, opacity: 0.7 }} />
                            <span>{selectedRoute ? `${selectedRoute.name}.yaml` : 'new-route.yaml'}</span>
                            {activeTab === 'editor' && (
                                <div className="absolute top-0 inset-x-0 h-[2px]" style={{ backgroundColor: currentTheme.editor.tabIndicator }} />
                            )}
                        </button>
                        <button
                            onClick={() => setActiveTab('test')}
                            className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative border-l`}
                            style={{
                                backgroundColor: activeTab === 'test' ? currentTheme.editor.tabActiveBg : 'transparent',
                                color: activeTab === 'test' ? currentTheme.editor.tabActiveText : currentTheme.editor.tabInactiveText,
                                borderColor: currentTheme.editor.border
                            }}
                            onMouseEnter={(e: any) => {
                                if (activeTab !== 'test') {
                                    e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg;
                                }
                            }}
                            onMouseLeave={(e: any) => {
                                if (activeTab !== 'test') {
                                    e.currentTarget.style.backgroundColor = 'transparent';
                                }
                            }}
                        >
                            <Terminal size={14} style={{ color: currentTheme.sidebar.iconEnabled, opacity: 0.7 }} />
                            <span>Debug Console</span>
                            {activeTab === 'test' && (
                                <div className="absolute top-0 inset-x-0 h-[2px]" style={{ backgroundColor: currentTheme.editor.tabIndicator }} />
                            )}
                        </button>

                        <div className="ml-auto flex items-center gap-2 px-4">
                            {activeTab === 'editor' && (
                                <button
                                    onClick={registerRoute}
                                    disabled={loading || !routeDefinition.trim()}
                                    className="flex items-center gap-1.5 px-3 py-1 text-white text-[11px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                                    style={{
                                        backgroundColor: loading || !routeDefinition.trim() ? currentTheme.sidebar.border : currentTheme.activityBar.indicator,
                                    }}
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
                                    theme={currentTheme.editor.monacoTheme}
                                    value={routeDefinition}
                                    onChange={(val) => setRouteDefinition(val || '')}
                                    options={{
                                        minimap: { enabled: true },
                                        fontSize: 14,
                                        fontFamily: currentFont,
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
                                    <div className="absolute bottom-6 left-6 right-6 bg-red-500/10 backdrop-blur-xl border border-red-500/20 p-5 rounded-2xl shadow-[0_24px_48px_-12px_rgba(0,0,0,0.5)] animate-blur-in flex items-start gap-5 z-20">
                                        <div className="p-2 rounded-xl bg-red-500/20 border border-red-500/20 text-red-500">
                                            <XCircle size={20} />
                                        </div>
                                        <div className="flex-1">
                                            <p className="text-[11px] font-black uppercase tracking-[2px] text-red-500 mb-1 opacity-80">Deployment Interface Error</p>
                                            <p className="text-[13px] font-bold text-red-200/90 leading-relaxed">{registrationError}</p>
                                        </div>
                                        <button
                                            onClick={() => setRegistrationError(null)}
                                            className="hover:bg-white/10 rounded-xl p-2 transition-all text-red-200/50 hover:text-red-200"
                                        >
                                            <X size={18} />
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
                                                    theme={currentTheme.editor.monacoTheme}
                                                    value={testPayload}
                                                    onChange={(val) => setTestPayload(val || '')}
                                                    options={{
                                                        minimap: { enabled: false },
                                                        fontSize: 13,
                                                        fontFamily: currentFont,
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
                                                    <pre className={`custom-scrollbar ${testResult.success ? 'text-[#59a869]' : 'text-[#e06c75]'}`} style={{ fontFamily: currentFont }}>
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
            <div className="h-6 flex items-center justify-between text-[11px] font-medium shadow-lg z-20 select-none border-t"
                style={{ backgroundColor: currentTheme.statusBar.bg, color: currentTheme.statusBar.text, borderColor: currentTheme.statusBar.border }}>
                <div className="flex items-center h-full">
                    <div className="flex items-center gap-2 px-3 h-full cursor-pointer transition-colors border-r"
                        style={{ borderColor: currentTheme.statusBar.border }}
                        onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.statusBar.hoverBg}
                        onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}>
                        <GitBranch size={12} className="opacity-90" />
                        <span className="translate-y-[0.5px]">main*</span>
                    </div>
                    <div className="flex items-center gap-2 px-3 h-full cursor-pointer transition-colors"
                        onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.statusBar.hoverBg}
                        onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}>
                        <RefreshCw size={12} className={`${loading ? "animate-spin" : ""} opacity-90`} />
                        <span className="translate-y-[0.5px]">{routes.length} Active Endpoints</span>
                    </div>
                </div>
                <div className="flex items-center h-full">
                    <div className="flex items-center gap-1.5 px-3 h-full cursor-pointer transition-colors"
                        onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.statusBar.hoverBg}
                        onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}>
                        <span className="translate-y-[0.5px]">Ln {routeDefinition.split('\n').length}, Col 1</span>
                    </div>
                    <div className="flex items-center px-3 h-full cursor-pointer uppercase transition-colors"
                        onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.statusBar.hoverBg}
                        onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}>
                        <span className="translate-y-[0.5px]">UTF-8</span>
                    </div>
                    <div className="flex items-center gap-2 px-4 h-full cursor-pointer shadow-inner border-l"
                        style={{ backgroundColor: 'rgba(255,255,255,0.15)', borderColor: currentTheme.statusBar.border }}>
                        <Zap size={12} className="fill-white" />
                        <span className="font-bold tracking-tight translate-y-[0.5px]">WORPEN IDE</span>
                    </div>
                </div>
            </div>
        </div>
    );
};
