import React from 'react';
import { CheckCircle, X } from 'lucide-react';

interface SuccessDisplayProps {
    message: string;
    onClose: () => void;
}

export const SuccessDisplay: React.FC<SuccessDisplayProps> = ({ message, onClose }) => {
    return (
        <div className="absolute bottom-6 left-6 right-6 bg-gradient-to-r from-[#59a869]/10 to-[#4a9158]/10 backdrop-blur-xl border border-[#59a869]/30 p-4 rounded-xl shadow-[0_16px_32px_-12px_rgba(0,0,0,0.4)] animate-blur-in z-20">
            <div className="flex items-center gap-3">
                <div className="p-2 rounded-lg bg-[#59a869]/20 border border-[#59a869]/30 text-[#59a869] flex-shrink-0">
                    <CheckCircle size={18} />
                </div>
                <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-[#59a869] leading-relaxed">
                        {message}
                    </p>
                </div>
                <button
                    onClick={onClose}
                    className="p-1.5 hover:bg-white/10 rounded-lg transition-all text-[#59a869]/50 hover:text-[#59a869] flex-shrink-0"
                >
                    <X size={16} />
                </button>
            </div>
        </div>
    );
};
