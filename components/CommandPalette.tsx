
import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { Search, Activity, Network, Code2, List, AlertCircle, Settings, X } from 'lucide-react';

interface CommandPaletteProps {
    isOpen: boolean;
    onClose: () => void;
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose }) => {
    const navigate = useNavigate();
    const [query, setQuery] = useState('');
    const [selectedIndex, setSelectedIndex] = useState(0);
    const inputRef = useRef<HTMLInputElement>(null);

    const commands = [
        { id: 'overview', label: 'Go to Overview', path: '/', icon: Activity, shortcut: 'G O' },
        { id: 'routes', label: 'API Routes Builder', path: '/routes', icon: Network, shortcut: 'G R' },
        { id: 'functions', label: 'Functions Library', path: '/functions', icon: Code2, shortcut: 'G F' },
        { id: 'traces', label: 'Live Traces', path: '/traces', icon: List, shortcut: 'G T' },
        { id: 'errors', label: 'Error Logs', path: '/errors', icon: AlertCircle, shortcut: 'G E' },
        { id: 'settings', label: 'Settings', path: '/settings', icon: Settings, shortcut: 'G ,' },
    ];

    const runCommand = (path?: string) => {
        if (path) navigate(path);
        onClose();
    };

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
                runCommand(filteredCommands[selectedIndex]?.path);
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
            <div className="w-full max-w-2xl bg-[#1e1f22] border border-[#43454a] rounded-lg shadow-[0_0_50px_rgba(232,93,38,0.12)] overflow-hidden flex flex-col relative">
                {/* Scanline decoration */}
                <div className="absolute inset-0 pointer-events-none bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%)] bg-[length:100%_4px] opacity-20"></div>

                {/* Search Input */}
                <div className="flex items-center gap-3 px-4 py-4 border-b border-[#43454a]">
                    <Search className="text-[#e85d26] shrink-0" size={20} />
                    <input
                        ref={inputRef}
                        type="text"
                        className="flex-1 bg-transparent border-none outline-none text-[#dfe1e5] placeholder-[#6e7073] font-mono text-sm"
                        placeholder="Type a command or search..."
                        value={query}
                        onChange={(e) => {
                            setQuery(e.target.value);
                            setSelectedIndex(0);
                        }}
                    />
                    <div className="flex items-center gap-2">
                        <span className="text-[10px] text-[#6e7073] bg-[#2b2d30] px-1.5 py-0.5 rounded border border-[#43454a]">ESC</span>
                        <button onClick={onClose} className="text-[#6e7073] hover:text-[#dfe1e5] transition-colors">
                            <X size={18} />
                        </button>
                    </div>
                </div>

                {/* Results */}
                <div className="max-h-[300px] overflow-y-auto p-2 space-y-1 custom-scrollbar">
                    {filteredCommands.map((cmd, index) => (
                        <button
                            key={cmd.id}
                            onClick={() => runCommand(cmd.path)}
                            className={`
                w-full flex items-center gap-3 px-3 py-3 rounded-md transition-all duration-100 group text-left
                ${index === selectedIndex
                                    ? 'bg-[#e85d2615] border border-[#e85d2640] shadow-[inset_0_0_10px_rgba(232,93,38,0.08)]'
                                    : 'hover:bg-[#2b2d30] border border-transparent'}
              `}
                            onMouseEnter={() => setSelectedIndex(index)}
                        >
                            <cmd.icon
                                size={18}
                                className={`${index === selectedIndex ? 'text-[#e85d26]' : 'text-[#6e7073]'}`}
                            />
                            <span className={`text-sm font-medium ${index === selectedIndex ? 'text-[#dfe1e5]' : 'text-[#6e7073]'}`}>
                                {cmd.label}
                            </span>
                            {cmd.shortcut && (
                                <span className="ml-auto text-xs font-mono text-[#6e7073] border border-[#43454a] px-1.5 rounded bg-[#2b2d30]">
                                    {cmd.shortcut}
                                </span>
                            )}
                        </button>
                    ))}
                    {filteredCommands.length === 0 && (
                        <div className="p-8 text-center text-[#6e7073] font-mono text-sm">
                            No matching commands found.
                        </div>
                    )}
                </div>

                {/* Footer */}
                <div className="px-4 py-2 bg-[#2b2d30] border-t border-[#43454a] flex justify-between items-center text-[10px] text-[#6e7073] font-mono">
                    <span>WORPEN // CMD_PALETTE</span>
                    <span>v2.4.0-stable</span>
                </div>
            </div>
        </div>
    );
};
