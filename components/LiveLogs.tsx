import React, { useEffect, useRef } from 'react';
import { LogEntry } from '../types';
import { TerminalFrame } from './TerminalFrame';

interface LiveLogsProps {
  logs: LogEntry[];
}

export const LiveLogs: React.FC<LiveLogsProps> = ({ logs }) => {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs]);

  const getLevelColor = (level: string) => {
    switch (level) {
      case 'INFO': return 'text-blue-400';
      case 'WARN': return 'text-yellow-400';
      case 'ERROR': return 'text-red-500 font-bold';
      case 'SUCCESS': return 'text-green-400';
      default: return 'text-gray-400';
    }
  };

  return (
    <TerminalFrame title="SYSTEM_EVENTS_STREAM" className="h-full">
      <div className="font-mono text-xs md:text-sm space-y-1">
        {logs.map((log) => (
          <div key={log.id} className="flex flex-col md:flex-row gap-1 md:gap-4 border-b border-green-900/30 pb-1">
            <span className="text-green-700 min-w-[80px]">{log.timestamp}</span>
            <span className={`min-w-[60px] ${getLevelColor(log.level)}`}>[{log.level}]</span>
            <span className="text-green-600 min-w-[100px]">{log.source} &gt;</span>
            <span className="text-gray-300 break-all">{log.message}</span>
          </div>
        ))}
        <div ref={bottomRef} />
        <div className="animate-pulse text-green-500 mt-2">_</div>
      </div>
    </TerminalFrame>
  );
};