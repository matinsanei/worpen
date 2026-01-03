import React, { useState } from 'react';
import { Folder, FolderPlus, Edit2, Trash2, X, Check, ChevronDown, ChevronRight } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';
import { RouteFolder } from '../../src/utils/routeFolders';

interface FolderManagerProps {
    currentTheme: IDETheme;
    folders: RouteFolder[];
    expandedFolders: string[];
    onCreateFolder: (name: string, color?: string) => void;
    onRenameFolder: (folderId: string, newName: string) => void;
    onDeleteFolder: (folderId: string) => void;
    onToggleFolder: (folderId: string) => void;
    onRemoveRouteFromFolder: (folderId: string, routeId: string) => void;
    onAddRouteToFolder?: (folderId: string, routeId: string) => void;
    onSelectRoute: (route: any) => void;
    routes: any[];
    selectedRoute: any;
}

const FOLDER_COLORS = [
    { name: 'Blue', value: '#4A90E2' },
    { name: 'Green', value: '#4CAF50' },
    { name: 'Orange', value: '#FF9800' },
    { name: 'Purple', value: '#9C27B0' },
    { name: 'Red', value: '#F44336' },
    { name: 'Cyan', value: '#00BCD4' },
    { name: 'Pink', value: '#E91E63' },
    { name: 'Yellow', value: '#FFC107' },
];

