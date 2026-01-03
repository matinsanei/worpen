/**
 * Route Folders Manager
 * Manages virtual folder structure for dynamic routes in frontend only
 * Stores data in localStorage for persistence
 */

export interface RouteFolder {
    id: string;
    name: string;
    routeIds: string[];
    color?: string;
    createdAt: number;
}

const STORAGE_KEY = 'worpen_route_folders';

export class RouteFoldersManager {
    private folders: Map<string, RouteFolder>;

    constructor() {
        this.folders = new Map();
        this.load();
    }

    /**
     * Load folders from localStorage
     */
    private load(): void {
        try {
            const stored = localStorage.getItem(STORAGE_KEY);
            if (stored) {
                const data: RouteFolder[] = JSON.parse(stored);
                data.forEach(folder => {
                    this.folders.set(folder.id, folder);
                });
            }
        } catch (error) {
            console.error('Failed to load route folders:', error);
        }
    }

    /**
     * Save folders to localStorage
     */
    private save(): void {
        try {
            const data = Array.from(this.folders.values());
            localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
        } catch (error) {
            console.error('Failed to save route folders:', error);
        }
    }

    /**
     * Create a new folder
     */
    createFolder(name: string, color?: string): RouteFolder {
        const folder: RouteFolder = {
            id: `folder_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            name,
            routeIds: [],
            color,
            createdAt: Date.now()
        };
        this.folders.set(folder.id, folder);
        this.save();
        return folder;
    }

    /**
     * Get all folders
     */
    getAllFolders(): RouteFolder[] {
        return Array.from(this.folders.values()).sort((a, b) => a.name.localeCompare(b.name));
    }

    /**
     * Get a specific folder
     */
    getFolder(folderId: string): RouteFolder | undefined {
        return this.folders.get(folderId);
    }

    /**
     * Update folder name
     */
    renameFolder(folderId: string, newName: string): boolean {
        const folder = this.folders.get(folderId);
        if (!folder) return false;
        
        folder.name = newName;
        this.save();
        return true;
    }

    /**
     * Update folder color
     */
    setFolderColor(folderId: string, color: string): boolean {
        const folder = this.folders.get(folderId);
        if (!folder) return false;
        
        folder.color = color;
        this.save();
        return true;
    }

    /**
     * Delete a folder
     */
    deleteFolder(folderId: string): boolean {
        const deleted = this.folders.delete(folderId);
        if (deleted) {
            this.save();
        }
        return deleted;
    }

    /**
     * Add a route to a folder
     */
    addRouteToFolder(folderId: string, routeId: string): boolean {
        const folder = this.folders.get(folderId);
        if (!folder) return false;
        
        // Remove from other folders first
        this.removeRouteFromAllFolders(routeId);
        
        if (!folder.routeIds.includes(routeId)) {
            folder.routeIds.push(routeId);
            this.save();
        }
        return true;
    }

    /**
     * Remove a route from a folder
     */
    removeRouteFromFolder(folderId: string, routeId: string): boolean {
        const folder = this.folders.get(folderId);
        if (!folder) return false;
        
        const index = folder.routeIds.indexOf(routeId);
        if (index > -1) {
            folder.routeIds.splice(index, 1);
            this.save();
            return true;
        }
        return false;
    }

    /**
     * Remove a route from all folders
     */
    removeRouteFromAllFolders(routeId: string): void {
        let changed = false;
        this.folders.forEach(folder => {
            const index = folder.routeIds.indexOf(routeId);
            if (index > -1) {
                folder.routeIds.splice(index, 1);
                changed = true;
            }
        });
        if (changed) {
            this.save();
        }
    }

    /**
     * Get folder containing a specific route
     */
    getFolderByRoute(routeId: string): RouteFolder | undefined {
        for (const folder of this.folders.values()) {
            if (folder.routeIds.includes(routeId)) {
                return folder;
            }
        }
        return undefined;
    }

    /**
     * Get routes that are not in any folder
     */
    getUncategorizedRouteIds(allRouteIds: string[]): string[] {
        const categorized = new Set<string>();
        this.folders.forEach(folder => {
            folder.routeIds.forEach(id => categorized.add(id));
        });
        return allRouteIds.filter(id => !categorized.has(id));
    }

    /**
     * Export folders for backup
     */
    export(): string {
        return JSON.stringify(Array.from(this.folders.values()), null, 2);
    }

    /**
     * Import folders from backup
     */
    import(jsonData: string): boolean {
        try {
            const data: RouteFolder[] = JSON.parse(jsonData);
            this.folders.clear();
            data.forEach(folder => {
                this.folders.set(folder.id, folder);
            });
            this.save();
            return true;
        } catch (error) {
            console.error('Failed to import folders:', error);
            return false;
        }
    }

    /**
     * Clear all folders
     */
    clearAll(): void {
        this.folders.clear();
        this.save();
    }
}

// Singleton instance
let instance: RouteFoldersManager | null = null;

export function getRouteFoldersManager(): RouteFoldersManager {
    if (!instance) {
        instance = new RouteFoldersManager();
    }
    return instance;
}
