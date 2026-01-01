import React, { useState, useEffect, useRef, useCallback } from 'react';
import { loader } from "@monaco-editor/react";
import { getApiBaseUrl } from '../api';
import { ALL_THEMES, IDETheme, worpenDark } from '../src/themes/ide';
import { ROUTE_TEMPLATES, AVAILABLE_FONTS } from '../components/RouteBuilder/constants';
import { ActivityBar } from '../components/RouteBuilder/ActivityBar';
import { RoutesSidebar } from '../components/RouteBuilder/RoutesSidebar';
import { SettingsModal } from '../components/RouteBuilder/SettingsModal';
import { EditorPanel } from '../components/RouteBuilder/EditorPanel';
import { TestPanel } from '../components/RouteBuilder/TestPanel';
import { AIGeneratorModal } from '../components/AIGeneratorModal';
import { CopilotSidebar } from '../components/CopilotSidebar';

export const DynamicRoutesView: React.FC = () => {
    // State Management
    const [routes, setRoutes] = useState<any[]>([]);
    const [selectedRoute, setSelectedRoute] = useState<any>(null);
    const [routeDefinition, setRouteDefinition] = useState('');
    const [testPayload, setTestPayload] = useState('{}');
    const [testResult, setTestResult] = useState<any>(null);
    const [loading, setLoading] = useState(false);
    const [activeTab, setActiveTab] = useState<'editor' | 'test'>('editor');
    const [registrationError, setRegistrationError] = useState<any>(null);
    
    // UI State
    const [sidebarWidth, setSidebarWidth] = useState(() => {
        const saved = localStorage.getItem('worpen_builder_sidebar_width');
        return saved !== null ? parseFloat(saved) : 33;
    });
    const [sidebarVisible, setSidebarVisible] = useState(() => {
        const saved = localStorage.getItem('worpen_builder_sidebar_visible');
        return saved !== null ? JSON.parse(saved) : true;
    });
    const [activeActivityId, setActiveActivityId] = useState<'explorer' | 'search' | 'git' | 'debug'>('explorer');
    const [expandedFolders, setExpandedFolders] = useState<string[]>(['templates', 'active-routes']);
    
    // Use ref for dragging state to avoid re-render issues
    const isDraggingRef = useRef(false);
    const containerRef = useRef<HTMLDivElement>(null);
    const dragStartXRef = useRef(0);
    const dragStartWidthRef = useRef(0);
    
    // Theme & Settings
    const [currentTheme, setCurrentTheme] = useState<IDETheme>(() => {
        const saved = localStorage.getItem('worpen_ide_theme');
        if (saved) {
            const theme = ALL_THEMES.find(t => t.id === saved);
            if (theme) return theme;
        }
        return worpenDark;
    });
    const [currentFont, setCurrentFont] = useState(() => {
        return localStorage.getItem('worpen_ide_font') || '"JetBrains Mono", monospace';
    });
    const [showSettingsModal, setShowSettingsModal] = useState(false);
    const [showAIModal, setShowAIModal] = useState(false);
    const [showCopilot, setShowCopilot] = useState(false);

    // AI Generation Handler
    const handleAIGenerated = (code: string) => {
        try {
            // Parse the JSON code
            const parsed = JSON.parse(code);
            
            // Convert to YAML-like format for route definition
            const yamlRoute = `route:
  path: "/api/generated"
  method: POST
  logic:
${code.split('\n').map(line => '    ' + line).join('\n')}`;
            
            setRouteDefinition(yamlRoute);
            setRegistrationError({ 
                error: 'Success', 
                details: 'AI generated logic inserted! Update the path and method, then deploy.' 
            });
        } catch (err) {
            // If not valid JSON, just insert as-is
            setRouteDefinition(code);
        }
    };

    // Copilot Apply Handler
    const handleCopilotApply = (code: string) => {
        // Try to intelligently merge or replace code
        const trimmedCode = code.trim();
        
        // If it's a JSON array, try to parse and merge with existing logic
        if (trimmedCode.startsWith('[') && trimmedCode.endsWith(']')) {
            try {
                const parsed = JSON.parse(trimmedCode);
                const yamlLogic = `route:
  path: "${selectedRoute?.path || '/api/generated'}"
  method: ${selectedRoute?.method || 'POST'}
  logic:
${trimmedCode.split('\n').map(line => '    ' + line).join('\n')}`;
                setRouteDefinition(yamlLogic);
            } catch {
                // If parsing fails, just append
                setRouteDefinition(routeDefinition + '\n' + trimmedCode);
            }
        } else {
            // Otherwise, replace entire content or append based on format
            if (trimmedCode.includes('route:') || trimmedCode.includes('logic:')) {
                setRouteDefinition(trimmedCode);
            } else {
                // Append to existing
                setRouteDefinition(routeDefinition + '\n' + trimmedCode);
            }
        }
        
        setRegistrationError({ 
            error: 'Success', 
            details: 'Code applied from Copilot! Review and deploy when ready.' 
        });
    };

    // Effects
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

    // Resize handlers using useCallback to prevent re-creation
    const handleMouseMove = useCallback((e: MouseEvent) => {
        if (!isDraggingRef.current || !sidebarVisible) return;
        
        e.preventDefault(); // Prevent text selection during drag
        
        const container = containerRef.current;
        if (!container) return;
        
        const containerRect = container.getBoundingClientRect();
        
        // Calculate delta from initial drag position
        const deltaX = e.clientX - dragStartXRef.current;
        const deltaPercent = (deltaX / containerRect.width) * 100;
        
        // Apply delta to initial width
        const newWidth = dragStartWidthRef.current + deltaPercent;
        const clampedWidth = Math.min(Math.max(newWidth, 10), 70);
        
        // Use requestAnimationFrame for smoother updates
        requestAnimationFrame(() => {
            setSidebarWidth(clampedWidth);
        });
    }, [sidebarVisible]);

    const handleMouseUp = useCallback(() => {
        if (isDraggingRef.current) {
            isDraggingRef.current = false;
            document.body.style.cursor = '';
            document.body.style.userSelect = '';
        }
    }, []);

    const handleMouseDown = useCallback((e: React.MouseEvent) => {
        isDraggingRef.current = true;
        dragStartXRef.current = e.clientX;
        dragStartWidthRef.current = sidebarWidth;
        document.body.style.cursor = 'col-resize';
        document.body.style.userSelect = 'none';
    }, [sidebarWidth]);

    useEffect(() => {
        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);

        return () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };
    }, [handleMouseMove, handleMouseUp]);

    // API Functions
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
            const isYAML = !routeDefinition.trim().startsWith('{') && !routeDefinition.trim().startsWith('[');
            const contentType = isYAML ? 'application/x-yaml' : 'application/json';

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
                    setRegistrationError(errorData);
                } catch {
                    setRegistrationError({ error: 'Failed to register route', details: errorText });
                }
            }
        } catch (error: any) {
            setRegistrationError({ error: 'Network error', details: error.message || 'Network error occurred' });
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

    // Event Handlers
    const handleActivityChange = (id: 'explorer' | 'search' | 'git' | 'debug') => {
        if (activeActivityId === id) {
            setSidebarVisible(!sidebarVisible);
        } else {
            setActiveActivityId(id);
            setSidebarVisible(true);
        }
    };

    const handleToggleFolder = (folder: string) => {
        setExpandedFolders(prev =>
            prev.includes(folder) ? prev.filter(f => f !== folder) : [...prev, folder]
        );
    };

    const handleLoadTemplate = (key: string) => {
        const template = ROUTE_TEMPLATES[key as keyof typeof ROUTE_TEMPLATES];
        setRouteDefinition(template.json);
        setRegistrationError(null);
    };

    const handleSelectRoute = (route: any) => {
        setSelectedRoute(route);
        setRouteDefinition(route.yaml_definition || JSON.stringify(route, null, 2));
        setActiveTab('editor');
    };

    return (
        <div className="flex flex-col h-full w-full bg-[#1e1f22] text-[#dfe1e5] font-sans selection:bg-[#3574f030] overflow-hidden" 
            style={{ backgroundColor: currentTheme.editor.bg }}>
            
            <div ref={containerRef} id="routes-container" className="flex flex-1 overflow-hidden">
                {/* Activity Bar */}
                <ActivityBar
                    currentTheme={currentTheme}
                    activeActivityId={activeActivityId}
                    sidebarVisible={sidebarVisible}
                    onActivityChange={handleActivityChange}
                    onSettingsClick={() => setShowSettingsModal(true)}
                />

                {/* Sidebar */}
                {sidebarVisible && (
                    <div
                        className="flex flex-col border-r select-none group/sidebar relative"
                        style={{ 
                            width: `${sidebarWidth}%`, 
                            minWidth: '180px', 
                            backgroundColor: currentTheme.sidebar.bg, 
                            borderColor: currentTheme.sidebar.border 
                        }}
                    >
                        <div className="h-9 px-4 flex items-center justify-between text-[11px] uppercase tracking-wider font-bold border-b"
                            style={{ color: currentTheme.sidebar.headerText, borderColor: currentTheme.sidebar.border }}>
                            <span>{activeActivityId}</span>
                        </div>

                        {activeActivityId === 'explorer' && (
                            <RoutesSidebar
                                currentTheme={currentTheme}
                                routes={routes}
                                selectedRoute={selectedRoute}
                                expandedFolders={expandedFolders}
                                onToggleFolder={handleToggleFolder}
                                onLoadTemplate={handleLoadTemplate}
                                onSelectRoute={handleSelectRoute}
                                onDeleteRoute={deleteRoute}
                            />
                        )}

                        {/* Resize Handle */}
                        <div
                            onMouseDown={handleMouseDown}
                            className={`absolute right-0 top-0 bottom-0 w-1 cursor-col-resize transition-colors z-10`}
                            style={{ backgroundColor: isDraggingRef.current ? currentTheme.activityBar.indicator : 'transparent' }}
                            onMouseEnter={(e: any) => { if (!isDraggingRef.current) e.currentTarget.style.backgroundColor = `${currentTheme.activityBar.indicator}50` }}
                            onMouseLeave={(e: any) => { if (!isDraggingRef.current) e.currentTarget.style.backgroundColor = 'transparent' }}
                        />
                    </div>
                )}

                {/* Main Editor/Test Panel */}
                {activeTab === 'editor' ? (
                    <EditorPanel
                        currentTheme={currentTheme}
                        currentFont={currentFont}
                        selectedRoute={selectedRoute}
                        routeDefinition={routeDefinition}
                        onRouteDefinitionChange={setRouteDefinition}
                        registrationError={registrationError}
                        onClearError={() => setRegistrationError(null)}
                        loading={loading}
                        onDeploy={registerRoute}
                        onOpenAI={() => setShowAIModal(true)}
                        onOpenCopilot={() => setShowCopilot(!showCopilot)}
                    />
                ) : (
                    <TestPanel
                        currentTheme={currentTheme}
                        currentFont={currentFont}
                        selectedRoute={selectedRoute}
                        testPayload={testPayload}
                        onTestPayloadChange={setTestPayload}
                        testResult={testResult}
                        loading={loading}
                        onRunTest={testRoute}
                    />
                )}

                {/* Copilot Sidebar */}
                <CopilotSidebar
                    isOpen={showCopilot}
                    onClose={() => setShowCopilot(false)}
                    currentCode={routeDefinition}
                    codeLanguage="yaml"
                    contextType="route"
                    onApplyCode={handleCopilotApply}
                />
            </div>

            {/* Settings Modal */}
            <SettingsModal
                isOpen={showSettingsModal}
                onClose={() => setShowSettingsModal(false)}
                currentTheme={currentTheme}
                onThemeChange={setCurrentTheme}
                currentFont={currentFont}
                onFontChange={setCurrentFont}
                availableFonts={AVAILABLE_FONTS}
            />

            {/* AI Generator Modal */}
            <AIGeneratorModal
                isOpen={showAIModal}
                onClose={() => setShowAIModal(false)}
                onUseCode={handleAIGenerated}
                type="route"
            />
        </div>
    );
};
