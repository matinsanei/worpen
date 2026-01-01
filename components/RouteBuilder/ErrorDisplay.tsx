import React from 'react';
import { XCircle, X } from 'lucide-react';

interface ErrorDisplayProps {
    error: any;
    onClose: () => void;
}

export const ErrorDisplay: React.FC<ErrorDisplayProps> = ({ error, onClose }) => {
    if (!error) return null;

    return (
        <div className="absolute bottom-6 left-6 right-6 bg-red-500/10 backdrop-blur-xl border border-red-500/20 p-5 rounded-2xl shadow-[0_24px_48px_-12px_rgba(0,0,0,0.5)] animate-blur-in z-20 max-h-[300px] overflow-y-auto custom-scrollbar">
            <div className="flex items-start gap-5">
                <div className="p-2 rounded-xl bg-red-500/20 border border-red-500/20 text-red-500 flex-shrink-0">
                    <XCircle size={20} />
                </div>
                <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between mb-2">
                        <p className="text-[11px] font-black uppercase tracking-[2px] text-red-500 opacity-80">
                            Registration Error
                        </p>
                        <button
                            onClick={onClose}
                            className="hover:bg-white/10 rounded-xl p-1.5 transition-all text-red-200/50 hover:text-red-200"
                        >
                            <X size={16} />
                        </button>
                    </div>
                    
                    {/* Main Error Message */}
                    {error.error && (
                        <p className="text-[13px] font-bold text-red-200/90 leading-relaxed mb-2">
                            {error.error}
                        </p>
                    )}
                    
                    {/* Error Details */}
                    {error.details && (
                        <div className="mt-3 p-3 bg-black/20 border border-red-500/10 rounded-lg">
                            <p className="text-[10px] font-bold uppercase tracking-wider text-red-400/70 mb-1.5">
                                Details:
                            </p>
                            <p className="text-[12px] text-red-100/80 font-mono leading-relaxed break-words whitespace-pre-wrap">
                                {error.details}
                            </p>
                        </div>
                    )}
                    
                    {/* Detected Format */}
                    {error.detected_format && (
                        <div className="mt-2 flex items-center gap-2 text-[11px]">
                            <span className="text-red-300/60 font-medium">Detected Format:</span>
                            <span className="px-2 py-0.5 bg-red-500/20 text-red-300 rounded-md font-mono font-bold">
                                {error.detected_format}
                            </span>
                        </div>
                    )}
                    
                    {/* Additional fields if any */}
                    {Object.keys(error).filter(k => !['error', 'details', 'detected_format'].includes(k)).length > 0 && (
                        <details className="mt-3">
                            <summary className="text-[10px] font-bold uppercase tracking-wider text-red-400/70 cursor-pointer hover:text-red-400 transition-colors">
                                Additional Info
                            </summary>
                            <pre className="mt-2 p-3 bg-black/20 border border-red-500/10 rounded-lg text-[11px] text-red-100/70 font-mono overflow-x-auto custom-scrollbar">
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
            </div>
        </div>
    );
};
