import React, { useEffect, useRef, useState } from 'react';
import { X, Maximize2, Minimize2, Terminal as TerminalIcon } from 'lucide-react';
import { terminalApi } from '../api';

interface InteractiveTerminalProps {
  type: 'container' | 'agent';
  targetId: string;
  targetName: string;
  onClose: () => void;
}

export const InteractiveTerminal: React.FC<InteractiveTerminalProps> = ({
  type,
  targetId,
  targetName,
  onClose,
}) => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [output, setOutput] = useState<string[]>([]);
  const [input, setInput] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const outputRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Create WebSocket connection
    const socket = type === 'container' 
      ? terminalApi.connectContainer(targetId)
      : terminalApi.connectAgent(targetId);

    socket.onopen = () => {
      setIsConnected(true);
      setOutput((prev) => [...prev, `[SYSTEM] Connected to ${type}: ${targetName}`]);
    };

    socket.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        
        if (message.type === 'connected') {
          setOutput((prev) => [...prev, `[SYSTEM] Session ID: ${message.session_id}`]);
        } else if (message.type === 'output') {
          setOutput((prev) => [...prev, message.data]);
        } else if (message.type === 'echo') {
          // Command echo - already shown in output
        } else if (message.type === 'disconnected') {
          setOutput((prev) => [...prev, `[SYSTEM] ${message.message}`]);
          setIsConnected(false);
        }
      } catch (error) {
        // If not JSON, just append as text
        setOutput((prev) => [...prev, event.data]);
      }
    };

    socket.onerror = (error) => {
      setOutput((prev) => [...prev, `[ERROR] Connection error: ${error}`]);
      setIsConnected(false);
    };

    socket.onclose = () => {
      setOutput((prev) => [...prev, '[SYSTEM] Connection closed']);
      setIsConnected(false);
    };

    setWs(socket);

    return () => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.close();
      }
    };
  }, [type, targetId, targetName]);

  useEffect(() => {
    // Auto-scroll to bottom when new output arrives
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [output]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || !ws || ws.readyState !== WebSocket.OPEN) return;

    // Send command to WebSocket
    ws.send(input);
    setInput('');

    // Focus back on input
    inputRef.current?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'c' && e.ctrlKey) {
      // Ctrl+C - interrupt
      e.preventDefault();
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send('\x03'); // Send Ctrl+C signal
      }
    } else if (e.key === 'd' && e.ctrlKey) {
      // Ctrl+D - exit
      e.preventDefault();
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send('exit');
      }
    }
  };

  return (
    <div 
      className={`
        ${isFullscreen 
          ? 'fixed inset-0 z-50' 
          : 'w-full h-[600px]'
        }
        bg-black border border-green-500/30 rounded-lg overflow-hidden flex flex-col
        shadow-[0_0_30px_rgba(0,255,65,0.2)]
      `}
    >
      {/* Terminal Header */}
      <div className="bg-gradient-to-r from-green-500/10 to-transparent border-b border-green-500/30 px-4 py-3 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <TerminalIcon size={16} className="text-green-500" />
          <div>
            <div className="text-sm font-bold text-white font-mono">
              INTERACTIVE SHELL
            </div>
            <div className="text-xs text-gray-500 font-mono">
              {type.toUpperCase()}: {targetName}
            </div>
          </div>
          <div className={`
            ml-4 px-2 py-1 rounded text-[10px] font-mono
            ${isConnected 
              ? 'bg-green-500/10 text-green-400 border border-green-500/30' 
              : 'bg-red-500/10 text-red-400 border border-red-500/30'
            }
          `}>
            {isConnected ? '● CONNECTED' : '○ DISCONNECTED'}
          </div>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setIsFullscreen(!isFullscreen)}
            className="p-2 hover:bg-white/10 rounded transition-colors text-gray-400 hover:text-white"
            title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}
          >
            {isFullscreen ? <Minimize2 size={16} /> : <Maximize2 size={16} />}
          </button>
          <button
            onClick={onClose}
            className="p-2 hover:bg-red-500/20 rounded transition-colors text-gray-400 hover:text-red-400"
            title="Close terminal"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      {/* Terminal Output */}
      <div 
        ref={outputRef}
        className="flex-1 overflow-y-auto p-4 font-mono text-sm text-green-400 bg-black/50"
        style={{
          scrollbarWidth: 'thin',
          scrollbarColor: 'rgba(0,255,65,0.3) transparent',
        }}
      >
        {output.map((line, index) => (
          <div key={index} className="whitespace-pre-wrap break-all">
            {line}
          </div>
        ))}
      </div>

      {/* Terminal Input */}
      <form onSubmit={handleSubmit} className="border-t border-green-500/30 bg-black/80 p-4">
        <div className="flex items-center gap-2">
          <span className="text-green-500 font-mono text-sm">❯</span>
          <input
            ref={inputRef}
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            disabled={!isConnected}
            placeholder={isConnected ? "Type command..." : "Connecting..."}
            className="flex-1 bg-transparent border-none outline-none text-green-400 font-mono text-sm placeholder-gray-600 disabled:opacity-50"
            autoFocus
          />
        </div>
        <div className="mt-2 flex gap-4 text-[10px] text-gray-600 font-mono">
          <span>Ctrl+C: Interrupt</span>
          <span>Ctrl+D: Exit</span>
          <span>Enter: Execute</span>
        </div>
      </form>
    </div>
  );
};
