
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
                <div className="absolute right-0 top-full mt-3 w-80 bg-black/40 backdrop-blur-xl border border-white/10 rounded-2xl shadow-[0_32px_64px_-16px_rgba(0,0,0,0.6)] z-50 overflow-hidden flex flex-col animate-blur-in origin-top-right">
                    {/* Header */}
                    <div className="p-4 border-b border-white/10 flex items-center justify-between bg-white/5">
                        <div className="flex items-center gap-3">
                            <span className="text-[11px] font-black text-gray-200 uppercase tracking-[2px] opacity-80">System Activity</span>
                            <span className="text-[10px] bg-white/10 px-2 py-0.5 rounded-full text-white font-bold">{notifications.length}</span>
                        </div>
                        {hasNotifications && (
                            <button
                                onClick={clearAll}
                                className="text-[10px] text-gray-500 hover:text-red-400 flex items-center gap-1.5 transition-colors uppercase font-black tracking-widest"
                            >
                                <Trash2 size={12} /> Clear
                            </button>
                        )}
                    </div>

                    {/* List */}
                    <div className="max-h-[350px] overflow-y-auto custom-scrollbar">
                        {notifications.length === 0 ? (
                            <div className="p-10 text-center text-gray-600 flex flex-col items-center gap-4">
                                <div className="p-4 rounded-2xl bg-white/5 border border-white/5">
                                    <CheckCircle size={24} className="opacity-20" />
                                </div>
                                <span className="text-xs font-bold uppercase tracking-widest opacity-40">All systems nominal</span>
                            </div>
                        ) : (
                            <div className="divide-y divide-white/5">
                                {notifications.map((n) => {
                                    const Icon = getIcon(n.type);
                                    const colorClass = getColor(n.type);
                                    return (
                                        <div key={n.id} className="p-4 hover:bg-white/5 transition-colors group relative">
                                            <div className="flex gap-4 items-start">
                                                <div className={`mt-0.5 ${colorClass} p-1.5 rounded-lg bg-white/5 border border-white/5`}>
                                                    <Icon size={14} />
                                                </div>
                                                <div className="flex-1 min-w-0">
                                                    <div className="flex justify-between items-start mb-1">
                                                        <span className={`text-xs font-black uppercase tracking-tight ${colorClass}`}>{n.title}</span>
                                                        <span className="text-[9px] text-gray-500 font-bold opacity-30 tracking-tighter uppercase whitespace-nowrap">
                                                            {new Date(n.timestamp).toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit' })}
                                                        </span>
                                                    </div>
                                                    <p className="text-[11px] text-gray-400 font-medium leading-relaxed break-words">
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
                                                className="absolute right-4 top-1/2 -translate-y-1/2 p-2 bg-black/80 backdrop-blur-md border border-white/10 rounded-xl text-gray-400 hover:text-red-500 hover:border-red-500/50 opacity-0 group-hover:opacity-100 transition-all shadow-xl scale-90 group-hover:scale-100"
                                            >
                                                <X size={14} />
                                            </button>
                                        </div>
                                    );
                                })}
                            </div>
                        )}
                    </div>

                    {/* Footer */}
                    <div className="p-3 border-t border-white/10 bg-white/5">
                        <div className="flex items-center gap-3 text-[10px] text-gray-600 font-black tracking-[2px] justify-center uppercase">
                            <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse shadow-[0_0_8px_rgba(34,197,94,0.4)]"></div>
                            Live Feed Active
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};
