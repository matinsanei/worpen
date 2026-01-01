import React from 'react';
import Editor from "@monaco-editor/react";
import { Terminal, Play, RefreshCw, FileJson, Activity, Clock } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';

interface TestPanelProps {
    currentTheme: IDETheme;
    currentFont: string;
    selectedRoute: any;
    testPayload: string;
    onTestPayloadChange: (value: string) => void;
    testResult: any;
    loading: boolean;
    onRunTest: () => void;
}

export const TestPanel: React.FC<TestPanelProps> = ({
    currentTheme,
    currentFont,
    selectedRoute,
    testPayload,
    onTestPayloadChange,
    testResult,
    loading,
    onRunTest
}) => {
    return (
        <div className="flex-1 flex flex-col overflow-hidden m-1.5 rounded-[var(--radius)] border shadow-lg"
            style={{ backgroundColor: currentTheme.editor.bg, borderColor: currentTheme.editor.border }}>
            {/* Tabs */}
            <div className="h-9 flex items-center justify-between overflow-x-auto no-scrollbar border-b"
                style={{ backgroundColor: currentTheme.editor.tabBg, borderColor: currentTheme.editor.border }}>
                <div className={`flex items-center gap-2 px-4 h-full text-[12px] font-medium transition-all relative`}
                    style={{
                        backgroundColor: currentTheme.editor.tabActiveBg,
                        color: currentTheme.editor.tabActiveText
                    }}>
                    <Terminal size={14} style={{ color: currentTheme.sidebar.iconEnabled, opacity: 0.7 }} />
                    <span>Debug Console</span>
                    <div className="absolute top-0 inset-x-0 h-[2px]" style={{ backgroundColor: currentTheme.editor.tabIndicator }} />
                </div>
            </div>

            {/* Test Content */}
            <div className="flex flex-col h-full bg-[#1e1f22]">
                <div className="flex-1 p-4 flex flex-col gap-4 overflow-y-auto custom-scrollbar">
                    <div className="flex items-center justify-between border-b border-[#43454a] pb-3">
                        <div className="flex items-center gap-2">
                            <Terminal size={18} className="text-[#3574f0]" />
                            <h3 className="text-[12px] font-bold text-[#dfe1e5] uppercase tracking-wider">Test Environment</h3>
                        </div>
                        <button
                            onClick={onRunTest}
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
                                    onChange={(val) => onTestPayloadChange(val || '')}
                                    options={{
                                        minimap: { enabled: false },
                                        fontSize: 13,
                                        fontFamily: currentFont,
                                        fontLigatures: false,
                                        letterSpacing: 0,
                                        lineHeight: 20,
                                        fontWeight: 'normal',
                                        lineNumbers: 'on',
                                        scrollBeyondLastLine: false,
                                        automaticLayout: true,
                                        padding: { top: 12 },
                                        wordWrap: 'off',
                                        smoothScrolling: false,
                                        cursorBlinking: 'solid',
                                        cursorSmoothCaretAnimation: 'off',
                                        disableLayerHinting: true,
                                        renderWhitespace: 'selection',
                                        stopRenderingLineAfter: -1
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
        </div>
    );
};
