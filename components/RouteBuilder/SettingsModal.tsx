import React from 'react';
import { Settings, Layout, Braces, GitBranch, XCircle } from 'lucide-react';
import { IDETheme, ALL_THEMES } from '../../src/themes/ide';

interface SettingsModalProps {
    isOpen: boolean;
    onClose: () => void;
    currentTheme: IDETheme;
    onThemeChange: (theme: IDETheme) => void;
    currentFont: string;
    onFontChange: (font: string) => void;
    availableFonts: Array<{ name: string; family: string }>;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({
    isOpen,
    onClose,
    currentTheme,
    onThemeChange,
    currentFont,
    onFontChange,
    availableFonts
}) => {
    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-md animate-in fade-in duration-500">
            <div
                className="w-[850px] h-[650px] rounded-2xl shadow-[0_32px_64px_-16px_rgba(0,0,0,0.6)] border flex overflow-hidden animate-blur-in"
                style={{ backgroundColor: currentTheme.sidebar.bg, borderColor: currentTheme.sidebar.border }}
            >
                {/* Modal Sidebar */}
                <div className="w-60 border-r flex flex-col animate-in fade-in duration-700 delay-150"
                    style={{ borderColor: currentTheme.sidebar.border, backgroundColor: currentTheme.activityBar.bg }}>
                    <div className="p-8 border-b" style={{ borderColor: currentTheme.sidebar.border }}>
                        <h2 className="text-xl font-black flex items-center gap-3 tracking-tighter"
                            style={{ color: currentTheme.sidebar.headerText }}>
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
                        <div className="mt-2 px-5 py-3 flex items-center gap-4 text-[14px] font-medium opacity-30 cursor-not-allowed"
                            style={{ color: currentTheme.sidebar.itemText }}>
                            <Braces size={20} /> Editor Config
                        </div>
                        <div className="px-5 py-3 flex items-center gap-4 text-[14px] font-medium opacity-30 cursor-not-allowed"
                            style={{ color: currentTheme.sidebar.itemText }}>
                            <GitBranch size={20} /> Version Control
                        </div>
                    </div>
                    <div className="p-8 text-[11px] font-bold opacity-30 tracking-widest"
                        style={{ color: currentTheme.sidebar.itemText }}>
                        WORPEN CORE v1.0.4.5
                    </div>
                </div>

                {/* Modal Main Content */}
                <div className="flex-1 flex flex-col overflow-hidden animate-in fade-in duration-700 delay-300"
                    style={{ backgroundColor: currentTheme.editor.bg }}>
                    <div className="h-16 border-b flex items-center justify-between px-10"
                        style={{ borderColor: currentTheme.sidebar.border }}>
                        <div className="flex flex-col">
                            <span className="text-xs font-black uppercase tracking-[3px] opacity-40"
                                style={{ color: currentTheme.sidebar.headerText }}>
                                Configuration
                            </span>
                            <span className="text-sm font-bold" style={{ color: currentTheme.sidebar.headerText }}>
                                Interface & Branding
                            </span>
                        </div>
                        <button
                            onClick={onClose}
                            className="hover:scale-110 transition-transform"
                            style={{ color: currentTheme.sidebar.headerText }}
                        >
                            <XCircle size={28} />
                        </button>
                    </div>

                    <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
                        {/* Theme Selection */}
                        <section className="mb-10">
                            <h3 className="text-[12px] font-black uppercase tracking-widest mb-4 opacity-70"
                                style={{ color: currentTheme.activityBar.indicator }}>
                                Color Theme
                            </h3>
                            <div className="grid grid-cols-2 lg:grid-cols-3 gap-3">
                                {ALL_THEMES.map(theme => (
                                    <button
                                        key={theme.id}
                                        onClick={() => onThemeChange(theme)}
                                        className="flex flex-col gap-3 p-4 rounded-lg border-2 transition-all hover:scale-[1.02]"
                                        style={{
                                            backgroundColor: currentTheme.id === theme.id ? theme.activityBar.bg : theme.activityBar.bg + '50',
                                            borderColor: currentTheme.id === theme.id ? currentTheme.activityBar.indicator : 'transparent'
                                        }}
                                    >
                                        <div className="flex items-center justify-between w-full">
                                            <span className="text-[13px] font-bold" style={{ color: theme.sidebar.activeItemText }}>
                                                {theme.name}
                                            </span>
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
                            <h3 className="text-[12px] font-black uppercase tracking-widest mb-4 opacity-70"
                                style={{ color: currentTheme.activityBar.indicator }}>
                                Editor Font
                            </h3>
                            <div className="grid grid-cols-2 gap-3">
                                {availableFonts.map(font => (
                                    <button
                                        key={font.family}
                                        onClick={() => onFontChange(font.family)}
                                        className="p-4 rounded-lg border-2 text-left transition-all hover:scale-[1.02]"
                                        style={{
                                            backgroundColor: currentFont === font.family ? currentTheme.activityBar.bg : currentTheme.activityBar.bg + '50',
                                            borderColor: currentFont === font.family ? currentTheme.activityBar.indicator : 'transparent',
                                            fontFamily: font.family
                                        }}
                                    >
                                        <div className="text-[14px] mb-1" style={{ color: currentTheme.sidebar.activeItemText }}>
                                            {font.name}
                                        </div>
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
    );
};
