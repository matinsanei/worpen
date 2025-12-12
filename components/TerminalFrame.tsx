import React from 'react';

interface TerminalFrameProps {
  title: string;
  children: React.ReactNode;
  className?: string;
  flashing?: boolean;
}

export const TerminalFrame: React.FC<TerminalFrameProps> = ({ title, children, className = "", flashing = false }) => {
  return (
    <div className={`relative border-2 border-green-800 bg-black/80 flex flex-col ${className} ${flashing ? 'animate-pulse border-red-800' : ''}`}>
      <div className="flex items-center justify-between bg-green-900/20 border-b border-green-800 px-2 py-1 select-none">
        <span className={`text-xs font-bold uppercase tracking-widest ${flashing ? 'text-red-500' : 'text-green-500'}`}>
           ╔ {title}
        </span>
        <div className="flex space-x-1">
           <div className="w-2 h-2 bg-green-700/50"></div>
           <div className="w-2 h-2 bg-green-700/50"></div>
           <div className="w-2 h-2 bg-green-500"></div>
        </div>
      </div>
      <div className="p-4 overflow-auto flex-1 relative">
        {children}
        {/* Corner Decals */}
        <div className="absolute bottom-0 right-0 w-4 h-4 border-b-2 border-r-2 border-green-600/50"></div>
        <div className="absolute bottom-0 left-0 w-4 h-4 border-b-2 border-l-2 border-green-600/50"></div>
      </div>
    </div>
  );
};