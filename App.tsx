
import React, { useState, useEffect } from 'react';
import { ViewState } from './types';
import { Dashboard } from './views/Dashboard';
import { Fleet } from './views/Fleet';
import { DockerView } from './views/DockerView';
import { AutomationView } from './views/AutomationView';
import { SettingsView } from './views/SettingsView';
import { DependencyView } from './views/DependencyView';
import { CicdView } from './views/CicdView';
import { 
  LayoutGrid, Server, AlertTriangle, Settings, Power, 
  Box, Bot, Layers, GitBranch, Hexagon, Command, 
  Bell, Search, Menu, X, ChevronRight, User
} from 'lucide-react';
import { TerminalFrame } from './components/TerminalFrame';
import { MOCK_INCIDENTS } from './constants';

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

const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewState>('DASHBOARD');
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [scrolled, setScrolled] = useState(false);

  const toggleSidebar = () => setSidebarOpen(!sidebarOpen);

  const NavItem = ({ view, icon: Icon, label, alert }: { view: ViewState, icon: any, label: string, alert?: boolean }) => {
    const active = currentView === view;
    return (
      <button 
        onClick={() => setCurrentView(view)}
        className={`
          w-full flex items-center gap-3 px-3 py-2 rounded-md transition-all duration-200 group
          ${active ? 'bg-green-500/10 text-green-400' : 'text-gray-400 hover:text-white hover:bg-white/5'}
        `}
      >
        <Icon size={18} className={`transition-colors ${active ? 'text-green-400' : 'text-gray-500 group-hover:text-white'}`} />
        {sidebarOpen && (
          <span className="text-sm font-medium tracking-wide flex-1 text-left">{label}</span>
        )}
        {sidebarOpen && alert && (
          <span className="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse shadow-[0_0_5px_rgba(239,68,68,0.5)]"></span>
        )}
        {!sidebarOpen && active && (
           <div className="absolute left-0 top-0 bottom-0 w-[2px] bg-green-500"></div>
        )}
      </button>
    );
  };

  const NavGroup = ({ label, children }: { label: string, children?: React.ReactNode }) => (
    <div className="mb-6">
      {sidebarOpen && (
        <h3 className="text-[10px] font-mono text-gray-600 uppercase tracking-widest px-3 mb-2">{label}</h3>
      )}
      <div className="space-y-0.5">
        {children}
      </div>
    </div>
  );

  return (
    <div className="flex h-screen w-full bg-[#030303] text-gray-200 selection:bg-green-500/30">
      
      {/* SIDEBAR */}
      <aside 
        className={`
          flex flex-col border-r border-white/5 bg-[#050505] transition-all duration-300 relative z-20
          ${sidebarOpen ? 'w-64' : 'w-16'}
        `}
      >
        {/* Brand */}
        <div className="h-16 flex items-center px-4 border-b border-white/5">
          <div className="flex items-center gap-3 text-green-500">
            <Hexagon size={28} strokeWidth={1.5} className="fill-green-500/10" />
            {sidebarOpen && (
              <div className="flex flex-col">
                <span className="font-bold text-lg tracking-tight text-white leading-none">WORPEN</span>
                <span className="text-[9px] text-gray-500 font-mono tracking-widest">OS v2.4</span>
              </div>
            )}
          </div>
        </div>

        {/* Navigation */}
        <div className="flex-1 overflow-y-auto py-6 px-3 custom-scrollbar">
          <NavGroup label="Overview">
            <NavItem view="DASHBOARD" icon={LayoutGrid} label="Dashboard" />
            <NavItem view="FLEET" icon={Server} label="Fleet Nodes" />
          </NavGroup>

          <NavGroup label="Orchestration">
            <NavItem view="DOCKER" icon={Box} label="Containers" />
            <NavItem view="AUTOMATION" icon={Bot} label="Auto-Healing" />
            <NavItem view="DEPENDENCY" icon={Layers} label="Artifacts" />
            <NavItem view="CICD" icon={GitBranch} label="Pipeline" />
          </NavGroup>

          <NavGroup label="System">
            <NavItem view="INCIDENTS" icon={AlertTriangle} label="Incidents" alert={true} />
            <NavItem view="SETTINGS" icon={Settings} label="Settings" />
          </NavGroup>
        </div>

        {/* User Footer */}
        <div className="p-3 border-t border-white/5">
          <button className={`w-full flex items-center gap-3 p-2 rounded-md hover:bg-white/5 transition-colors ${!sidebarOpen ? 'justify-center' : ''}`}>
             <div className="w-8 h-8 rounded bg-gradient-to-tr from-green-900 to-black border border-green-800 flex items-center justify-center text-green-500 font-bold text-xs">
                OP
             </div>
             {sidebarOpen && (
               <div className="text-left overflow-hidden">
                 <div className="text-xs font-medium text-white truncate">Operator_Root</div>
                 <div className="text-[10px] text-gray-500 truncate">Sec_Level: 5</div>
               </div>
             )}
          </button>
        </div>
      </aside>

      {/* MAIN CONTENT */}
      <div className="flex-1 flex flex-col min-w-0 bg-black relative">
        {/* Background Grid */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#111_1px,transparent_1px),linear-gradient(to_bottom,#111_1px,transparent_1px)] bg-[size:4rem_4rem] opacity-20 pointer-events-none"></div>

        {/* HEADER */}
        <header className="h-16 flex items-center justify-between px-6 border-b border-white/5 bg-[#030303]/80 backdrop-blur-sm sticky top-0 z-10">
           <div className="flex items-center gap-4">
              <button onClick={toggleSidebar} className="text-gray-500 hover:text-white transition-colors">
                 {sidebarOpen ? <Menu size={20} /> : <ChevronRight size={20} />}
              </button>
              
              {/* Breadcrumbs */}
              <div className="hidden md:flex items-center gap-2 text-sm text-gray-500 font-mono">
                 <span>worpen</span>
                 <span className="text-gray-700">/</span>
                 <span className="text-green-500">{currentView.toLowerCase()}</span>
              </div>
           </div>

           <div className="flex items-center gap-4">
              {/* Global Search Mockup */}
              <div className="hidden md:flex items-center gap-2 px-3 py-1.5 bg-white/5 rounded border border-white/10 text-gray-500 w-64">
                 <Search size={14} />
                 <span className="text-xs">Search resources...</span>
                 <div className="ml-auto flex gap-1">
                    <span className="text-[10px] border border-white/10 px-1 rounded bg-black">⌘</span>
                    <span className="text-[10px] border border-white/10 px-1 rounded bg-black">K</span>
                 </div>
              </div>

              {/* Actions */}
              <button className="text-gray-500 hover:text-white transition-colors relative">
                 <Bell size={20} />
                 <span className="absolute top-0 right-0 w-2 h-2 bg-red-500 rounded-full"></span>
              </button>
              
              <div className="h-6 w-[1px] bg-white/10 mx-2"></div>
              
              <button className="text-red-500 hover:bg-red-500/10 p-2 rounded-md transition-colors">
                 <Power size={20} />
              </button>
           </div>
        </header>

        {/* VIEWPORT */}
        <main className="flex-1 overflow-y-auto custom-scrollbar relative">
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
      </div>
    </div>
  );
};

export default App;
