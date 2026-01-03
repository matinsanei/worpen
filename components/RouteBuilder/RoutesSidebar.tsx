import React from 'react';
import { ChevronDown, ChevronRight, Braces, Globe, Plus, Trash2, Folder as FolderIcon } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';
import { ROUTE_TEMPLATES } from './constants';
import { FolderManager } from './FolderManager';
import { RouteFolder } from '../../src/utils/routeFolders';

interface RoutesSidebarProps {
    currentTheme: IDETheme;
    routes: any[];
    selectedRoute: any;
    expandedFolders: string[];
    onToggleFolder: (folder: string) => void;
    onLoadTemplate: (key: string) => void;
    onSelectRoute: (route: any) => void;
    onDeleteRoute: (routeId: string) => void;
    // Folder management props
    folders?: RouteFolder[];
    onCreateFolder?: (name: string, color?: string) => void;
    onRenameFolder?: (folderId: string, newName: string) => void;
    onDeleteFolder?: (folderId: string) => void;
    onToggleFolderExpansion?: (folderId: string) => void;
    onAddRouteToFolder?: (folderId: string, routeId: string) => void;
    onRemoveRouteFromFolder?: (folderId: string, routeId: string) => void;
    expandedFolderIds?: string[];
    uncategorizedRoutes?: any[];
}

export const RoutesSidebar: React.FC<RoutesSidebarProps> = ({
    currentTheme,
    routes,
    selectedRoute,
    expandedFolders,
    onToggleFolder,
    onLoadTemplate,
    onSelectRoute,
    onDeleteRoute,
    // Folder props with defaults
    folders = [],
    onCreateFolder = () => {},
    onRenameFolder = () => {},
    onDeleteFolder = () => {},
    onToggleFolderExpansion = () => {},
    onAddRouteToFolder = () => {},
    onRemoveRouteFromFolder = () => {},
    expandedFolderIds = [],
    uncategorizedRoutes = []
}) => {
    const [draggedRoute, setDraggedRoute] = React.useState<string | null>(null);
    const [dropTarget, setDropTarget] = React.useState<string | null>(null);

    const handleDragStart = (e: React.DragEvent, routeId: string) => {
        setDraggedRoute(routeId);
        e.dataTransfer.effectAllowed = 'move';
        e.dataTransfer.setData('text/plain', routeId);
    };

    const handleDragOver = (e: React.DragEvent, folderId: string) => {
        e.preventDefault();
        e.dataTransfer.dropEffect = 'move';
        setDropTarget(folderId);
    };

    const handleDragLeave = () => {
        setDropTarget(null);
    };

    const handleDrop = (e: React.DragEvent, folderId: string) => {
        e.preventDefault();
        const routeId = e.dataTransfer.getData('text/plain');
        if (routeId && onAddRouteToFolder) {
            onAddRouteToFolder(folderId, routeId);
        }
        setDraggedRoute(null);
        setDropTarget(null);
    };

    return (
        <div className="flex-1 overflow-y-auto custom-scrollbar flex flex-col py-2">
            {/* Templates Section - Compact */}
            <button
                onClick={() => onToggleFolder('templates')}
                className="h-6 px-2 flex items-center text-[10px] font-semibold transition-colors mb-1"
                style={{ color: currentTheme.sidebar.folderText }}
                onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg}
                onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}
            >
                <div className="mr-1">
                    {expandedFolders.includes('templates') ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                </div>
                <Braces size={12} className="mr-1.5" style={{ opacity: 0.7 }} />
                <span className="uppercase tracking-tight text-[10px]">Templates</span>
            </button>
            {expandedFolders.includes('templates') && (
                <div className="flex flex-col pb-2 mb-2 border-b" style={{ borderColor: currentTheme.sidebar.border }}>
                    {Object.entries(ROUTE_TEMPLATES).map(([key, template]) => (
                        <button
                            key={key}
                            onClick={() => onLoadTemplate(key)}
                            className="px-4 py-1 text-[11px] flex items-center gap-1.5 transition-all group text-left mx-1 rounded-md"
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
                            <Plus size={10} className="opacity-0 group-hover:opacity-100 transition-opacity" style={{ color: currentTheme.activityBar.activeIcon }} />
                            <span className="truncate text-[10px]">{template.name}</span>
                        </button>
                    ))}
                </div>
            )}

            {/* Custom Folders Section - Always visible */}
            <div className="mb-2">
                <button
                    onClick={() => onToggleFolder('custom-folders')}
                    className="h-7 px-2 flex items-center text-[11px] font-bold transition-colors w-full"
                    style={{ color: currentTheme.sidebar.folderText }}
                    onMouseEnter={(e: any) => e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg}
                    onMouseLeave={(e: any) => e.currentTarget.style.backgroundColor = 'transparent'}
                >
                    <div className="mr-1 shadow-sm">
                        {expandedFolders.includes('custom-folders') ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                    </div>
                    <FolderIcon size={14} className="mr-2" />
                    <span className="uppercase tracking-tight">MY FOLDERS {folders.length > 0 && `(${folders.length})`}</span>
                </button>
                {expandedFolders.includes('custom-folders') && (
                    <FolderManager
                        currentTheme={currentTheme}
                        folders={folders}
                        expandedFolders={expandedFolderIds}
                        onCreateFolder={onCreateFolder}
                        onRenameFolder={onRenameFolder}
                        onDeleteFolder={onDeleteFolder}
                        onToggleFolder={onToggleFolderExpansion}
                        onRemoveRouteFromFolder={onRemoveRouteFromFolder}
                        onAddRouteToFolder={onAddRouteToFolder}
                        onSelectRoute={onSelectRoute}
                        routes={routes}
                        selectedRoute={selectedRoute}
                    />
                )}
            </div>

            {/* Active Routes Section - Only Uncategorized */}
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
                    <span className="uppercase tracking-tight">
                        {folders && folders.length > 0 ? 'UNCATEGORIZED' : 'ACTIVE ROUTES'} ({folders && folders.length > 0 ? uncategorizedRoutes.length : routes.length})
                    </span>
                </button>
                {expandedFolders.includes('active-routes') && (
                    <div className="flex flex-col py-1">
                        {(folders && folders.length > 0 ? uncategorizedRoutes : routes).length === 0 ? (
                            <div className="px-8 py-4 text-center text-[11px] text-[#6e7073] italic">
                                {folders && folders.length > 0 ? 'All routes are organized in folders' : 'No dynamic routes found'}
                            </div>
                        ) : (
                            (folders && folders.length > 0 ? uncategorizedRoutes : routes).map((route) => (
                                <div
                                    key={route.id}
                                    draggable={folders && folders.length > 0}
                                    onDragStart={(e) => folders && folders.length > 0 && handleDragStart(e, route.id)}
                                    className={`group px-6 py-1.5 text-[12px] flex items-center gap-2 cursor-pointer transition-all mx-1 rounded-[var(--radius)] ${
                                        draggedRoute === route.id ? 'opacity-50' : ''
                                    }`}
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
