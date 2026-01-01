import React from 'react';
import { File, Search, GitBranch, Play, Settings } from 'lucide-react';
import { IDETheme } from '../../src/themes/ide';

interface ActivityBarProps {
    currentTheme: IDETheme;
    activeActivityId: 'explorer' | 'search' | 'git' | 'debug';
    sidebarVisible: boolean;
    onActivityChange: (id: 'explorer' | 'search' | 'git' | 'debug') => void;
    onSettingsClick: () => void;
}

export const ActivityBar: React.FC<ActivityBarProps> = ({
    currentTheme,
    activeActivityId,
    sidebarVisible,
    onActivityChange,
    onSettingsClick
}) => {
    const activities = [
        { id: 'explorer' as const, icon: File, label: 'Explorer' },
        { id: 'search' as const, icon: Search, label: 'Search' },
        { id: 'git' as const, icon: GitBranch, label: 'Source Control' },
        { id: 'debug' as const, icon: Play, label: 'Run and Debug' },
    ];

    return (
        <div className="w-10 flex flex-col items-center py-2 gap-2 border-r" style={{ backgroundColor: currentTheme.activityBar.bg, borderColor: currentTheme.activityBar.border }}>
            {activities.map((item) => (
                <button
                    key={item.id}
                    onClick={() => onActivityChange(item.id)}
                    className={`p-2 transition-all relative group rounded-[var(--radius)] mx-1`}
                    style={{
                        color: activeActivityId === item.id && sidebarVisible ? currentTheme.activityBar.activeIcon : currentTheme.activityBar.inactiveIcon,
                        backgroundColor: activeActivityId === item.id && sidebarVisible ? currentTheme.activityBar.activeBg : 'transparent'
                    }}
                    onMouseEnter={(e) => {
                        if (!(activeActivityId === item.id && sidebarVisible)) {
                            e.currentTarget.style.backgroundColor = currentTheme.activityBar.hoverBg;
                            e.currentTarget.style.color = currentTheme.activityBar.activeIcon;
                        }
                    }}
                    onMouseLeave={(e) => {
                        if (!(activeActivityId === item.id && sidebarVisible)) {
                            e.currentTarget.style.backgroundColor = 'transparent';
                            e.currentTarget.style.color = currentTheme.activityBar.inactiveIcon;
                        }
                    }}
                    title={item.label}
                >
                    <item.icon size={20} strokeWidth={activeActivityId === item.id ? 2.5 : 2} />
                    {activeActivityId === item.id && sidebarVisible && (
                        <div className="absolute left-[-4px] top-1/2 -translate-y-1/2 w-1 h-5 rounded-r-full shadow-[0_0_8px_rgba(53,116,240,0.3)]" style={{ backgroundColor: currentTheme.activityBar.indicator }} />
                    )}
                </button>
            ))}
            <div className="mt-auto flex flex-col gap-2 mb-1 relative">
                <button
                    onClick={onSettingsClick}
                    className="p-2 transition-all rounded-[var(--radius)] mx-1"
                    style={{
                        color: currentTheme.activityBar.inactiveIcon,
                    }}
                    onMouseEnter={(e) => {
                        e.currentTarget.style.backgroundColor = currentTheme.activityBar.hoverBg;
                        e.currentTarget.style.color = currentTheme.activityBar.activeIcon;
                    }}
                    onMouseLeave={(e) => {
                        e.currentTarget.style.backgroundColor = 'transparent';
                        e.currentTarget.style.color = currentTheme.activityBar.inactiveIcon;
                    }}
                >
                    <Settings size={20} strokeWidth={2} />
                </button>
            </div>
        </div>
    );
};
