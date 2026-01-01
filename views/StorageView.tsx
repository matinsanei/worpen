import React from 'react';
import { Database, Table, Key, Clock, HardDrive } from 'lucide-react';

export const StorageView: React.FC = () => {
    return (
        <div className="h-full flex flex-col bg-[#1e1f22] text-[#dfe1e5]">
            {/* Header */}
            <div className="border-b border-[#43454a] bg-[#2b2d30] p-6">
                <div className="flex items-center gap-3 mb-2">
                    <div className="w-10 h-10 rounded-lg bg-[#3574f020] border border-[#3574f040] flex items-center justify-center">
                        <Database size={20} className="text-[#3574f0]" />
                    </div>
                    <div>
                        <h1 className="text-2xl font-bold text-[#dfe1e5]">Storage</h1>
                        <p className="text-sm text-[#6e7073]">Manage databases, tables, and key-value stores</p>
                    </div>
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-y-auto p-6">
                <div className="max-w-6xl mx-auto">
                    {/* Coming Soon Banner */}
                    <div className="bg-[#2b2d30] border border-[#43454a] rounded-lg p-8 text-center mb-6">
                        <div className="w-16 h-16 rounded-full bg-[#3574f020] border border-[#3574f040] flex items-center justify-center mx-auto mb-4">
                            <HardDrive size={32} className="text-[#3574f0]" />
                        </div>
                        <h2 className="text-xl font-bold text-[#dfe1e5] mb-2">Storage Module Coming Soon</h2>
                        <p className="text-[#6e7073] max-w-md mx-auto">
                            Integrated database management, key-value stores, and persistent storage for your Worpen applications.
                        </p>
                    </div>

                    {/* Feature Preview Grid */}
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div className="bg-[#2b2d30] border border-[#43454a] rounded-lg p-6 hover:border-[#3574f060] transition-all">
                            <div className="w-12 h-12 rounded-lg bg-[#3574f010] border border-[#3574f030] flex items-center justify-center mb-4">
                                <Table size={20} className="text-[#3574f0]" />
                            </div>
                            <h3 className="font-semibold text-[#dfe1e5] mb-2">SQL Tables</h3>
                            <p className="text-sm text-[#6e7073]">
                                Create and manage relational database tables with schema definitions and migrations.
                            </p>
                        </div>

                        <div className="bg-[#2b2d30] border border-[#43454a] rounded-lg p-6 hover:border-[#3574f060] transition-all">
                            <div className="w-12 h-12 rounded-lg bg-[#59a86910] border border-[#59a86930] flex items-center justify-center mb-4">
                                <Key size={20} className="text-[#59a869]" />
                            </div>
                            <h3 className="font-semibold text-[#dfe1e5] mb-2">Key-Value Store</h3>
                            <p className="text-sm text-[#6e7073]">
                                Fast in-memory key-value storage for caching, sessions, and temporary data.
                            </p>
                        </div>

                        <div className="bg-[#2b2d30] border border-[#43454a] rounded-lg p-6 hover:border-[#3574f060] transition-all">
                            <div className="w-12 h-12 rounded-lg bg-[#e5c07b10] border border-[#e5c07b30] flex items-center justify-center mb-4">
                                <Clock size={20} className="text-[#e5c07b]" />
                            </div>
                            <h3 className="font-semibold text-[#dfe1e5] mb-2">Time-Series Data</h3>
                            <p className="text-sm text-[#6e7073]">
                                Store and query time-series data for logs, metrics, and event tracking.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
