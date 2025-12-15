
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
      border: 'border-l-blue-500',
      text: 'text-blue-500',
      icon: Info
    },
    SUCCESS: {
      border: 'border-l-green-500',
      text: 'text-green-500',
      icon: CheckCircle
    },
    WARNING: {
      border: 'border-l-yellow-500',
      text: 'text-yellow-500',
      icon: AlertTriangle
    },
    ERROR: {
      border: 'border-l-red-500',
      text: 'text-red-500',
      icon: AlertOctagon
    }
  };

  const style = styles[notification.type];
  const Icon = style.icon;

  return (
    <div className={`
      relative overflow-hidden bg-[#0a0a0a] border border-white/10 border-l-2 ${style.border}
      shadow-lg shadow-black/50 transition-all duration-300 ease-out
      ${mounted ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-8'}
    `}>
      <div className="flex items-start p-3 gap-3">
        <div className={`mt-0.5 ${style.text}`}>
          <Icon size={16} />
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex justify-between items-start mb-1">
            <span className={`text-xs font-bold font-mono ${style.text}`}>{notification.title}</span>
            <button onClick={onClose} className="text-gray-600 hover:text-white transition-colors">
              <X size={12} />
            </button>
          </div>
          <p className="text-xs text-gray-400 font-mono leading-relaxed">
            {notification.message}
          </p>
          <div className="mt-2 pt-2 border-t border-white/5 flex justify-between items-center text-[9px] text-gray-600 font-mono">
            <span>#{notification.id.toUpperCase()}</span>
            <span>{new Date(notification.timestamp).toLocaleTimeString([], { hour12: false })}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
