
import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
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
      
      {/* Toast Container - Fixed Top Right */}
      <div className="fixed top-20 right-6 z-50 flex flex-col gap-3 w-80 md:w-96 pointer-events-none">
        {notifications.map((n) => (
          <div key={n.id} className="pointer-events-auto animate-slide-in-right">
             <Toast notification={n} onClose={() => removeNotification(n.id)} />
          </div>
        ))}
      </div>
    </NotificationContext.Provider>
  );
};

const Toast: React.FC<{ notification: Notification; onClose: () => void }> = ({ notification, onClose }) => {
  const styles = {
    INFO: {
      border: 'border-blue-500',
      bg: 'bg-blue-950/90',
      text: 'text-blue-400',
      icon: Info
    },
    SUCCESS: {
      border: 'border-green-500',
      bg: 'bg-green-950/90',
      text: 'text-green-400',
      icon: CheckCircle
    },
    WARNING: {
      border: 'border-yellow-500',
      bg: 'bg-yellow-950/90',
      text: 'text-yellow-400',
      icon: AlertTriangle
    },
    ERROR: {
      border: 'border-red-500',
      bg: 'bg-red-950/90',
      text: 'text-red-400',
      icon: AlertOctagon
    }
  };

  const style = styles[notification.type];
  const Icon = style.icon;

  return (
    <div className={`
      relative overflow-hidden rounded-sm 
      border-l-4 ${style.border} 
      bg-[#0a0a0a] border-y border-r border-white/10
      shadow-[0_0_15px_rgba(0,0,0,0.5)] 
      backdrop-blur-md
      group
    `}>
      {/* Scanline overlay */}
      <div className="absolute inset-0 bg-gradient-to-b from-white/5 to-transparent opacity-20 pointer-events-none"></div>
      
      <div className="flex p-4 gap-3 relative z-10">
        <div className={`mt-1 ${style.text}`}>
          <Icon size={18} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex justify-between items-start">
            <h4 className={`text-xs font-bold font-mono tracking-wider mb-1 ${style.text}`}>
              {notification.title}
            </h4>
            <button 
              onClick={onClose}
              className="text-gray-500 hover:text-white transition-colors"
            >
              <X size={14} />
            </button>
          </div>
          <p className="text-xs text-gray-300 leading-relaxed font-sans">
            {notification.message}
          </p>
          <div className="mt-2 text-[9px] text-gray-600 font-mono text-right">
             {new Date(notification.timestamp).toLocaleTimeString()}
          </div>
        </div>
      </div>
    </div>
  );
};
