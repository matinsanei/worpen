import React from 'react';
import Editor from "@monaco-editor/react";
import { Braces, Save, RefreshCw, Sparkles, MessageSquare } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';
import { ErrorDisplay } from './ErrorDisplay';
import { SuccessDisplay } from './SuccessDisplay';

interface EditorPanelProps {
    currentTheme: IDETheme;
    currentFont: string;
    selectedRoute: any;
    routeDefinition: string;
    onRouteDefinitionChange: (value: string) => void;
    registrationError: any;
    successMessage: string | null;
    onClearError: () => void;
    onClearSuccess: () => void;
    loading: boolean;
    onDeploy: () => void;
    onOpenAI?: () => void;
    onOpenCopilot?: () => void;
}

export const EditorPanel: React.FC<EditorPanelProps> = ({
    currentTheme,
    currentFont,
    selectedRoute,
    routeDefinition,
    onRouteDefinitionChange,
    registrationError,
    successMessage,
    onClearError,
    onClearSuccess,
    loading,
    onDeploy,
    onOpenAI,
    onOpenCopilot
}) => {
    return (
        <div className="flex-1 flex flex-col overflow-hidden m-1.5 rounded-[var(--radius)] border shadow-lg"
            style={{ backgroundColor: currentTheme.editor.bg, borderColor: currentTheme.editor.border }}>
            {/* Tabs / Breadcrumbs */}
            <div className="h-9 flex items-center justify-between overflow-x-auto no-scrollbar border-b"
                style={{ backgroundColor: currentTheme.editor.tabBg, borderColor: currentTheme.editor.border }}>
                <div className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative`}
                    style={{
                        backgroundColor: currentTheme.editor.tabActiveBg,
                        color: currentTheme.editor.tabActiveText
                    }}>
                    <Braces size={14} style={{ color: currentTheme.activityBar.activeIcon, opacity: 0.7 }} />
                    <span>{selectedRoute ? `${selectedRoute.name}.yaml` : 'new-route.yaml'}</span>
                    <div className="absolute top-0 inset-x-0 h-[2px]" style={{ backgroundColor: currentTheme.editor.tabIndicator }} />
                </div>

                <div className="flex items-center gap-2 px-4">
                    {onOpenCopilot && (
                        <button
                            onClick={onOpenCopilot}
                            className="flex items-center gap-1.5 px-3 py-1 bg-[#c678dd]/20 hover:bg-[#c678dd]/30 border border-[#c678dd]/30 text-[#c678dd] text-[11px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                        >
                            <MessageSquare size={12} />
                            Copilot
                        </button>
                    )}
                    {onOpenAI && (
                        <button
                            onClick={onOpenAI}
                            className="flex items-center gap-1.5 px-3 py-1 bg-gradient-to-r from-[#3574f0] to-[#c678dd] hover:from-[#2d5fc7] hover:to-[#a85ec4] text-white text-[11px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                        >
                            <Sparkles size={12} />
                            AI
                        </button>
                    )}
                    <button
                        onClick={onDeploy}
                        disabled={loading || !routeDefinition.trim()}
                        className="flex items-center gap-1.5 px-3 py-1 text-white text-[11px] font-bold rounded-[var(--radius)] transition-all shadow-md active:scale-95"
                        style={{
                            backgroundColor: loading || !routeDefinition.trim() ? currentTheme.sidebar.border : currentTheme.activityBar.indicator,
                        }}
                    >
                        {loading ? <RefreshCw size={12} className="animate-spin" /> : <Save size={12} />}
                        DEPLOY
                    </button>
                </div>
            </div>

            {/* Editor Content */}
            <div className="flex-1 flex flex-col relative overflow-hidden bg-[#1e1f22]">
                <div className="h-full w-full">
                    <Editor
                        height="100%"
                        defaultLanguage="yaml"
                        theme={currentTheme.editor.monacoTheme}
                        value={routeDefinition}
                        onChange={(val) => onRouteDefinitionChange(val || '')}
                        options={{
                            minimap: { enabled: true },
                            fontSize: 14,
                            fontFamily: currentFont,
                            fontLigatures: false,
                            letterSpacing: 0,
                            lineHeight: 21,
                            fontWeight: 'normal',
                            lineNumbers: 'on',
                            roundedSelection: true,
                            scrollBeyondLastLine: false,
                            readOnly: false,
                            automaticLayout: true,
                            padding: { top: 16 },
                            wordWrap: 'off',
                            wrappingStrategy: 'advanced',
                            smoothScrolling: false,
                            cursorBlinking: 'solid',
                            cursorSmoothCaretAnimation: 'off',
                            disableLayerHinting: true,
                            renderWhitespace: 'selection',
                            stopRenderingLineAfter: -1
                        }}
                    />
                    <ErrorDisplay error={registrationError} onClose={onClearError} />
                    {successMessage && (
                        <SuccessDisplay message={successMessage} onClose={onClearSuccess} />
                    )}
                </div>
            </div>
        </div>
    );
};
