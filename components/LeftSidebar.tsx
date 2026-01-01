
import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
    Activity, Database, AlertCircle, Settings,
    Network, Code2, List, Hexagon
} from 'lucide-react';
import { ViewState } from '../types';

interface LeftSidebarProps {
    isOpen: boolean;
    currentView: ViewState;
    onViewChange: (view: ViewState) => void;
}

export const LeftSidebar: React.FC<LeftSidebarProps> = ({ isOpen, currentView, onViewChange }) => {

    const viewToPath: Record<ViewState, string> = {
        'DASHBOARD': '/',
        'DYNAMIC_ROUTES': '/routes',
        'FUNCTIONS': '/functions',
        'STORAGE': '/storage',
        'CICD': '/traces',
        'INCIDENTS': '/errors',
        'SETTINGS': '/settings',
        // Legacy paths (hidden from UI but kept for compatibility)
        'FLEET': '/fleet',
        'DOCKER': '/containers',
        'AUTOMATION': '/auto-healing',
        'DEPENDENCY': '/artifacts',
    };

    const NavItem = ({ view, icon: Icon, label, alert }: { view: ViewState, icon: any, label: string, alert?: boolean }) => {
        const active = currentView === view;
        const path = viewToPath[view];

        return (
            <Link
                to={path}
                onClick={() => onViewChange(view)}
                className={`
          w-full flex items-center my-0.5 rounded-[var(--radius)] transition-all duration-200 group relative py-1.5
          ${isOpen ? 'px-2' : 'justify-center'}
          ${active ? 'bg-[#3574f020] text-[#3574f0]' : 'text-[#dfe1e5] hover:bg-[#393b40]'}
        `}
                title={!isOpen ? label : ''}
            >
                {/* Active Indicator - Fixed position for collapsed/expanded */}
                {active && (
                    <div className={`absolute left-0 top-1/2 -translate-y-1/2 w-1 h-5 bg-[#3574f0] rounded-r-full shadow-[0_0_8px_rgba(53,116,240,0.3)] ${isOpen ? 'translate-x-[-12px]' : 'translate-x-[-6px]'}`}></div>
                )}

                {/* Icon */}
                <div className="flex justify-center items-center z-10 flex-shrink-0">
                    <Icon size={18} className={`transition-colors duration-200 ${active ? 'text-[#3574f0]' : 'text-[#6e7073] group-hover:text-[#dfe1e5]'}`} strokeWidth={active ? 2.5 : 2} />
                </div>

                {/* Label */}
                <div className={`
            flex items-center whitespace-nowrap overflow-hidden transition-all duration-300 ease-out
            ${isOpen ? 'w-40 opacity-100 translate-x-0 ml-3' : 'w-0 opacity-0 -translate-x-2'}
        `}>
                    <span className={`text-[13px] ${active ? 'font-semibold' : 'font-medium'} flex-1 text-left`}>{label}</span>

                    {/* Alert Dot */}
                    {alert && (
                        <div className="ml-auto pl-2">
                            <span className="block w-2 h-2 rounded-full bg-[#e06c75] shadow-[0_0_4px_rgba(224,108,117,0.4)]"></span>
                        </div>
                    )}
                </div>
            </Link>
        );
    };

    const NavGroup = ({ label, children }: { label: string, children?: React.ReactNode }) => (
        <div className="mb-4">
            <div className={`px-2 mb-1.5 overflow-hidden transition-opacity duration-300 ${isOpen ? 'opacity-100 h-6' : 'opacity-0 h-0'}`}>
                <h3 className="text-[11px] font-bold text-[#6e7073] uppercase tracking-wider flex items-center gap-2">
                    {label}
                </h3>
            </div>
            <div className="flex flex-col">
                {children}
            </div>
        </div>
    );

    return (
        <aside
            className={`
        flex flex-col border-r border-[#43454a] bg-[#2b2d30] transition-all duration-300 ease-in-out relative z-20 flex-shrink-0 group/sidebar
        ${isOpen ? 'w-64' : 'w-12'}
      `}
        >
            {/* Brand - Integrated with sidebar */}
            <div className={`h-12 flex items-center border-b border-[#43454a] relative flex-shrink-0 group/brand ${isOpen ? 'px-4' : 'justify-center'}`}>
                <div className="flex items-center relative z-10 overflow-hidden">
                    {/* Logo - Rounded Square */}
                    <div className="relative w-7 h-7 flex items-center justify-center flex-shrink-0 rounded-[var(--radius)] bg-[#3574f0] shadow-lg shadow-[#3574f020]">
                        <Hexagon size={16} className="text-white" strokeWidth={2.5} />
                    </div>

                    {/* Typography */}
                    <div className={`flex flex-col whitespace-nowrap transition-all duration-300 ease-out overflow-hidden ${isOpen ? 'opacity-100 translate-x-0 w-32 ml-3' : 'opacity-0 -translate-x-2 w-0 ml-0'}`}>
                        <span className="font-bold text-[14px] text-[#dfe1e5] tracking-tight">Worpen</span>
                    </div>
                </div>
            </div>

            {/* Navigation */}
            <div className={`flex-1 overflow-y-auto py-4 custom-scrollbar flex flex-col overflow-x-hidden ${isOpen ? 'px-3' : 'px-1.5'}`}>
                <NavGroup label="CORE">
                    <NavItem view="DASHBOARD" icon={Activity} label="Overview" />
                    <NavItem view="DYNAMIC_ROUTES" icon={Network} label="API Routes" />
                    <NavItem view="FUNCTIONS" icon={Code2} label="Functions Lib" />
                    <NavItem view="STORAGE" icon={Database} label="Storage" />
                </NavGroup>

                <NavGroup label="OBSERVABILITY">
                    <NavItem view="CICD" icon={List} label="Live Traces" />
                    <NavItem view="INCIDENTS" icon={AlertCircle} label="Error Logs" alert={true} />
                </NavGroup>

                <NavGroup label="SETTINGS">
                    <NavItem view="SETTINGS" icon={Settings} label="Settings" />
                </NavGroup>
            </div>

            {/* User Footer */}
            <div className="p-2 border-t border-[#43454a] bg-[#2b2d30]">
                <button className={`w-full flex items-center rounded-[var(--radius)] hover:bg-[#393b40] transition-colors group relative overflow-hidden ${isOpen ? 'p-2 gap-3' : 'justify-center p-1.5'}`}>
                    <div className="relative min-w-[28px]">
                        <div className="w-7 h-7 rounded-[var(--radius)] bg-[#4e5157] border border-[#5a5d63] flex items-center justify-center text-white font-bold text-[10px] group-hover:border-[#3574f0] transition-colors">
                            OP
                        </div>
                    </div>

                    <div className={`text-left overflow-hidden whitespace-nowrap transition-opacity duration-300 ${isOpen ? 'w-auto opacity-100' : 'w-0 opacity-0'}`}>
                        <div className="text-[12px] font-semibold text-[#dfe1e5] truncate transition-colors">Operator_Root</div>
                        <div className="text-[10px] text-[#6e7073] truncate font-mono">Sec_Level: 5</div>
                    </div>
                </button>
            </div>
        </aside>
    );
};
