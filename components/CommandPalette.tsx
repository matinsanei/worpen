
import React, { useState, useEffect, useRef } from 'react';
import { Search, Terminal, Cpu, Shield, Zap, Server, Code, X } from 'lucide-react';

interface CommandPaletteProps {
    isOpen: boolean;
    onClose: () => void;
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose }) => {
    const [query, setQuery] = useState('');
    const [selectedIndex, setSelectedIndex] = useState(0);
    const inputRef = useRef<HTMLInputElement>(null);

    const commands = [
        { id: 'dashboard', label: 'Go to Dashboard', icon: Terminal, shortcut: 'G D' },
        { id: 'fleet', label: 'View Fleet Nodes', icon: Server, shortcut: 'G F' },
        { id: 'deploy', label: 'Trigger Deployment', icon: Zap, shortcut: 'Alt D' },
        { id: 'logs', label: 'System Logs', icon: Code, shortcut: 'Alt L' },
        { id: 'security', label: 'Security Scan', icon: Shield, shortcut: 'Alt S' },
        { id: 'kernel', label: 'Kernel Panic (Simulation)', icon: Cpu, shortcut: '!!!' },
    ];

    const filteredCommands = commands.filter(cmd =>
        cmd.label.toLowerCase().includes(query.toLowerCase())
    );

    useEffect(() => {
        if (isOpen) {
            setTimeout(() => inputRef.current?.focus(), 50);
        }
    }, [isOpen]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (!isOpen) return;

            if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedIndex(prev => (prev + 1) % filteredCommands.length);
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedIndex(prev => (prev - 1 + filteredCommands.length) % filteredCommands.length);
            } else if (e.key === 'Enter') {
                e.preventDefault();
                // Execute command (mock)
                console.log('Executing:', filteredCommands[selectedIndex]?.id);
                onClose();
            } else if (e.key === 'Escape') {
                onClose();
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [isOpen, filteredCommands, selectedIndex, onClose]);

    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 z-50 flex items-start justify-center pt-[20vh] bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
            <div className="w-full max-w-2xl bg-[#0a0a0a] border border-green-500/30 rounded-lg shadow-[0_0_50px_rgba(0,255,65,0.1)] overflow-hidden flex flex-col relative">
                {/* Scanline decoration */}
                <div className="absolute inset-0 pointer-events-none bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%)] bg-[length:100%_4px] opacity-20"></div>

                {/* Search Input */}
                <div className="flex items-center gap-3 px-4 py-4 border-b border-white/10">
                    <Search className="text-green-500 shrink-0" size={20} />
                    <input
                        ref={inputRef}
                        type="text"
                        className="flex-1 bg-transparent border-none outline-none text-white placeholder-gray-600 font-mono text-sm"
                        placeholder="Type a command or search..."
                        value={query}
                        onChange={(e) => {
                            setQuery(e.target.value);
                            setSelectedIndex(0);
                        }}
                    />
                    <div className="flex items-center gap-2">
                        <span className="text-[10px] text-gray-500 bg-white/5 px-1.5 py-0.5 rounded border border-white/10">ESC</span>
                        <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors">
                            <X size={18} />
                        </button>
                    </div>
                </div>

                {/* Results */}
                <div className="max-h-[300px] overflow-y-auto p-2 space-y-1 custom-scrollbar">
                    {filteredCommands.map((cmd, index) => (
                        <button
                            key={cmd.id}
                            onClick={() => onClose()}
                            className={`
                w-full flex items-center gap-3 px-3 py-3 rounded-md transition-all duration-100 group text-left
                ${index === selectedIndex
                                    ? 'bg-green-500/10 border border-green-500/20 shadow-[inset_0_0_10px_rgba(0,255,65,0.1)]'
                                    : 'hover:bg-white/5 border border-transparent'}
              `}
                            onMouseEnter={() => setSelectedIndex(index)}
                        >
                            <cmd.icon
                                size={18}
                                className={`${index === selectedIndex ? 'text-green-400' : 'text-gray-500'}`}
                            />
                            <span className={`text-sm font-medium ${index === selectedIndex ? 'text-green-50' : 'text-gray-400'}`}>
                                {cmd.label}
                            </span>
                            {cmd.shortcut && (
                                <span className="ml-auto text-xs font-mono text-gray-600 border border-white/5 px-1.5 rounded bg-black/50">
                                    {cmd.shortcut}
                                </span>
                            )}
                        </button>
                    ))}
                    {filteredCommands.length === 0 && (
                        <div className="p-8 text-center text-gray-600 font-mono text-sm">
                            No matching commands found.
                        </div>
                    )}
                </div>

                {/* Footer */}
                <div className="px-4 py-2 bg-white/5 border-t border-white/5 flex justify-between items-center text-[10px] text-gray-500 font-mono">
                    <span>WORPEN // CMD_PALETTE</span>
                    <span>v2.4.0-stable</span>
                </div>
            </div>
        </div>
    );
};
