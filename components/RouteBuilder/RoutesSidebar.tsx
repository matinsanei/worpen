import React from 'react';
import { ChevronDown, ChevronRight, Braces, Globe, Plus, Trash2 } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';
import { ROUTE_TEMPLATES } from './constants';

interface RoutesSidebarProps {
    currentTheme: IDETheme;
    routes: any[];
    selectedRoute: any;
    expandedFolders: string[];
    onToggleFolder: (folder: string) => void;
    onLoadTemplate: (key: string) => void;
    onSelectRoute: (route: any) => void;
    onDeleteRoute: (routeId: string) => void;
}

export const RoutesSidebar: React.FC<RoutesSidebarProps> = ({
    currentTheme,
    routes,
    selectedRoute,
    expandedFolders,
    onToggleFolder,
    onLoadTemplate,
    onSelectRoute,
    onDeleteRoute
}) => {
    return (
        <div className="flex-1 overflow-y-auto custom-scrollbar flex flex-col py-2">
            {/* Templates Section */}
            <button
                onClick={() => onToggleFolder('templates')}
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
                            onClick={() => onLoadTemplate(key)}
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
                    onClick={() => onToggleFolder('active-routes')}
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
                                    onClick={() => onSelectRoute(route)}
                                >
                                    <Globe size={14} style={{ color: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemText : (route.enabled ? currentTheme.sidebar.iconEnabled : currentTheme.sidebar.iconDisabled) }} />
                                    <div className="flex flex-col overflow-hidden">
                                        <span className="truncate font-medium">{route.name}</span>
                                        <span className="truncate text-[10px]" style={{ color: selectedRoute?.id === route.id ? 'rgba(255,255,255,0.7)' : currentTheme.sidebar.headerText }}>{route.path}</span>
                                    </div>
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            onDeleteRoute(route.id);
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
        </div>
    );
};
