import React, { useState, useEffect } from 'react';
import { ViewState, AgentStatus } from './types';
import { Dashboard } from './views/Dashboard';
import { Fleet } from './views/Fleet';
import { DockerView } from './views/DockerView';
import { AutomationView } from './views/AutomationView';
import { SettingsView } from './views/SettingsView';
import { DependencyView } from './views/DependencyView';
import { LayoutGrid, Server, AlertTriangle, Settings, Power, Box, Bot, Shield, User, Globe, Activity, Layers } from 'lucide-react';
import { MOCK_INCIDENTS, MOCK_AGENTS } from './constants';
import { TerminalFrame } from './components/TerminalFrame';

const IncidentsView: React.FC = () => (
  <div className="h-full p-2">
    <TerminalFrame title="INCIDENT_LOGS [SELF_HEALING_EVENTS]">
      <table className="w-full text-left text-xs md:text-sm border-collapse">
        <thead>
          <tr className="border-b-2 border-green-700 text-green-300 uppercase">
            <th className="p-2">Time</th>
            <th className="p-2">Service</th>
            <th className="p-2">Node</th>
            <th className="p-2">Issue</th>
            <th className="p-2">Auto-Action</th>
            <th className="p-2">Status</th>
          </tr>
        </thead>
        <tbody>
          {MOCK_INCIDENTS.map((inc) => (
            <tr key={inc.id} className="border-b border-green-900/30 hover:bg-green-900/20 font-mono">
              <td className="p-2">{inc.time}</td>
              <td className="p-2">{inc.service}</td>
              <td className="p-2 text-gray-400">{inc.node}</td>
              <td className="p-2 text-yellow-400">{inc.issue}</td>
              <td className="p-2 text-blue-400">[{inc.action}]</td>
              <td className="p-2">
                 <span className={`px-1 ${inc.status === 'RESOLVED' ? 'bg-green-900 text-green-300' : 'bg-red-900 text-red-300'}`}>
                   {inc.status}
                 </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </TerminalFrame>
  </div>
);

const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewState>('DASHBOARD');
  const [time, setTime] = useState(new Date().toLocaleTimeString());

  useEffect(() => {
    const timer = setInterval(() => {
      setTime(new Date().toLocaleTimeString());
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const renderView = () => {
    switch(currentView) {
      case 'DASHBOARD': return <Dashboard />;
      case 'FLEET': return <Fleet />;
      case 'DOCKER': return <DockerView />;
      case 'AUTOMATION': return <AutomationView />;
      case 'DEPENDENCY': return <DependencyView />;
      case 'INCIDENTS': return <IncidentsView />;
      case 'SETTINGS': return <SettingsView />;
      default: return <div className="p-10 text-center animate-pulse">MODULE UNDER CONSTRUCTION...</div>;
    }
  };

  const NavGroup = ({ label, children }: { label: string, children: React.ReactNode }) => (
    <div className="mb-6">
      <h3 className="text-[10px] text-gray-600 font-bold mb-2 px-4 uppercase tracking-widest border-b border-gray-900 pb-1">
        // {label}
      </h3>
      <div className="flex flex-col gap-1">
        {children}
      </div>
    </div>
  );

  const NavItem = ({ view, icon: Icon, label, alert }: { view: ViewState, icon: any, label: string, alert?: boolean }) => (
    <button 
      onClick={() => setCurrentView(view)}
      className={`group flex items-center gap-3 w-full px-4 py-2 transition-all relative
        ${currentView === view 
          ? 'bg-green-900/30 text-green-100' 
          : 'text-gray-400 hover:text-green-300 hover:bg-green-900/10'
        }`}
    >
      {currentView === view && (
        <div className="absolute left-0 top-0 bottom-0 w-1 bg-green-500 shadow-[0_0_10px_#00ff00]"></div>
      )}
      <Icon size={16} className={`group-hover:text-green-400 ${currentView === view ? 'text-green-400' : ''}`} />
      <span className="text-xs font-medium tracking-wider">{label}</span>
      {alert && <span className="ml-auto w-2 h-2 rounded-full bg-red-500 animate-pulse"></span>}
    </button>
  );

  // Health Calculation Logic
  const criticalCount = MOCK_AGENTS.filter(a => a.status === AgentStatus.OFFLINE || a.status === AgentStatus.CRITICAL).length;
  const healingCount = MOCK_AGENTS.filter(a => a.status === AgentStatus.HEALING).length;
  
  let healthIndicatorClass = "bg-green-500 shadow-[0_0_5px_#22c55e]";
  let healthTextClass = "text-green-400";
  let healthValue = "98.4%";

  if (criticalCount > 0) {
    healthIndicatorClass = "bg-red-500 shadow-[0_0_5px_#ef4444]";
    healthTextClass = "text-red-500";
    healthValue = "WARNING";
  } else if (healingCount > 0) {
    healthIndicatorClass = "bg-yellow-500 shadow-[0_0_5px_#eab308]";
    healthTextClass = "text-yellow-400";
    healthValue = "HEALING";
  }

  return (
    <div className="flex flex-col h-screen w-full bg-black text-green-500 overflow-hidden">
      
      {/* Top Bar */}
      <header className="h-10 border-b border-green-900 bg-[#050505] flex items-center justify-between px-4 z-10 relative">
        <div className="flex items-center gap-3">
          <Shield className="w-5 h-5 text-green-500" />
          <span className="font-bold text-lg tracking-[0.2em] text-white glow-text">WORPEN</span>
          <span className="px-1 bg-green-900 text-[9px] text-green-200 border border-green-700">BETA</span>
        </div>
        
        {/* Top Stats */}
        <div className="hidden lg:flex items-center gap-8 text-[10px] font-mono text-gray-500">
           <div className="flex items-center gap-2">
             <div className={`w-2 h-2 rounded-full animate-pulse ${healthIndicatorClass}`}></div>
             <Activity size={12} className={healthTextClass}/>
             <span>CLUSTER_HEALTH: <span className={healthTextClass}>{healthValue}</span></span>
           </div>
           <div className="flex items-center gap-2">
             <Globe size={12} className="text-blue-600"/>
             <span>REGION: <span className="text-blue-400">EU-WEST-1</span></span>
           </div>
           <div className="text-green-300 border-l border-green-900 pl-4">{time}</div>
        </div>
      </header>

      <div className="flex flex-1 overflow-hidden relative z-0">
        
        {/* ENHANCED SIDEBAR */}
        <aside className="w-16 md:w-64 border-r border-green-900 flex flex-col bg-[#020202] relative">
           {/* Scanline overlay for sidebar only */}
           <div className="absolute inset-0 pointer-events-none bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] z-0 opacity-20 bg-[length:100%_2px,3px_100%]"></div>

           {/* User Profile Block */}
           <div className="p-4 border-b border-green-900/50 mb-2 relative z-10">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-green-900/20 border border-green-700 flex items-center justify-center">
                   <User className="text-green-500" size={20} />
                </div>
                <div className="hidden md:block">
                  <div className="text-xs font-bold text-white">OPERATOR_ROOT</div>
                  <div className="text-[9px] text-gray-500">SEC_LEVEL: <span className="text-red-500">5 (GOD)</span></div>
                </div>
              </div>
           </div>

           {/* Navigation Links */}
           <div className="flex-1 py-4 overflow-y-auto relative z-10 scrollbar-hide">
              <NavGroup label="MONITORING">
                <NavItem view="DASHBOARD" icon={LayoutGrid} label="COCKPIT" />
                <NavItem view="FLEET" icon={Server} label="FLEET NODES" />
              </NavGroup>

              <NavGroup label="ORCHESTRATION">
                <NavItem view="DOCKER" icon={Box} label="DOCKER CONTAINERS" />
                <NavItem view="AUTOMATION" icon={Bot} label="AUTO-HEALING" />
                <NavItem view="DEPENDENCY" icon={Layers} label="ARTIFACT NEXUS" />
              </NavGroup>

              <NavGroup label="SYSTEM">
                <NavItem view="INCIDENTS" icon={AlertTriangle} label="INCIDENT LOGS" alert={true} />
                <NavItem view="SETTINGS" icon={Settings} label="CONFIG" />
              </NavGroup>
           </div>
           
           {/* Bottom Actions */}
           <div className="p-4 border-t border-green-900 relative z-10">
              <div className="mb-4 text-[9px] font-mono text-gray-600 space-y-1 hidden md:block">
                <div className="flex justify-between"><span>PING:</span> <span className="text-green-600">4ms</span></div>
                <div className="flex justify-between"><span>PKT_LOSS:</span> <span className="text-green-600">0%</span></div>
              </div>
              <button className="flex items-center justify-center gap-3 text-red-500 hover:text-white hover:bg-red-600 w-full p-2 border border-red-900/30 transition-colors">
                 <Power size={14} />
                 <span className="hidden md:inline font-bold text-xs">TERMINATE SESSION</span>
              </button>
           </div>
        </aside>

        {/* Main Content */}
        <main className="flex-1 overflow-hidden bg-black relative">
          {/* Background Grid */}
          <div className="absolute inset-0 bg-[linear-gradient(to_right,#0f1510_1px,transparent_1px),linear-gradient(to_bottom,#0f1510_1px,transparent_1px)] bg-[size:40px_40px] opacity-20 pointer-events-none"></div>
          
          <div className="h-full overflow-y-auto p-4 md:p-6 custom-scrollbar relative z-10">
            {renderView()}
          </div>
        </main>
      </div>
      
      {/* Footer / Status Line */}
      <div className="h-6 bg-green-900/20 border-t border-green-900 flex items-center justify-between px-4 text-[9px] font-mono text-gray-500">
        <div>SYSTEM STATUS: <span className="text-green-500">OPTIMAL</span></div>
        <div>WORPEN_OS v2.4.0-rc1</div>
      </div>
    </div>
  );
};

export default App;