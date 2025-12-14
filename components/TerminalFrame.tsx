
import React from 'react';

interface TerminalFrameProps {
  title?: string;
  children: React.ReactNode;
  className?: string;
  flashing?: boolean;
  action?: React.ReactNode;
  noPadding?: boolean;
}

export const TerminalFrame: React.FC<TerminalFrameProps> = ({ 
  title, 
  children, 
  className = "", 
  flashing = false, 
  action,
  noPadding = false 
}) => {
  return (
    <div className={`
      relative group flex flex-col
      bg-[#09090b]/80 backdrop-blur-md
      border border-green-900/30
      shadow-[0_0_20px_rgba(0,0,0,0.5)]
      transition-all duration-300 hover:border-green-500/30 hover:shadow-[0_0_30px_rgba(0,255,65,0.05)]
      rounded-sm overflow-hidden
      ${flashing ? 'animate-pulse border-red-500/50' : ''}
      ${className}
    `}>
      {/* High-tech accent lines */}
      <div className="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-green-500/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
      
      {title && (
        <div className="flex items-center justify-between px-4 py-3 border-b border-white/5 bg-white/[0.02]">
          <div className="flex items-center gap-3">
             <div className={`w-1 h-3 ${flashing ? 'bg-red-500' : 'bg-green-500'} rounded-sm`}></div>
             <span className="font-mono text-[10px] md:text-xs font-bold tracking-[0.2em] text-gray-400 uppercase group-hover:text-green-400 transition-colors">
                {title}
             </span>
          </div>
          {action && <div className="flex items-center">{action}</div>}
        </div>
      )}
      
      <div className={`flex-1 relative ${noPadding ? '' : 'p-4'}`}>
        {children}
      </div>

      {/* Decorative Corners */}
      <div className="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-green-500/30 opacity-50"></div>
      <div className="absolute bottom-0 left-0 w-2 h-2 border-b border-l border-green-500/30 opacity-50"></div>
    </div>
  );
};
