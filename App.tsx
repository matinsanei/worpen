
import React, { useState, useEffect, Suspense, lazy } from 'react';
import { BrowserRouter, Routes, Route, useNavigate, useLocation } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import { PageTransition } from './components/PageTransition';
import { ViewState } from './types';
// Heavy views are code-split: Monaco editor (~4MB) and charts only load
// when their route opens, keeping the first paint small.
const Dashboard = lazy(() => import('./views/Dashboard').then(m => ({ default: m.Dashboard })));
const CicdView = lazy(() => import('./views/CicdView').then(m => ({ default: m.CicdView })));
const DynamicRoutesView = lazy(() => import('./views/DynamicRoutesView').then(m => ({ default: m.DynamicRoutesView })));
const IncidentsView = lazy(() => import('./views/IncidentsView').then(m => ({ default: m.IncidentsView })));
const FunctionsView = lazy(() => import('./views/FunctionsView').then(m => ({ default: m.FunctionsView })));
const SettingsView = lazy(() => import('./views/SettingsView').then(m => ({ default: m.SettingsView })));
import { LeftSidebar } from './components/LeftSidebar';
import { NotificationPanel } from './components/NotificationPanel';
import {
  LayoutGrid, Server, AlertTriangle, Settings, Power,
  Box, Bot, Layers, GitBranch, Hexagon, Command,
  Bell, Search, Menu, X, ChevronRight, User,
  PanelRight, Wifi, Cpu, Activity, Lock, LockOpen
} from 'lucide-react';
import { NotificationProvider, useNotifications } from './components/NotificationSystem';
import { RightSidebar } from './components/RightSidebar';
import { CommandPalette } from './components/CommandPalette';
import { SplashScreen } from './components/SplashScreen';
import { getToken } from './api';


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
  const { addNotification } = useNotifications();
  const [hasToken, setHasToken] = useState(() => !!getToken());

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
    '/traces': 'CICD',
    '/errors': 'INCIDENTS',
    '/settings': 'SETTINGS',
  };

  const currentView = pathToView[location.pathname] || 'DASHBOARD';

  const handleViewChange = (view: ViewState) => {
    const viewToPath: Record<ViewState, string> = {
      'DASHBOARD': '/',
      'DYNAMIC_ROUTES': '/routes',
      'FUNCTIONS': '/functions',
      'CICD': '/traces',
      'INCIDENTS': '/errors',
      'SETTINGS': '/settings',
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

  // Global auth events: 401 -> notify + jump to Settings; token edits -> refresh badge
  useEffect(() => {
    const onUnauthorized = () => {
      addNotification('ERROR', 'UNAUTHORIZED', 'Server rejected the request (401). Set your API token in Settings.');
      navigate('/settings');
    };
    const onAuthChanged = () => setHasToken(!!getToken());
    window.addEventListener('worpen:unauthorized', onUnauthorized);
    window.addEventListener('worpen:auth-changed', onAuthChanged);
    return () => {
      window.removeEventListener('worpen:unauthorized', onUnauthorized);
      window.removeEventListener('worpen:auth-changed', onAuthChanged);
    };
  }, [addNotification, navigate]);

  const toggleSidebar = () => setSidebarOpen(!sidebarOpen);
  const toggleRightSidebar = () => setRightSidebarOpen(!rightSidebarOpen);

  return (
    <div className="flex h-screen w-full bg-[var(--bg-deep)] text-[var(--text-main)] selection:bg-[#e85d2630] overflow-hidden">

      <SplashScreen />
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
              <span className="text-[#e85d26] font-semibold">{currentView}</span>
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
                onClick={() => handleViewChange('SETTINGS')}
                className={`transition-colors p-1.5 rounded-[var(--radius)] ${hasToken ? 'text-[#e85d26] bg-[#e85d2620]' : 'text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40]'}`}
                title={hasToken ? 'API token configured' : 'No API token — click to configure in Settings'}
              >
                {hasToken ? <Lock size={16} /> : <LockOpen size={16} />}
              </button>

              <button
                onClick={toggleRightSidebar}
                className={`transition-colors p-1.5 rounded-[var(--radius)] ${rightSidebarOpen ? 'text-[#e85d26] bg-[#e85d2620]' : 'text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40]'}`}
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
        <main className={`flex-1 ${['/', '/traces'].includes(location.pathname) ? 'overflow-y-auto' : 'overflow-hidden'} custom-scrollbar relative bg-[#1e1f22] m-1.5 rounded-[var(--radius)] border border-[#43454a] shadow-inner overflow-x-hidden`}>
          <AnimatePresence mode="wait">
            <PageTransition key={location.pathname} className="h-full">
              <Suspense fallback={
                <div className="h-full flex items-center justify-center text-[#6e7073] text-[13px] font-mono">
                  <span className="animate-pulse">loading view…</span>
                </div>
              }>
              <Routes location={location}>
                <Route path="/" element={<Dashboard />} />
                <Route path="/routes" element={<DynamicRoutesView />} />
                <Route path="/functions" element={<FunctionsView />} />
                <Route path="/traces" element={<CicdView />} />
                <Route path="/errors" element={<IncidentsView />} />
                <Route path="/settings" element={<SettingsView />} />
              </Routes>
              </Suspense>
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
