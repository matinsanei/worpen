
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
    switch(status) {
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
    <div className="h-full p-2 flex flex-col gap-4">
      {/* Header */}
      <div className="flex justify-between items-end border-b-2 border-green-800 pb-2">
        <div>
           <h1 className="text-2xl font-bold flex items-center gap-2">
             <Layers className="text-yellow-400" /> 
             ARTIFACT_NEXUS
           </h1>
           <p className="text-xs text-green-600">DEPENDENCY GRAPH & LAYER DEDUPLICATION</p>
        </div>
        <div className="flex gap-4 items-end">
            <div className="text-right">
                <div className="text-[10px] text-gray-400">STORAGE SAVED</div>
                <div className="text-xl font-mono text-green-400 font-bold">{calculateSavings()}</div>
            </div>
            <button 
                onClick={handleOptimize}
                disabled={optimized || analyzing}
                className={`px-4 py-2 border text-xs flex items-center gap-2 font-bold transition-all
                    ${optimized 
                        ? 'bg-black border-green-900 text-gray-500 cursor-not-allowed' 
                        : 'bg-green-900 border-green-500 text-white hover:bg-green-700 animate-pulse'
                    }`}
            >
            {analyzing ? <RefreshCw className="animate-spin" size={14}/> : <Share2 size={14} />}
            {analyzing ? 'ANALYZING LAYERS...' : optimized ? 'OPTIMIZED' : 'DEDUPLICATE LAYERS'}
            </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 h-full min-h-0">
        
        {/* Left: Global Cache Pool */}
        <div className="lg:col-span-1 h-full">
            <TerminalFrame title="SHARED_LAYER_REGISTRY" className="h-full">
                <div className="p-2 space-y-4">
                    <div className="flex items-center justify-between p-3 border border-green-800 bg-black/50">
                        <div className="flex items-center gap-3">
                            <Database size={20} className="text-green-500" />
                            <div>
                                <div className="text-xs text-gray-400">GLOBAL CACHE SIZE</div>
                                <div className="text-lg font-bold text-green-300">4.2 GB</div>
                            </div>
                        </div>
                        <div className="text-[10px] text-green-700">98% HIT RATE</div>
                    </div>

                    <div className="space-y-2">
                        <div className="text-[10px] font-bold text-gray-500 uppercase">Common Base Images</div>
                        {['node:18-alpine', 'postgres:13-alpine', 'nginx:latest'].map((img) => (
                            <div key={img} className="flex items-center justify-between p-2 border border-green-900/50 hover:bg-green-900/10 transition-colors cursor-pointer group">
                                <span className="text-xs font-mono text-green-400 group-hover:text-white flex items-center gap-2">
                                    <ShieldCheck size={12} /> {img}
                                </span>
                                <span className="text-[10px] text-gray-600">CACHED</span>
                            </div>
                        ))}
                    </div>

                    {optimized && (
                        <div className="space-y-2 animate-pulse">
                            <div className="text-[10px] font-bold text-green-500 uppercase mt-4">Newly Shared Layers</div>
                            <div className="p-2 border border-green-500/50 bg-green-900/10 text-xs text-green-300 flex justify-between">
                                <span>node_modules (npm install)</span>
                                <span>85MB</span>
                            </div>
                        </div>
                    )}
                </div>
            </TerminalFrame>
        </div>

        {/* Right: Visualization */}
        <div className="lg:col-span-2 h-full overflow-y-auto">
            <div className="flex flex-col gap-6">
                {MOCK_IMAGES.map((img) => (
                    <TerminalFrame key={img.id} title={`MANIFEST: ${img.name.toUpperCase()} [${img.tag}]`}>
                        <div className="flex flex-col gap-1 p-2">
                            <div className="flex justify-between text-[10px] text-gray-500 mb-2 border-b border-green-900/50 pb-1">
                                <span>TOTAL SIZE: {img.totalSize}</span>
                                <span>LAYERS: {img.layers.length}</span>
                            </div>
                            
                            {/* Layer Stack */}
                            <div className="flex flex-col-reverse gap-1">
                                {img.layers.map((layer, idx) => {
                                    const isRedundant = layer.status === 'REDUNDANT';
                                    const displayStatus = optimized && isRedundant ? 'LINKED (SHARED)' : layer.status;
                                    
                                    return (
                                        <div 
                                            key={idx} 
                                            className={`relative p-2 border flex justify-between items-center transition-all duration-1000
                                                ${getLayerColor(layer.status, optimized)}
                                                ${optimized && isRedundant ? 'translate-x-2 opacity-80' : ''}
                                            `}
                                        >
                                            <div className="flex items-center gap-3">
                                                <div className="font-mono text-[10px] opacity-70 w-24 truncate">{layer.hash}</div>
                                                <div className="text-xs font-bold flex items-center gap-2">
                                                    <Package size={12} />
                                                    {layer.instruction}
                                                </div>
                                            </div>
                                            <div className="flex items-center gap-4">
                                                <span className="font-mono text-xs">{layer.size}</span>
                                                <span className="text-[9px] font-bold px-1 border border-current opacity-70 w-24 text-center">
                                                    {displayStatus}
                                                </span>
                                            </div>

                                            {/* Visual connection line for optimization */}
                                            {isRedundant && !optimized && (
                                                <div className="absolute -right-12 text-red-500 text-[10px] animate-pulse flex items-center">
                                                    DUPLICATE <ArrowRight size={10} />
                                                </div>
                                            )}
                                        </div>
                                    );
                                })}
                            </div>
                        </div>
                    </TerminalFrame>
                ))}
            </div>
        </div>

      </div>
    </div>
  );
};
