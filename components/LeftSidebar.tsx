
import React from 'react';
import {
    LayoutGrid, Server, AlertTriangle, Settings,
    Box, Bot, Layers, GitBranch, Hexagon
} from 'lucide-react';
import { ViewState } from '../types';

interface LeftSidebarProps {
    isOpen: boolean;
    currentView: ViewState;
    onNavigate: (view: ViewState) => void;
}

export const LeftSidebar: React.FC<LeftSidebarProps> = ({ isOpen, currentView, onNavigate }) => {

    const NavItem = ({ view, icon: Icon, label, alert }: { view: ViewState, icon: any, label: string, alert?: boolean }) => {
        const active = currentView === view;

        return (
            <button
                onClick={() => onNavigate(view)}
                className={`
          w-full flex items-center px-3 py-2 rounded-md transition-all duration-300 group relative overflow-hidden
          ${active ? 'bg-green-500/10 text-green-400' : 'text-gray-400 hover:text-white hover:bg-white/5'}
        `}
                title={!isOpen ? label : ''}
            >
                {/* Active Indicator Line */}
                <div className={`absolute left-0 top-0 bottom-0 w-[3px] bg-green-500 transition-all duration-300 ${active ? 'opacity-100' : 'opacity-0'}`}></div>

                {/* Icon - Fixed Width */}
                <div className="min-w-[20px] flex justify-center z-10">
                    <Icon size={18} className={`transition-colors duration-300 ${active ? 'text-green-400' : 'text-gray-500 group-hover:text-white'}`} />
                </div>

                {/* Label - Animated */}
                <div className={`
            flex items-center ml-3 whitespace-nowrap overflow-hidden transition-all duration-500 ease-[cubic-bezier(0.25,1,0.5,1)] origin-left
            ${isOpen ? 'w-40 opacity-100 translate-x-0' : 'w-0 opacity-0 -translate-x-2'}
        `}>
                    <span className="text-sm font-medium tracking-wide flex-1 text-left relative z-10">{label}</span>

                    {/* Alert Dot */}
                    {alert && (
                        <div className={`ml-auto pl-2 transition-opacity duration-300 ${isOpen ? 'opacity-100' : 'opacity-0'}`}>
                            <span className="block w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse shadow-[0_0_5px_rgba(239,68,68,0.5)]"></span>
                        </div>
                    )}
                </div>
            </button>
        );
    };

    const NavGroup = ({ label, children }: { label: string, children?: React.ReactNode }) => (
        <div className="mb-6">
            <div className={`px-3 mb-2 overflow-hidden transition-all duration-500 ${isOpen ? 'opacity-100 h-auto' : 'opacity-0 h-0'}`}>
                <h3 className="text-[10px] font-mono text-gray-600 uppercase tracking-widest flex items-center gap-2 whitespace-nowrap">
                    <span className="w-1 h-1 bg-gray-700 rounded-full"></span>
                    {label}
                </h3>
            </div>
            {/* Separator for collapsed state */}
            {!isOpen && <div className="h-[1px] bg-white/5 mx-4 mb-3"></div>}

            <div className="space-y-0.5">
                {children}
            </div>
        </div>
    );

    return (
        <aside
            className={`
        flex flex-col border-r border-white/5 bg-[#050505] transition-[width] duration-300 ease-[cubic-bezier(0.25,1,0.5,1)] relative z-20 flex-shrink-0 group/sidebar overflow-hidden
        ${isOpen ? 'w-64' : 'w-20'}
      `}
        >
            {/* AKA 'Namaz' (Strip/Spine) - The glowing green spine */}
            <div className={`
        absolute left-0 top-0 bottom-0 w-[2px] bg-gradient-to-b from-transparent via-green-500 to-transparent z-50
        transition-all duration-500
        ${isOpen ? 'opacity-20 blur-[1px]' : 'opacity-100 blur-[2px] shadow-[0_0_15px_rgba(0,255,65,0.6)]'}
      `}></div>

            {/* Background Matrix/Noise when collapsed */}
            {!isOpen && (
                <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-10 mix-blend-overlay pointer-events-none"></div>
            )}

            {/* Brand */}
            <div className="h-16 flex items-center px-6 border-b border-white/5 relative overflow-hidden flex-shrink-0">
                {/* Glow */}
                <div className="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-green-500/50 to-transparent"></div>

                <div className="flex items-center gap-3 text-green-500 relative z-10 w-full">
                    <div className="relative min-w-[28px]">
                        <Hexagon size={28} strokeWidth={1.5} className={`fill-green-500/10 text-green-500 transition-all duration-500 ${!isOpen && 'animate-pulse drop-shadow-[0_0_8px_rgba(0,255,65,0.5)]'}`} />
                        <div className="absolute inset-0 flex items-center justify-center">
                            <div className="w-1 h-1 bg-green-400 rounded-full animate-ping"></div>
                        </div>
                    </div>

                    <div className={`flex flex-col overflow-hidden whitespace-nowrap transition-all duration-300 ease-out ${isOpen ? 'opacity-100 translate-x-0 w-32' : 'opacity-0 -translate-x-4 w-0'}`}>
                        <span className="font-bold text-lg tracking-tight text-white leading-none font-display">WORPEN</span>
                        <span className="text-[9px] text-gray-500 font-mono tracking-widest">OS v2.4 // ENT</span>
                    </div>
                </div>
            </div>

            {/* Navigation */}
            <div className="flex-1 overflow-y-auto py-6 px-3 custom-scrollbar flex-col overflow-x-hidden">
                <NavGroup label="Overview">
                    <NavItem view="DASHBOARD" icon={LayoutGrid} label="Dashboard" />
                    <NavItem view="FLEET" icon={Server} label="Fleet Nodes" />
                </NavGroup>

                <NavGroup label="Orchestration">
                    <NavItem view="DOCKER" icon={Box} label="Containers" />
                    <NavItem view="AUTOMATION" icon={Bot} label="Auto-Healing" />
                    <NavItem view="DEPENDENCY" icon={Layers} label="Artifacts" />
                    <NavItem view="CICD" icon={GitBranch} label="Pipeline" />
                </NavGroup>

                <NavGroup label="System">
                    <NavItem view="INCIDENTS" icon={AlertTriangle} label="Incidents" alert={true} />
                    <NavItem view="SETTINGS" icon={Settings} label="Settings" />
                </NavGroup>
            </div>

            {/* User Footer */}
            <div className="p-3 border-t border-white/5 bg-[#080808]">
                <button className={`w-full flex items-center p-2 rounded-md hover:bg-white/5 transition-colors group relative overflow-hidden ${isOpen ? 'gap-3' : 'justify-center'}`}>
                    <div className="relative min-w-[32px]">
                        <div className="w-8 h-8 rounded bg-gradient-to-br from-green-900 via-black to-black border border-green-800 flex items-center justify-center text-green-500 font-bold text-xs group-hover:border-green-500 transition-colors">
                            OP
                        </div>
                        <div className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 bg-black rounded-full flex items-center justify-center">
                            <div className="w-1.5 h-1.5 bg-green-500 rounded-full"></div>
                        </div>
                    </div>

                    <div className={`text-left overflow-hidden whitespace-nowrap transition-all duration-300 ease-out ${isOpen ? 'w-32 opacity-100 translate-x-0' : 'w-0 opacity-0 -translate-x-2'}`}>
                        <div className="text-xs font-medium text-white truncate group-hover:text-green-400 transition-colors">Operator_Root</div>
                        <div className="text-[10px] text-gray-500 truncate font-mono">Sec_Level: 5</div>
                    </div>
                </button>
            </div>
        </aside>
    );
};
