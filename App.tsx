
import React, { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route, useNavigate, useLocation } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import { PageTransition } from './components/PageTransition';
import { ViewState } from './types';
import { Dashboard } from './views/Dashboard';
import { Fleet } from './views/Fleet';
import { DockerView } from './views/DockerView';
import { AutomationView } from './views/AutomationView';
import { SettingsView } from './views/SettingsView';
import { DependencyView } from './views/DependencyView';
import { CicdView } from './views/CicdView';
import { DynamicRoutesView } from './views/DynamicRoutesView';
import { IncidentsView } from './views/IncidentsView';
import { FunctionsView } from './views/FunctionsView';
import { StorageView } from './views/StorageView';
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


const AppContent: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(() => {
    const saved = localStorage.getItem('worpen_sidebar_open');
    return saved !== null ? JSON.parse(saved) : true;
  });
  const [rightSidebarOpen, setRightSidebarOpen] = useState(() => {
    const saved = localStorage.getItem('worpen_right_sidebar_open');
    return saved !== null ? JSON.parse(saved) : false;
  });
  const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);

  useEffect(() => {
    localStorage.setItem('worpen_sidebar_open', JSON.stringify(sidebarOpen));
  }, [sidebarOpen]);

  useEffect(() => {
    localStorage.setItem('worpen_right_sidebar_open', JSON.stringify(rightSidebarOpen));
  }, [rightSidebarOpen]);

  // Map URL paths to ViewState
  const pathToView: Record<string, ViewState> = {
    '/': 'DASHBOARD',
    '/routes': 'DYNAMIC_ROUTES',
    '/functions': 'FUNCTIONS',
    '/storage': 'STORAGE',
    '/traces': 'CICD',
    '/errors': 'INCIDENTS',
    '/settings': 'SETTINGS',
    // Legacy routes (kept for compatibility)
    '/fleet': 'FLEET',
    '/containers': 'DOCKER',
    '/auto-healing': 'AUTOMATION',
    '/artifacts': 'DEPENDENCY',
    '/pipeline': 'CICD',
    '/route-builder': 'DYNAMIC_ROUTES',
    '/incidents': 'INCIDENTS',
  };

  const currentView = pathToView[location.pathname] || 'DASHBOARD';

  const handleViewChange = (view: ViewState) => {
    const viewToPath: Record<ViewState, string> = {
      'DASHBOARD': '/',
      'DYNAMIC_ROUTES': '/routes',
      'FUNCTIONS': '/functions',
      'STORAGE': '/storage',
      'CICD': '/traces',
      'INCIDENTS': '/errors',
      'SETTINGS': '/settings',
      // Legacy views
      'FLEET': '/fleet',
      'DOCKER': '/containers',
      'AUTOMATION': '/auto-healing',
      'DEPENDENCY': '/artifacts',
    };
    navigate(viewToPath[view]);
  };

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
    <div className="flex h-screen w-full bg-[var(--bg-deep)] text-[var(--text-main)] selection:bg-[#3574f030] overflow-hidden">

      <CommandPalette isOpen={commandPaletteOpen} onClose={() => setCommandPaletteOpen(false)} />

      {/* LEFT SIDEBAR */}
      <LeftSidebar
        isOpen={sidebarOpen}
        currentView={currentView}
        onViewChange={handleViewChange}
      />

      {/* MAIN CONTENT */}
      <div className="flex-1 flex flex-col min-w-0 bg-transparent relative">

        {/* HEADER */}
        <header className="h-12 flex items-center justify-between px-4 border-b border-[#43454a] bg-[#2b2d30] sticky top-0 z-10 flex-shrink-0">
          <div className="flex items-center gap-4">
            <button
              onClick={toggleSidebar}
              className="text-[#6e7073] hover:text-[#dfe1e5] transition-colors p-1 hover:bg-[#393b40] rounded-[var(--radius)]"
            >
              <Menu size={16} />
            </button>

            {/* Breadcrumbs */}
            <div className="hidden md:flex items-center gap-2 text-[12px] font-medium text-[#6e7073]">
              <span>worpen</span>
              <span className="text-[#43454a]">/</span>
              <span className="text-[#3574f0] font-semibold">{currentView}</span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            {/* Search Bar */}
            <button
              onClick={() => setCommandPaletteOpen(true)}
              className="hidden md:flex items-center gap-2 px-3 py-1.5 bg-[#1e1f22] rounded-[var(--radius)] border border-[#43454a] text-[#6e7073] w-64 hover:border-[#6a6e75] transition-all group"
            >
              <Search size={14} className="group-hover:text-[#dfe1e5]" />
              <span className="text-[12px]">Search ...</span>
              <div className="ml-auto flex gap-1">
                <span className="text-[10px] border border-[#43454a] px-1 rounded bg-[#2b2d30] text-[#6e7073]">⌘K</span>
              </div>
            </button>

            {/* Actions */}
            <div className="flex items-center gap-1 border-l border-[#43454a] pl-3 ml-1">
              <NotificationPanel />

              <button
                onClick={toggleRightSidebar}
                className={`transition-colors p-1.5 rounded-[var(--radius)] ${rightSidebarOpen ? 'text-[#3574f0] bg-[#3574f020]' : 'text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40]'}`}
                title="Command Center"
              >
                <PanelRight size={18} />
              </button>

              <button className="text-[#6e7073] hover:text-[#e06c75] hover:bg-[#e06c7515] p-1.5 rounded-[var(--radius)] transition-colors">
                <Power size={18} />
              </button>
            </div>
          </div>
        </header>

        {/* VIEWPORT */}
        <main className={`flex-1 ${['/', '/traces', '/artifacts'].includes(location.pathname) ? 'overflow-y-auto' : 'overflow-hidden'} custom-scrollbar relative bg-[#1e1f22] m-1.5 rounded-[var(--radius)] border border-[#43454a] shadow-inner overflow-x-hidden`}>
          <AnimatePresence mode="wait">
            <PageTransition key={location.pathname} className="h-full">
              <Routes location={location}>
                {/* Primary IDE Routes */}
                <Route path="/" element={<Dashboard />} />
                <Route path="/routes" element={<DynamicRoutesView />} />
                <Route path="/functions" element={<FunctionsView />} />
                <Route path="/storage" element={<StorageView />} />
                <Route path="/traces" element={<CicdView />} />
                <Route path="/errors" element={<IncidentsView />} />
                <Route path="/settings" element={<SettingsView />} />
                
                {/* Legacy Routes (for backward compatibility) */}
                <Route path="/fleet" element={<Fleet />} />
                <Route path="/containers" element={<DockerView />} />
                <Route path="/auto-healing" element={<AutomationView />} />
                <Route path="/artifacts" element={<DependencyView />} />
                <Route path="/pipeline" element={<CicdView />} />
                <Route path="/route-builder" element={<DynamicRoutesView />} />
                <Route path="/incidents" element={<IncidentsView />} />
              </Routes>
            </PageTransition>
          </AnimatePresence>
        </main>

        {/* STATUS BAR */}
        <div className="h-6 bg-[#2b2d30] border-t border-[#43454a] flex items-center justify-between px-3 text-[11px] font-medium text-[#6e7073] select-none z-20">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1.5 text-[#59a869]">
              <div className="w-2 h-2 bg-[#59a869] rounded-full"></div>
              OPERATIONAL
            </span>
            <span className="flex items-center gap-1 hover:text-[#dfe1e5] cursor-pointer">
              <Wifi size={12} /> 12ms
            </span>
            <span className="flex items-center gap-1 hover:text-[#dfe1e5] cursor-pointer">
              <Cpu size={12} /> 14%
            </span>
          </div>
          <div className="flex items-center gap-4">
            <span className="hover:text-[#dfe1e5] cursor-pointer">WS: CONNECTED</span>
            <span className="hover:text-[#dfe1e5] cursor-pointer border-l border-[#43454a] pl-4">BUILD: 2409.12</span>
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
    <BrowserRouter>
      <NotificationProvider>
        <AppContent />
      </NotificationProvider>
    </BrowserRouter>
  );
};

export default App;
