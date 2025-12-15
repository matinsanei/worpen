
import React, { useState, useEffect } from 'react';
import { ViewState } from './types';
import { Dashboard } from './views/Dashboard';
import { Fleet } from './views/Fleet';
import { DockerView } from './views/DockerView';
import { AutomationView } from './views/AutomationView';
import { SettingsView } from './views/SettingsView';
import { DependencyView } from './views/DependencyView';
import { CicdView } from './views/CicdView';
import { LeftSidebar } from './components/LeftSidebar';
import { NotificationPanel } from './components/NotificationPanel';
import {
  LayoutGrid, Server, AlertTriangle, Settings, Power,
  Box, Bot, Layers, GitBranch, Hexagon, Command,
  Bell, Search, Menu, X, ChevronRight, User,
  PanelRight, Wifi, Cpu, Activity
} from 'lucide-react';
import { TerminalFrame } from './components/TerminalFrame';
import { MOCK_INCIDENTS } from './constants';
import { NotificationProvider, useNotifications } from './components/NotificationSystem';
import { RightSidebar } from './components/RightSidebar';
import { CommandPalette } from './components/CommandPalette';

const IncidentsView: React.FC = () => (
  <div className="h-full p-6">
    <TerminalFrame title="INCIDENT LOGS" className="h-full">
      <div className="overflow-auto h-full">
        <table className="w-full text-left text-sm border-collapse">
          <thead className="bg-white/5 font-mono text-xs text-gray-400">
            <tr>
              <th className="p-4 font-normal">TIME</th>
              <th className="p-4 font-normal">SERVICE</th>
              <th className="p-4 font-normal">NODE</th>
              <th className="p-4 font-normal">ISSUE</th>
              <th className="p-4 font-normal">STATUS</th>
            </tr>
          </thead>
          <tbody className="text-gray-300 font-mono text-xs">
            {MOCK_INCIDENTS.map((inc) => (
              <tr key={inc.id} className="border-b border-white/5 hover:bg-white/5 transition-colors">
                <td className="p-4 text-gray-500">{inc.time}</td>
                <td className="p-4">{inc.service}</td>
                <td className="p-4 text-gray-500">{inc.node}</td>
                <td className="p-4 text-yellow-500">{inc.issue}</td>
                <td className="p-4">
                  <span className={`px-2 py-1 rounded text-[10px] ${inc.status === 'RESOLVED' ? 'bg-green-500/10 text-green-400' : 'bg-red-500/10 text-red-400'}`}>
                    {inc.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </TerminalFrame>
  </div>
);

const AppContent: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewState>('DASHBOARD');
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [rightSidebarOpen, setRightSidebarOpen] = useState(false);
  const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);

  // Inject startup notification
  const { addNotification } = useNotifications();
  useEffect(() => {
    // Simulate system boot notification
    setTimeout(() => {
      addNotification('INFO', 'SYSTEM_INIT', 'Worpen OS v2.4 initialized. All systems nominal.', 4000);
    }, 1000);
  }, [addNotification]);

  // Command Palette Shortcut
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setCommandPaletteOpen(prev => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const toggleSidebar = () => setSidebarOpen(!sidebarOpen);
  const toggleRightSidebar = () => setRightSidebarOpen(!rightSidebarOpen);

  return (
    <div className="flex h-screen w-full bg-[#030303] text-gray-200 selection:bg-green-500/30 overflow-hidden font-sans">

      <CommandPalette isOpen={commandPaletteOpen} onClose={() => setCommandPaletteOpen(false)} />

      {/* LEFT SIDEBAR */}
      <LeftSidebar
        isOpen={sidebarOpen}
        currentView={currentView}
        onNavigate={setCurrentView}
      />

      {/* MAIN CONTENT */}
      <div className="flex-1 flex flex-col min-w-0 bg-black relative">
        {/* Background Grid */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#111_1px,transparent_1px),linear-gradient(to_bottom,#111_1px,transparent_1px)] bg-[size:4rem_4rem] opacity-20 pointer-events-none"></div>
        <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(0,255,65,0.03),transparent_70%)] pointer-events-none"></div>

        {/* HEADER */}
        <header className="h-16 flex items-center justify-between px-6 border-b border-white/5 bg-[#030303]/80 backdrop-blur-sm sticky top-0 z-10 flex-shrink-0">
          <div className="flex items-center gap-4">
            <button onClick={toggleSidebar} className="text-gray-500 hover:text-white transition-colors">
              {sidebarOpen ? <Menu size={20} /> : <ChevronRight size={20} />}
            </button>

            {/* Breadcrumbs */}
            <div className="hidden md:flex items-center gap-2 text-sm text-gray-500 font-mono">
              <span className="text-gray-600">worpen</span>
              <span className="text-gray-700">/</span>
              <span className="text-green-500 tracking-wider uppercase">{currentView.toLowerCase()}</span>
            </div>
          </div>

          <div className="flex items-center gap-4">
            {/* Command Palette Trigger */}
            <button
              onClick={() => setCommandPaletteOpen(true)}
              className="hidden md:flex items-center gap-3 px-3 py-1.5 bg-white/5 rounded-md border border-white/10 text-gray-500 w-64 hover:bg-white/10 hover:border-white/20 transition-all group"
            >
              <Search size={14} className="group-hover:text-white transition-colors" />
              <span className="text-xs">Search resources...</span>
              <div className="ml-auto flex gap-1">
                <span className="text-[10px] border border-white/10 px-1.5 rounded bg-black text-gray-400">⌘K</span>
              </div>
            </button>

            {/* Actions */}
            <div className="flex items-center gap-2 border-l border-white/10 pl-4 ml-2">
              <NotificationPanel />

              <button
                onClick={toggleRightSidebar}
                className={`transition-colors p-2 rounded-md hover:bg-white/5 ${rightSidebarOpen ? 'text-green-400 bg-green-500/10' : 'text-gray-500 hover:text-white'}`}
                title="Toggle Command Center"
              >
                <PanelRight size={20} />
              </button>

              <button className="text-gray-500 hover:text-red-500 hover:bg-red-500/10 p-2 rounded-md transition-colors ml-2">
                <Power size={20} />
              </button>
            </div>
          </div>
        </header>

        {/* VIEWPORT */}
        <main className="flex-1 overflow-y-auto custom-scrollbar relative p-1">
          <div className="h-full">
            {currentView === 'DASHBOARD' && <Dashboard />}
            {currentView === 'FLEET' && <Fleet />}
            {currentView === 'DOCKER' && <DockerView />}
            {currentView === 'AUTOMATION' && <AutomationView />}
            {currentView === 'DEPENDENCY' && <DependencyView />}
            {currentView === 'CICD' && <CicdView />}
            {currentView === 'INCIDENTS' && <IncidentsView />}
            {currentView === 'SETTINGS' && <SettingsView />}
          </div>
        </main>

        {/* STATUS BAR */}
        <div className="h-6 bg-[#050505] border-t border-white/5 flex items-center justify-between px-4 text-[10px] font-mono text-gray-500 select-none z-20">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1.5 text-green-500">
              <div className="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"></div>
              SYSTEM_OPERATIONAL
            </span>
            <span className="flex items-center gap-1">
              <Wifi size={10} /> 12ms
            </span>
            <span className="flex items-center gap-1">
              <Cpu size={10} /> 14%
            </span>
          </div>
          <div className="flex items-center gap-4">
            <span>WS: CONNECTED</span>
            <span>BUILD: 2409.12</span>
          </div>
        </div>

      </div>

      {/* RIGHT SIDEBAR */}
      <RightSidebar isOpen={rightSidebarOpen} onClose={() => setRightSidebarOpen(false)} />
    </div>
  );
};

const App: React.FC = () => {
  return (
    <NotificationProvider>
      <AppContent />
    </NotificationProvider>
  );
};

export default App;