export const FolderManager: React.FC<FolderManagerProps> = ({
    currentTheme,
    folders,
    expandedFolders,
    onCreateFolder,
    onRenameFolder,
    onDeleteFolder,
    onToggleFolder,
    onRemoveRouteFromFolder,
    onAddRouteToFolder,
    onSelectRoute,
    routes,
    selectedRoute
}) => {
    const [isCreating, setIsCreating] = useState(false);
    const [newFolderName, setNewFolderName] = useState('');
    const [selectedColor, setSelectedColor] = useState(FOLDER_COLORS[0].value);
    const [editingFolderId, setEditingFolderId] = useState<string | null>(null);
    const [editingName, setEditingName] = useState('');
    const [dropTarget, setDropTarget] = useState<string | null>(null);

    const handleCreateFolder = () => {
        if (newFolderName.trim()) {
            onCreateFolder(newFolderName.trim(), selectedColor);
            setNewFolderName('');
            setSelectedColor(FOLDER_COLORS[0].value);
            setIsCreating(false);
        }
    };

    const handleStartEdit = (folder: RouteFolder) => {
        setEditingFolderId(folder.id);
        setEditingName(folder.name);
    };

    const handleSaveEdit = () => {
        if (editingFolderId && editingName.trim()) {
            onRenameFolder(editingFolderId, editingName.trim());
            setEditingFolderId(null);
            setEditingName('');
        }
    };

    const handleCancelEdit = () => {
        setEditingFolderId(null);
        setEditingName('');
    };

    const getRouteById = (routeId: string) => {
        return routes.find(r => r.id === routeId);
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
        setDropTarget(null);
    };

    return (
        <div className="flex flex-col">
            {/* Create Folder Button */}
            <div className="px-2 py-1">
                {!isCreating ? (
                    <button
                        onClick={() => setIsCreating(true)}
                        className="w-full h-8 flex items-center gap-2 px-3 text-[11px] rounded-md transition-all hover:scale-[1.02]"
                        style={{ 
                            backgroundColor: currentTheme.sidebar.itemHoverBg,
                            color: currentTheme.sidebar.itemText 
                        }}
                    >
                        <FolderPlus size={14} />
                        <span className="font-medium">New Folder</span>
                    </button>
                ) : (
                    <div 
                        className="w-full p-2 rounded-md"
                        style={{ backgroundColor: currentTheme.sidebar.itemHoverBg }}
                    >
                        <input
                            type="text"
                            value={newFolderName}
                            onChange={(e) => setNewFolderName(e.target.value)}
                            onKeyDown={(e) => {
                                if (e.key === 'Enter') handleCreateFolder();
                                if (e.key === 'Escape') setIsCreating(false);
                            }}
                            placeholder="Folder name..."
                            autoFocus
                            className="w-full px-2 py-1 text-[11px] rounded mb-2"
                            style={{
                                backgroundColor: currentTheme.editor.bg,
                                color: currentTheme.editor.tabActiveText,
                                border: `1px solid ${currentTheme.sidebar.border}`
                            }}
                        />
                        <div className="flex items-center gap-2 mb-2">
                            <span className="text-[10px]" style={{ color: currentTheme.sidebar.itemText }}>Color:</span>
                            <div className="flex gap-1">
                                {FOLDER_COLORS.map(color => (
                                    <button
                                        key={color.value}
                                        onClick={() => setSelectedColor(color.value)}
                                        className="w-5 h-5 rounded-full transition-transform hover:scale-110"
                                        style={{
                                            backgroundColor: color.value,
                                            border: selectedColor === color.value ? '2px solid white' : '1px solid rgba(255,255,255,0.3)'
                                        }}
                                        title={color.name}
                                    />
                                ))}
                            </div>
                        </div>
                        <div className="flex gap-1">
                            <button
                                onClick={handleCreateFolder}
                                className="flex-1 h-6 flex items-center justify-center gap-1 text-[10px] rounded transition-colors"
                                style={{
                                    backgroundColor: currentTheme.activityBar.activeIcon,
                                    color: currentTheme.editor.bg
                                }}
                            >
                                <Check size={12} />
                                Create
                            </button>
                            <button
                                onClick={() => setIsCreating(false)}
                                className="h-6 px-2 flex items-center justify-center rounded transition-colors"
                                style={{
                                    backgroundColor: currentTheme.sidebar.border,
                                    color: currentTheme.sidebar.itemText
                                }}
                            >
                                <X size={12} />
                            </button>
                        </div>
                    </div>
                )}
            </div>

            {/* Folders List */}
            <div className="flex flex-col py-1">
                {folders.map(folder => {
                    const folderRoutes = folder.routeIds.map(getRouteById).filter(Boolean);
                    const isExpanded = expandedFolders.includes(folder.id);
                    const isEditing = editingFolderId === folder.id;

                    return (
                        <div key={folder.id} className="mb-1">
                            {/* Folder Header */}
                            <div
                                className="group h-7 px-2 flex items-center text-[11px] font-medium transition-colors relative"
                                style={{ 
                                    color: currentTheme.sidebar.folderText,
                                    backgroundColor: dropTarget === folder.id ? currentTheme.activityBar.indicator + '30' : 'transparent'
                                }}
                                onMouseEnter={(e: any) => {
                                    if (dropTarget !== folder.id) {
                                        e.currentTarget.style.backgroundColor = currentTheme.sidebar.folderHoverBg;
                                    }
                                }}
                                onMouseLeave={(e: any) => {
                                    if (dropTarget !== folder.id) {
                                        e.currentTarget.style.backgroundColor = 'transparent';
                                    }
                                }}
                                onDragOver={(e) => handleDragOver(e, folder.id)}
                                onDragLeave={handleDragLeave}
                                onDrop={(e) => handleDrop(e, folder.id)}
                            >
                                <button
                                    onClick={() => onToggleFolder(folder.id)}
                                    className="mr-1 flex items-center"
                                >
                                    {isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
                                </button>
                                
                                <Folder 
                                    size={14} 
                                    className="mr-2" 
                                    style={{ color: folder.color || currentTheme.activityBar.activeIcon }}
                                />
                                
                                {isEditing ? (
                                    <input
                                        type="text"
                                        value={editingName}
                                        onChange={(e) => setEditingName(e.target.value)}
                                        onKeyDown={(e) => {
                                            if (e.key === 'Enter') handleSaveEdit();
                                            if (e.key === 'Escape') handleCancelEdit();
                                        }}
                                        onBlur={handleSaveEdit}
                                        autoFocus
                                        className="flex-1 px-1 text-[11px] rounded"
                                        style={{
                                            backgroundColor: currentTheme.editor.bg,
                                            color: currentTheme.editor.tabActiveText,
                                            border: `1px solid ${currentTheme.sidebar.border}`
                                        }}
                                    />
                                ) : (
                                    <>
                                        <span className="flex-1 truncate">{folder.name}</span>
                                        <span className="text-[10px] mr-2" style={{ color: currentTheme.sidebar.iconDisabled }}>
                                            ({folderRoutes.length})
                                        </span>
                                    </>
                                )}
                                
                                {!isEditing && (
                                    <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button
                                            onClick={(e) => {
                                                e.stopPropagation();
                                                handleStartEdit(folder);
                                            }}
                                            className="p-1 rounded hover:bg-black/20"
                                            style={{ color: currentTheme.sidebar.iconEnabled }}
                                        >
                                            <Edit2 size={12} />
                                        </button>
                                        <button
                                            onClick={(e) => {
                                                e.stopPropagation();
                                                if (confirm(`Delete folder "${folder.name}"? Routes will be moved to uncategorized.`)) {
                                                    onDeleteFolder(folder.id);
                                                }
                                            }}
                                            className="p-1 rounded hover:bg-black/20"
                                            style={{ color: currentTheme.sidebar.iconEnabled }}
                                        >
                                            <Trash2 size={12} />
                                        </button>
                                    </div>
                                )}
                            </div>

                            {/* Folder Routes */}
                            {isExpanded && (
                                <div className="flex flex-col py-1 pl-4">
                                    {folderRoutes.length === 0 ? (
                                        <div className="px-6 py-2 text-center text-[10px]" style={{ color: currentTheme.sidebar.iconDisabled }}>
                                            Empty folder
                                        </div>
                                    ) : (
                                        folderRoutes.map((route: any) => (
                                            <div
                                                key={route.id}
                                                className="group px-4 py-1.5 text-[12px] flex items-center gap-2 cursor-pointer transition-all mx-1 rounded-md"
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
                                                <span 
                                                    className="px-1.5 py-0.5 text-[9px] font-bold rounded uppercase"
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
                                                <span className="flex-1 truncate">{route.path}</span>
                                                <button
                                                    onClick={(e) => {
                                                        e.stopPropagation();
                                                        onRemoveRouteFromFolder(folder.id, route.id);
                                                    }}
                                                    className="opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-black/20"
                                                    style={{ color: currentTheme.sidebar.iconEnabled }}
                                                    title="Remove from folder"
                                                >
                                                    <X size={12} />
                                                </button>
                                            </div>
                                        ))
                                    )}
                                </div>
                            )}
                        </div>
                    );
                })}
            </div>
        </div>
    );
};
