
import React, { createContext, useContext, useState, useCallback, ReactNode, useEffect } from 'react';
import { X, CheckCircle, AlertTriangle, Info, AlertOctagon, Bell } from 'lucide-react';

export type NotificationType = 'INFO' | 'SUCCESS' | 'WARNING' | 'ERROR';

export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  duration?: number;
  timestamp: number;
}

interface NotificationContextType {
  notifications: Notification[];
  addNotification: (type: NotificationType, title: string, message: string, duration?: number) => void;
  removeNotification: (id: string) => void;
  clearAll: () => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export const useNotifications = () => {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error('useNotifications must be used within a NotificationProvider');
  }
  return context;
};

export const NotificationProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const addNotification = useCallback((type: NotificationType, title: string, message: string, duration = 5000) => {
    const id = Math.random().toString(36).substr(2, 9);
    const newNotification: Notification = {
      id,
      type,
      title,
      message,
      duration,
      timestamp: Date.now()
    };

    setNotifications((prev) => [newNotification, ...prev]); // Newest first

    if (duration > 0) {
      setTimeout(() => {
        removeNotification(id);
      }, duration);
    }
  }, []);

  const removeNotification = useCallback((id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  }, []);

  const clearAll = useCallback(() => {
    setNotifications([]);
  }, []);

  return (
    <NotificationContext.Provider value={{ notifications, addNotification, removeNotification, clearAll }}>
      {children}

      {/* Toast Container - Bottom Right - Minimalist Terminal Style */}
      <div className="fixed bottom-10 right-8 z-50 flex flex-col gap-2 w-80 pointer-events-none">
        {notifications.map((n) => (
          <div key={n.id} className="pointer-events-auto">
            <Toast notification={n} onClose={() => removeNotification(n.id)} />
          </div>
        ))}
      </div>
    </NotificationContext.Provider>
  );
};

const Toast: React.FC<{ notification: Notification; onClose: () => void }> = ({ notification, onClose }) => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const styles = {
    INFO: {
      color: '#3b82f6',
      icon: Info
    },
    SUCCESS: {
      color: '#10b981',
      icon: CheckCircle
    },
    WARNING: {
      color: '#f59e0b',
      icon: AlertTriangle
    },
    ERROR: {
      color: '#ef4444',
      icon: AlertOctagon
    }
  };

  const style = styles[notification.type];
  const Icon = style.icon;

  return (
    <div className={`
      relative overflow-hidden bg-black/40 backdrop-blur-xl border border-white/10
      rounded-2xl shadow-[0_16px_32px_-12px_rgba(0,0,0,0.5)] transition-all duration-500 ease-out
      ${mounted ? 'animate-blur-in' : 'opacity-0 scale-95 blur-md'}
    `}>
      {/* Status indicator glow */}
      <div className="absolute top-0 left-0 w-1 h-full opacity-60" style={{ backgroundColor: style.color }}></div>
      <div className="absolute top-0 left-0 w-12 h-full bg-gradient-to-r from-[var(--color-status)] to-transparent opacity-[0.03]" style={{ '--color-status': style.color } as any}></div>

      <div className="flex items-start p-4 gap-4">
        <div className="mt-1 p-2 rounded-xl bg-white/5 border border-white/5" style={{ color: style.color }}>
          <Icon size={18} />
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex justify-between items-start mb-1">
            <span className="text-[13px] font-black tracking-tighter uppercase" style={{ color: style.color }}>{notification.title}</span>
            <button onClick={onClose} className="text-white/20 hover:text-white transition-colors p-1">
              <X size={14} />
            </button>
          </div>
          <p className="text-[12px] text-gray-300 font-medium leading-relaxed">
            {notification.message}
          </p>
          <div className="mt-3 pt-3 border-t border-white/5 flex justify-between items-center text-[10px] text-gray-500 font-bold tracking-widest uppercase">
            <span>#{notification.id.substr(0, 6).toUpperCase()}</span>
            <span>{new Date(notification.timestamp).toLocaleTimeString([], { hour12: false })}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
