import { Agent, AgentStatus, Incident, LogEntry, Container, AutomationRule } from "./types";

export const MOCK_AGENTS: Agent[] = [
  {
    id: "bee-001",
    name: "hive-worker-alpha",
    ip: "192.168.1.101",
    os: "Alpine Linux",
    version: "v2.0.4",
    status: AgentStatus.ONLINE,
    cpuLoad: 12,
    memoryLoad: 34,
    lastSeen: "0ms ago",
    uptime: "45d 12h"
  },
  {
    id: "bee-002",
    name: "hive-worker-beta",
    ip: "192.168.1.102",
    os: "Ubuntu Server",
    version: "v2.0.4",
    status: AgentStatus.HEALING,
    cpuLoad: 89,
    memoryLoad: 92,
    lastSeen: "12ms ago",
    uptime: "12d 4h"
  },
  {
    id: "bee-003",
    name: "edge-robot-x1",
    ip: "10.0.0.55",
    os: "Yocto",
    version: "v1.9.9",
    status: AgentStatus.ONLINE,
    cpuLoad: 4,
    memoryLoad: 15,
    lastSeen: "5ms ago",
    uptime: "201d 1h"
  },
  {
    id: "bee-004",
    name: "vps-fra-09",
    ip: "172.16.0.4",
    os: "Debian 12",
    version: "v2.0.4",
    status: AgentStatus.OFFLINE,
    cpuLoad: 0,
    memoryLoad: 0,
    lastSeen: "4h ago",
    uptime: "0"
  },
  {
    id: "bee-005",
    name: "db-cluster-node-1",
    ip: "192.168.20.5",
    os: "Alpine Linux",
    version: "v2.0.4",
    status: AgentStatus.ONLINE,
    cpuLoad: 45,
    memoryLoad: 78,
    lastSeen: "1ms ago",
    uptime: "5d 22h"
  }
];

export const MOCK_LOGS: LogEntry[] = [
  { id: '1', timestamp: '10:42:01', level: 'INFO', source: 'CORE', message: 'Heartbeat received from 1,204 agents.' },
  { id: '2', timestamp: '10:42:05', level: 'WARN', source: 'bee-002', message: 'Memory usage spike detected (92%).' },
  { id: '3', timestamp: '10:42:06', level: 'ERROR', source: 'bee-002', message: 'Container [saleor-api] SIGKILL received (OOM).' },
  { id: '4', timestamp: '10:42:06', level: 'INFO', source: 'bee-002', message: 'Strategy matched: [Restart_On_Failure].' },
  { id: '5', timestamp: '10:42:07', level: 'SUCCESS', source: 'bee-002', message: 'Container [saleor-api] successfully restarted. PID: 4521.' },
  { id: '6', timestamp: '10:42:15', level: 'INFO', source: 'CORE', message: 'Fleet synchronization complete.' },
];

export const MOCK_INCIDENTS: Incident[] = [
  { id: 'inc-992', service: 'nginx-proxy', node: 'hive-worker-alpha', issue: 'Connection Timeout', action: 'Config Reload', status: 'RESOLVED', time: '10:30 AM' },
  { id: 'inc-993', service: 'saleor-api', node: 'hive-worker-beta', issue: 'Out of Memory', action: 'Container Restart', status: 'RESOLVED', time: '10:42 AM' },
  { id: 'inc-994', service: 'redis-cache', node: 'edge-robot-x1', issue: 'Disk Full', action: 'Log Rotation', status: 'PENDING', time: '10:45 AM' },
];

export const MOCK_CONTAINERS: Container[] = [
  { id: 'a1b2c3d4', name: 'saleor-api', image: 'saleor/saleor:3.1', state: 'RUNNING', status: 'Up 2 hours', node: 'hive-worker-beta', ports: '8000:8000' },
  { id: 'e5f6g7h8', name: 'saleor-dashboard', image: 'saleor/dashboard:3.1', state: 'RUNNING', status: 'Up 2 hours', node: 'hive-worker-beta', ports: '9000:80' },
  { id: 'i9j0k1l2', name: 'postgres-db', image: 'postgres:13-alpine', state: 'RUNNING', status: 'Up 5 days', node: 'db-cluster-node-1', ports: '5432:5432' },
  { id: 'm3n4o5p6', name: 'redis-cache', image: 'redis:6-alpine', state: 'EXITED', status: 'Exited (137) 10 min ago', node: 'edge-robot-x1', ports: '6379:6379' },
  { id: 'q7r8s9t0', name: 'nginx-proxy', image: 'nginx:latest', state: 'RUNNING', status: 'Up 45 days', node: 'hive-worker-alpha', ports: '80:80, 443:443' },
];

export const MOCK_AUTOMATION_RULES: AutomationRule[] = [
  { 
    id: 'rule-01', 
    name: 'OOM_KILL_RESTART', 
    triggerEvent: 'CONTAINER_OOM_KILLED', 
    targetService: '*', 
    lastRun: '12 min ago',
    active: true,
    script: `// Strategy: Immediate Restart
if (event.exit_code === 137) {
  log.warn("OOM Detected on " + service);
  docker.restart(container_id);
  notify.slack("#ops-alerts", "Restarted " + service);
}`
  },
  { 
    id: 'rule-02', 
    name: 'DISK_CLEANUP_90', 
    triggerEvent: 'DISK_USAGE_HIGH', 
    targetService: 'system', 
    lastRun: '2 days ago',
    active: true,
    script: `// Strategy: Prune Docker System
if (metrics.disk_usage > 90) {
  log.info("Pruning unused images...");
  await exec("docker system prune -f");
}`
  },
  { 
    id: 'rule-03', 
    name: 'HIGH_LATENCY_SCALE', 
    triggerEvent: 'RESPONSE_TIME_GT_500MS', 
    targetService: 'saleor-api', 
    lastRun: 'Never',
    active: false,
    script: `// Strategy: Horizontal Scale
if (metrics.avg_response_time > 500) {
  let current = await docker.replicas(service);
  if (current < 5) {
     docker.scale(service, current + 1);
  }
}`
  }
];