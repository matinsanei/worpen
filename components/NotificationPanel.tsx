
import React, { useState, useRef, useEffect } from 'react';
import { Bell, X, Check, Trash2, Info, AlertTriangle, AlertOctagon, CheckCircle } from 'lucide-react';
import { useNotifications, Notification } from './NotificationSystem';

export const NotificationPanel: React.FC = () => {
    const { notifications, removeNotification, clearAll } = useNotifications();
    const [isOpen, setIsOpen] = useState(false);
    const panelRef = useRef<HTMLDivElement>(null);

    const hasNotifications = notifications.length > 0;

    // Close on click outside
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (panelRef.current && !panelRef.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        };

        if (isOpen) {
            document.addEventListener('mousedown', handleClickOutside);
        }
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, [isOpen]);

    const getIcon = (type: Notification['type']) => {
        switch (type) {
            case 'INFO': return Info;
            case 'SUCCESS': return CheckCircle;
            case 'WARNING': return AlertTriangle;
            case 'ERROR': return AlertOctagon;
        }
    };

    const getColor = (type: Notification['type']) => {
        switch (type) {
            case 'INFO': return 'text-blue-400';
            case 'SUCCESS': return 'text-green-400';
            case 'WARNING': return 'text-yellow-400';
            case 'ERROR': return 'text-red-500';
        }
    };

    return (
        <div className="relative" ref={panelRef}>
            <button
                onClick={() => setIsOpen(!isOpen)}
                className={`transition-colors relative group p-2 rounded-md ${isOpen ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-white hover:bg-white/5'}`}
                title="Notifications"
            >
                <Bell size={20} className={hasNotifications ? "text-green-400" : ""} />
                {hasNotifications && (
                    <span className="absolute top-1 right-1 flex h-2.5 w-2.5 items-center justify-center rounded-full bg-red-500 text-[8px] text-white font-bold animate-pulse shadow-[0_0_5px_rgba(239,68,68,0.8)]"></span>
                )}
            </button>

            {isOpen && (
                <div className="absolute right-0 top-full mt-2 w-80 bg-[#050505] border border-white/10 rounded-sm shadow-2xl shadow-black/80 z-50 overflow-hidden flex flex-col animate-in fade-in zoom-in-95 duration-100 origin-top-right">
                    {/* Header */}
                    <div className="p-3 border-b border-white/10 flex items-center justify-between bg-white/5">
                        <div className="flex items-center gap-2">
                            <span className="text-xs font-bold text-gray-200 uppercase tracking-wider font-mono">System Logs</span>
                            <span className="text-[10px] bg-white/10 px-1.5 py-0.5 rounded text-gray-400 font-mono">{notifications.length}</span>
                        </div>
                        {hasNotifications && (
                            <button
                                onClick={clearAll}
                                className="text-[10px] text-gray-500 hover:text-red-400 flex items-center gap-1 transition-colors uppercase font-mono tracking-wider"
                            >
                                <Trash2 size={10} /> Clear All
                            </button>
                        )}
                    </div>

                    {/* List */}
                    <div className="max-h-[300px] overflow-y-auto custom-scrollbar bg-black/50">
                        {notifications.length === 0 ? (
                            <div className="p-8 text-center text-gray-600 flex flex-col items-center gap-2">
                                <Check size={24} className="opacity-20" />
                                <span className="text-xs font-mono">No active alerts</span>
                            </div>
                        ) : (
                            <div className="divide-y divide-white/5">
                                {notifications.map((n) => {
                                    const Icon = getIcon(n.type);
                                    const colorClass = getColor(n.type);
                                    return (
                                        <div key={n.id} className="p-3 hover:bg-white/5 transition-colors group relative">
                                            <div className="flex gap-3 items-start">
                                                <div className={`mt-0.5 ${colorClass}`}>
                                                    <Icon size={14} />
                                                </div>
                                                <div className="flex-1 min-w-0">
                                                    <div className="flex justify-between items-start">
                                                        <span className={`text-xs font-medium ${colorClass}`}>{n.title}</span>
                                                        <span className="text-[9px] text-gray-600 font-mono ml-2 whitespace-nowrap">
                                                            {new Date(n.timestamp).toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit' })}
                                                        </span>
                                                    </div>
                                                    <p className="text-[11px] text-gray-400 mt-0.5 leading-snug break-words">
                                                        {n.message}
                                                    </p>
                                                </div>
                                            </div>

                                            {/* Delete Action - Visible on hover */}
                                            <button
                                                onClick={(e) => {
                                                    e.stopPropagation();
                                                    removeNotification(n.id);
                                                }}
                                                className="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 bg-black border border-white/10 rounded text-gray-500 hover:text-red-500 hover:border-red-500/50 opacity-0 group-hover:opacity-100 transition-all shadow-lg"
                                            >
                                                <X size={12} />
                                            </button>
                                        </div>
                                    );
                                })}
                            </div>
                        )}
                    </div>

                    {/* Footer */}
                    <div className="p-2 border-t border-white/10 bg-[#080808]">
                        <div className="flex items-center gap-2 text-[10px] text-gray-600 font-mono justify-center">
                            <div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"></div>
                            LIVE FEED :: ACTIVE
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};
