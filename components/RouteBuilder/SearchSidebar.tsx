import React, { useState, useMemo } from 'react';
import { Search, X, Globe, Folder } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';
import { RouteFolder } from '../../src/utils/routeFolders';

interface SearchSidebarProps {
    currentTheme: IDETheme;
    routes: any[];
    folders: RouteFolder[];
    onSelectRoute: (route: any) => void;
    selectedRoute: any;
}

export const SearchSidebar: React.FC<SearchSidebarProps> = ({
    currentTheme,
    routes,
    folders,
    onSelectRoute,
    selectedRoute
}) => {
    const [searchQuery, setSearchQuery] = useState('');

    // Search logic
    const searchResults = useMemo(() => {
        if (!searchQuery.trim()) return [];

        const query = searchQuery.toLowerCase();
        const results: Array<{
            route: any;
            folder?: RouteFolder;
            matches: string[];
        }> = [];

        routes.forEach(route => {
            const matches: string[] = [];

            // Search in route name
            if (route.name?.toLowerCase().includes(query)) {
                matches.push('name');
            }

            // Search in route path
            if (route.path?.toLowerCase().includes(query)) {
                matches.push('path');
            }

            // Search in route method
            if (route.method?.toLowerCase().includes(query)) {
                matches.push('method');
            }

            // Search in route description
            if (route.description?.toLowerCase().includes(query)) {
                matches.push('description');
            }

            // Find folder containing this route
            const folder = folders.find(f => f.routeIds.includes(route.id));

            if (matches.length > 0) {
                results.push({ route, folder, matches });
            }
        });

        return results;
    }, [searchQuery, routes, folders]);

    const highlightText = (text: string, query: string) => {
        if (!query.trim()) return text;
        
        const parts = text.split(new RegExp(`(${query})`, 'gi'));
        return parts.map((part, i) => 
            part.toLowerCase() === query.toLowerCase() ? (
                <span key={i} className="bg-yellow-500/30 font-semibold">{part}</span>
            ) : (
                part
            )
        );
    };

    return (
        <div className="flex-1 flex flex-col overflow-hidden">
            {/* Search Input */}
            <div className="p-2">
                <div 
                    className="flex items-center gap-2 px-3 py-2 rounded-md"
                    style={{ 
                        backgroundColor: currentTheme.editor.bg,
                        border: `1px solid ${currentTheme.sidebar.border}`
                    }}
                >
                    <Search size={14} style={{ color: currentTheme.sidebar.iconDisabled }} />
                    <input
                        type="text"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        placeholder="Search routes..."
                        className="flex-1 bg-transparent outline-none text-[12px]"
                        style={{ color: currentTheme.editor.tabActiveText }}
                        autoFocus
                    />
                    {searchQuery && (
                        <button
                            onClick={() => setSearchQuery('')}
                            className="p-1 hover:bg-black/20 rounded transition-colors"
                            style={{ color: currentTheme.sidebar.iconEnabled }}
                        >
                            <X size={12} />
                        </button>
                    )}
                </div>
            </div>

            {/* Results */}
            <div className="flex-1 overflow-y-auto custom-scrollbar px-2">
                {!searchQuery.trim() ? (
                    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
                        <Search size={32} style={{ color: currentTheme.sidebar.iconDisabled, opacity: 0.5 }} className="mb-3" />
                        <p className="text-[11px]" style={{ color: currentTheme.sidebar.iconDisabled }}>
                            Type to search routes by name, path, method, or description
                        </p>
                    </div>
                ) : searchResults.length === 0 ? (
                    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
                        <Search size={32} style={{ color: currentTheme.sidebar.iconDisabled, opacity: 0.5 }} className="mb-3" />
                        <p className="text-[11px] font-medium mb-1" style={{ color: currentTheme.sidebar.itemText }}>
                            No results found
                        </p>
                        <p className="text-[10px]" style={{ color: currentTheme.sidebar.iconDisabled }}>
                            Try a different search term
                        </p>
                    </div>
                ) : (
                    <>
                        <div className="px-2 py-1 text-[10px] font-semibold" style={{ color: currentTheme.sidebar.iconDisabled }}>
                            {searchResults.length} {searchResults.length === 1 ? 'result' : 'results'}
                        </div>
                        <div className="flex flex-col gap-1 pb-2">
                            {searchResults.map(({ route, folder, matches }) => (
                                <div
                                    key={route.id}
                                    className="group px-3 py-2 rounded-md cursor-pointer transition-all"
                                    style={{
                                        backgroundColor: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemBg : 'transparent',
                                        color: selectedRoute?.id === route.id ? currentTheme.sidebar.activeItemText : currentTheme.sidebar.itemText
                                    }}
                                    onClick={() => onSelectRoute(route)}
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
                                >
                                    <div className="flex items-center gap-2 mb-1">
                                        <Globe 
                                            size={12} 
                                            style={{ 
                                                color: selectedRoute?.id === route.id 
                                                    ? currentTheme.sidebar.activeItemText 
                                                    : (route.enabled ? currentTheme.activityBar.activeIcon : currentTheme.sidebar.iconDisabled) 
                                            }} 
                                        />
                                        <span className="font-medium text-[11px] truncate">
                                            {highlightText(route.name || 'Unnamed Route', searchQuery)}
                                        </span>
                                    </div>
                                    
                                    <div className="flex items-center gap-2 ml-5">
                                        <span 
                                            className="px-1.5 py-0.5 text-[8px] font-bold rounded uppercase"
                                            style={{
                                                backgroundColor: route.method === 'GET' ? '#4A90E2' :
                                                               route.method === 'POST' ? '#4CAF50' :
                                                               route.method === 'PUT' ? '#FF9800' :
                                                               route.method === 'DELETE' ? '#F44336' : '#9E9E9E',
                                                color: 'white'
                                            }}
                                        >
                                            {route.method}
                                        </span>
                                        <span className="text-[10px] truncate" style={{ color: currentTheme.sidebar.headerText }}>
                                            {highlightText(route.path, searchQuery)}
                                        </span>
                                    </div>

                                    {folder && (
                                        <div className="flex items-center gap-1.5 mt-1 ml-5">
                                            <Folder size={10} style={{ color: folder.color || currentTheme.sidebar.iconDisabled }} />
                                            <span className="text-[9px]" style={{ color: currentTheme.sidebar.iconDisabled }}>
                                                {folder.name}
                                            </span>
                                        </div>
                                    )}

                                    {route.description && matches.includes('description') && (
                                        <div className="mt-1 ml-5 text-[9px] truncate" style={{ color: currentTheme.sidebar.iconDisabled }}>
                                            {highlightText(route.description, searchQuery)}
                                        </div>
                                    )}

                                    <div className="flex flex-wrap gap-1 mt-1 ml-5">
                                        {matches.map(match => (
                                            <span 
                                                key={match}
                                                className="px-1.5 py-0.5 text-[8px] rounded"
                                                style={{ 
                                                    backgroundColor: currentTheme.sidebar.itemHoverBg,
                                                    color: currentTheme.activityBar.indicator
                                                }}
                                            >
                                                {match}
                                            </span>
                                        ))}
                                    </div>
                                </div>
                            ))}
                        </div>
                    </>
                )}
            </div>

            {/* Search Tips */}
            {searchQuery.trim() && searchResults.length > 0 && (
                <div 
                    className="px-3 py-2 text-[9px] border-t"
                    style={{ 
                        color: currentTheme.sidebar.iconDisabled,
                        borderColor: currentTheme.sidebar.border,
                        backgroundColor: currentTheme.sidebar.bg
                    }}
                >
                    💡 Tip: Search is case-insensitive and searches across name, path, method, and description
                </div>
            )}
        </div>
    );
};
