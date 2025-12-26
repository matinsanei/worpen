
import React, { useState } from 'react';
import { TerminalFrame } from '../components/TerminalFrame';
import { MOCK_IMAGES } from '../constants';
import { Layers, Database, RefreshCw, Share2, ArrowRight, Package, ShieldCheck } from 'lucide-react';

export const DependencyView: React.FC = () => {
    const [optimized, setOptimized] = useState(false);
    const [analyzing, setAnalyzing] = useState(false);

    const handleOptimize = () => {
        setAnalyzing(true);
        setTimeout(() => {
            setAnalyzing(false);
            setOptimized(true);
        }, 1500);
    };

    const getLayerColor = (status: string, isOptimized: boolean) => {
        if (isOptimized && status === 'REDUNDANT') return 'text-green-500 border-green-700 bg-green-900/20'; // Converted to shared
        switch (status) {
            case 'CACHED': return 'text-green-500 border-green-700 bg-green-900/20';
            case 'DOWNLOADING': return 'text-blue-400 border-blue-700 bg-blue-900/20';
            case 'REDUNDANT': return 'text-red-400 border-red-700 bg-red-900/20';
            default: return 'text-gray-500';
        }
    };

    const calculateSavings = () => {
        // Mock calculation
        return optimized ? "125MB" : "0B";
    };

    return (
        <div className="min-h-full p-6 flex flex-col gap-6 max-w-[1600px] mx-auto">
            {/* Header */}
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#43454a] pb-6">
                <div>
                    <h1 className="text-2xl font-bold flex items-center gap-3 text-[#dfe1e5] tracking-tight">
                        <div className="p-2 rounded-[var(--radius)] bg-[#f2c55c15] border border-[#f2c55c20]">
                            <Layers className="text-[#f2c55c]" size={20} />
                        </div>
                        Artifact Nexus
                    </h1>
                    <p className="text-[11px] text-[#6e7073] font-bold uppercase tracking-wider mt-1">Dependency Graph & Layer Deduplication</p>
                </div>
                <div className="flex gap-6 items-center">
                    <div className="text-right">
                        <div className="text-[10px] text-[#6e7073] font-bold uppercase tracking-wider mb-1">Storage Saved</div>
                        <div className="text-2xl font-bold text-[#59a869] tracking-tighter">{calculateSavings()}</div>
                    </div>
                    <button
                        onClick={handleOptimize}
                        disabled={optimized || analyzing}
                        className={`px-5 py-2.5 rounded-[var(--radius)] text-[11px] flex items-center gap-2 font-bold transition-all uppercase tracking-tight shadow-lg 
                    ${optimized
                                ? 'bg-[#2b2d30] border border-[#43454a] text-[#6e7073] cursor-not-allowed'
                                : 'bg-[#3574f0] hover:bg-[#3574f0e0] text-white border border-[#3574f0] active:scale-95'
                            }`}
                    >
                        {analyzing ? <RefreshCw className="animate-spin" size={14} /> : <Share2 size={14} />}
                        {analyzing ? 'Analyzing...' : optimized ? 'Optimized' : 'Deduplicate'}
                    </button>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">

                {/* Left: Global Cache Pool */}
                <div className="lg:col-span-4 h-full">
                    <div className="jb-card flex flex-col h-full overflow-hidden shadow-2xl">
                        <div className="flex items-center gap-2 px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
                            <Database size={14} className="text-[#3574f0]" />
                            <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Shared Layer Registry</span>
                        </div>
                        <div className="p-4 space-y-5 overflow-auto custom-scrollbar">
                            <div className="flex items-center justify-between p-4 rounded-[var(--radius)] border border-[#43454a] bg-[#1e1f22]/50 shadow-inner">
                                <div className="flex items-center gap-3">
                                    <div className="p-2 rounded-full bg-[#59a86915]">
                                        <ShieldCheck size={20} className="text-[#59a869]" />
                                    </div>
                                    <div>
                                        <div className="text-[10px] text-[#6e7073] font-bold uppercase tracking-wider">Global Cache Pool</div>
                                        <div className="text-xl font-bold text-[#dfe1e5]">4.2 GB</div>
                                    </div>
                                </div>
                                <div className="text-[10px] font-bold text-[#59a869] bg-[#59a86910] px-2 py-1 rounded">98% HIT RATE</div>
                            </div>

                            <div className="space-y-2">
                                <div className="text-[10px] font-bold text-[#6e7073] uppercase tracking-widest px-1">Common Base Images</div>
                                {['node:18-alpine', 'postgres:13-alpine', 'nginx:latest'].map((img) => (
                                    <div key={img} className="flex items-center justify-between p-3 rounded-[var(--radius)] border border-[#43454a]/50 hover:bg-[#393b4040] hover:border-[#6a6e75] transition-all cursor-pointer group">
                                        <span className="text-[12px] font-bold text-[#dfe1e5] group-hover:text-[#3574f0] transition-colors flex items-center gap-3">
                                            <Package size={14} className="opacity-70" /> {img}
                                        </span>
                                        <span className="text-[10px] font-bold text-[#6e7073] uppercase tracking-tighter">CACHED</span>
                                    </div>
                                ))}
                            </div>

                            {optimized && (
                                <div className="space-y-2 animate-in fade-in slide-in-from-bottom-2 duration-500">
                                    <div className="text-[10px] font-bold text-[#59a869] uppercase tracking-widest px-1 mt-6 flex items-center gap-2">
                                        <div className="w-1.5 h-1.5 rounded-full bg-[#59a869] animate-pulse"></div>
                                        Newly Shared Layers
                                    </div>
                                    <div className="p-3 rounded-[var(--radius)] border border-[#59a86930] bg-[#59a86908] text-[12px] text-[#dfe1e5] flex justify-between items-center shadow-lg shadow-[#59a86905]">
                                        <span className="font-bold">node_modules (npm install)</span>
                                        <span className="font-mono text-[#59a869] font-bold">85MB</span>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* Right: Visualization */}
                <div className="lg:col-span-8 h-full overflow-y-auto custom-scrollbar pr-2">
                    <div className="flex flex-col gap-8 pb-6">
                        {MOCK_IMAGES.map((img) => (
                            <div key={img.id} className="jb-card overflow-hidden shadow-2xl">
                                <div className="flex items-center justify-between px-4 py-3 border-b border-[#43454a] bg-[#2b2d30]/50">
                                    <div className="flex items-center gap-2">
                                        <Package size={14} className="text-[#f2c55c]" />
                                        <span className="text-[11px] font-bold text-[#dfe1e5] uppercase tracking-wider">Manifest: {img.name} <span className="text-[#6e7073] ml-1">[{img.tag}]</span></span>
                                    </div>
                                    <div className="flex gap-4 text-[10px] font-bold text-[#6e7073] uppercase tracking-tighter">
                                        <span>SIZE: {img.totalSize}</span>
                                        <span>LAYERS: {img.layers.length}</span>
                                    </div>
                                </div>
                                <div className="flex flex-col gap-1.5 p-4 bg-[#1e1f22]/30">
                                    {/* Layer Stack */}
                                    <div className="flex flex-col-reverse gap-2">
                                        {img.layers.map((layer, idx) => {
                                            const isRedundant = layer.status === 'REDUNDANT';
                                            const displayStatus = optimized && isRedundant ? 'LINKED (SHARED)' : layer.status;

                                            const getJBStatusColor = (status: string) => {
                                                if (optimized && isRedundant) return 'border-[#59a86940] text-[#59a869] bg-[#59a86910] shadow-[0_0_15px_rgba(89,168,105,0.05)]';
                                                switch (status) {
                                                    case 'CACHED': return 'border-[#59a86940] text-[#59a869] bg-[#59a86910]';
                                                    case 'DOWNLOADING': return 'border-[#3574f040] text-[#3574f0] bg-[#3574f010]';
                                                    case 'REDUNDANT': return 'border-[#e06c7540] text-[#e06c75] bg-[#e06c7510]';
                                                    default: return 'border-[#43454a] text-[#6e7073] bg-[#2b2d30]';
                                                }
                                            };

                                            return (
                                                <div
                                                    key={idx}
                                                    className={`relative p-3 rounded-[var(--radius)] border flex justify-between items-center transition-all duration-700
                                                ${getJBStatusColor(layer.status)}
                                                ${optimized && isRedundant ? 'translate-x-3 opacity-90 scale-[1.01]' : ''}
                                            `}
                                                >
                                                    <div className="flex items-center gap-4">
                                                        <div className="font-mono text-[10px] text-[#6e7073] w-28 truncate font-bold">{layer.hash}</div>
                                                        <div className="text-[12px] font-bold flex items-center gap-3">
                                                            <div className="w-1.5 h-1.5 rounded-full bg-current opacity-40"></div>
                                                            {layer.instruction}
                                                        </div>
                                                    </div>
                                                    <div className="flex items-center gap-6">
                                                        <span className="font-mono text-[11px] font-bold opacity-80">{layer.size}</span>
                                                        <span className="text-[9px] font-bold px-2 py-0.5 rounded border border-current opacity-70 w-24 text-center tracking-tighter uppercase">
                                                            {displayStatus}
                                                        </span>
                                                    </div>

                                                    {/* Visual connection line for optimization */}
                                                    {isRedundant && !optimized && (
                                                        <div className="absolute -right-16 text-[#e06c75] text-[10px] font-bold animate-pulse flex items-center gap-1.5">
                                                            DUPLICATE <ArrowRight size={12} />
                                                        </div>
                                                    )}
                                                </div>
                                            );
                                        })}
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>

            </div>
        </div>
    );
};
