import React from 'react';
import { AlertTriangle, X } from 'lucide-react';

interface ErrorDisplayProps {
    error: any;
    onClose: () => void;
}

export const ErrorDisplay: React.FC<ErrorDisplayProps> = ({ error, onClose }) => {
    if (!error) return null;

    return (
        <div className="absolute bottom-6 left-6 right-6 bg-gradient-to-r from-[#e06c75]/10 to-[#be5046]/10 backdrop-blur-xl border border-[#e06c75]/30 rounded-xl shadow-[0_16px_32px_-12px_rgba(0,0,0,0.4)] animate-blur-in z-20 max-h-[240px] overflow-hidden">
            <div className="flex items-start gap-3 p-4">
                <div className="p-2 rounded-lg bg-[#e06c75]/20 border border-[#e06c75]/30 text-[#e06c75] flex-shrink-0">
                    <AlertTriangle size={18} />
                </div>
                <div className="flex-1 min-w-0 overflow-y-auto custom-scrollbar max-h-[200px] pr-2">
                    {/* Main Error Message */}
                    {error.error && (
                        <p className="text-sm font-bold text-[#e06c75] leading-relaxed mb-2">
                            {error.error}
                        </p>
                    )}
                    
                    {/* Error Details */}
                    {error.details && (
                        <p className="text-xs text-[#dfe1e5]/80 leading-relaxed break-words">
                            {error.details}
                        </p>
                    )}
                    
                    {/* Detected Format */}
                    {error.detected_format && (
                        <div className="mt-2 flex items-center gap-2">
                            <span className="text-xs text-[#8b8d91]">Format:</span>
                            <span className="px-2 py-0.5 bg-[#e06c75]/20 text-[#e06c75] rounded text-xs font-mono font-bold">
                                {error.detected_format}
                            </span>
                        </div>
                    )}
                    
                    {/* Additional fields if any */}
                    {Object.keys(error).filter(k => !['error', 'details', 'detected_format'].includes(k)).length > 0 && (
                        <details className="mt-2">
                            <summary className="text-xs text-[#8b8d91] cursor-pointer hover:text-[#e06c75] transition-colors">
                                More details
                            </summary>
                            <pre className="mt-2 p-2 bg-[#1e1f22] border border-[#3d3f43] rounded text-[10px] text-[#dfe1e5] font-mono overflow-x-auto custom-scrollbar">
                                {JSON.stringify(
                                    Object.fromEntries(
                                        Object.entries(error).filter(([k]) => !['error', 'details', 'detected_format'].includes(k))
                                    ),
                                    null,
                                    2
                                )}
                            </pre>
                        </details>
                    )}
                </div>
                <button
                    onClick={onClose}
                    className="p-1.5 hover:bg-white/10 rounded-lg transition-all text-[#e06c75]/50 hover:text-[#e06c75] flex-shrink-0"
                >
                    <X size={16} />
                </button>
            </div>
        </div>
    );
};
