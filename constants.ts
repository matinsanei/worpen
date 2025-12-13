
import { Agent, AgentStatus, Incident, LogEntry, Container, AutomationRule, ImageManifest, Pipeline } from "./types";

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
    uptime: "45d 12h",
    peers: 4
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
    uptime: "12d 4h",
    peers: 3
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
    uptime: "201d 1h",
    peers: 2
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
    uptime: "0",
    peers: 0
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
    uptime: "5d 22h",
    peers: 4
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

export interface GossipLog {
  id: string;
  from: string;
  to: string;
  type: 'SYNC_RULE' | 'HEARTBEAT' | 'SHARED_LAYER' | 'ALERT';
  payload: string;
  latency: string;
}

export const MOCK_GOSSIP_LOGS: GossipLog[] = [
  { id: 'g1', from: 'bee-001', to: 'bee-002', type: 'SYNC_RULE', payload: 'rule-01 (OOM_KILL)', latency: '2ms' },
  { id: 'g2', from: 'bee-003', to: 'bee-005', type: 'HEARTBEAT', payload: 'status: OK', latency: '5ms' },
  { id: 'g3', from: 'bee-002', to: 'bee-001', type: 'SHARED_LAYER', payload: 'sha256:7a8b...1', latency: '12ms' },
  { id: 'g4', from: 'bee-005', to: 'bee-002', type: 'SYNC_RULE', payload: 'rule-03 (SCALE)', latency: '3ms' },
  { id: 'g5', from: 'bee-001', to: 'bee-003', type: 'ALERT', payload: 'WARN: High Load', latency: '4ms' },
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

export const MOCK_IMAGES: ImageManifest[] = [
  {
    id: "img-001",
    name: "frontend-dashboard",
    tag: "v3.2.1",
    totalSize: "145MB",
    layers: [
      { hash: "sha256:7a8b...1", instruction: "FROM node:18-alpine", size: "40MB", isShared: true, status: 'CACHED' },
      { hash: "sha256:8c9d...2", instruction: "WORKDIR /app", size: "0B", isShared: true, status: 'CACHED' },
      { hash: "sha256:9d0e...3", instruction: "COPY package.json .", size: "2KB", isShared: true, status: 'CACHED' },
      { hash: "sha256:ae1f...4", instruction: "RUN npm install", size: "85MB", isShared: false, status: 'REDUNDANT' },
      { hash: "sha256:bf2a...5", instruction: "COPY . .", size: "15MB", isShared: false, status: 'DOWNLOADING' },
      { hash: "sha256:c34b...6", instruction: "RUN npm run build", size: "5MB", isShared: false, status: 'DOWNLOADING' }
    ]
  },
  {
    id: "img-002",
    name: "backend-api",
    tag: "v3.2.1",
    totalSize: "180MB",
    layers: [
      { hash: "sha256:7a8b...1", instruction: "FROM node:18-alpine", size: "40MB", isShared: true, status: 'CACHED' },
      { hash: "sha256:8c9d...2", instruction: "WORKDIR /app", size: "0B", isShared: true, status: 'CACHED' },
      { hash: "sha256:1a2b...9", instruction: "COPY package.json .", size: "2KB", isShared: true, status: 'CACHED' },
      { hash: "sha256:ae1f...4", instruction: "RUN npm install", size: "85MB", isShared: false, status: 'REDUNDANT' },
      { hash: "sha256:ef56...8", instruction: "COPY src/ .", size: "55MB", isShared: false, status: 'DOWNLOADING' }
    ]
  }
];

export const MOCK_PIPELINES: Pipeline[] = [
  {
    id: "pipe-001",
    name: "production-deploy",
    branch: "main",
    commit: "7f8a91b (fix: hotfix payment gateway)",
    author: "dev_ops_Lead",
    status: "SUCCESS",
    lastRun: "2 hours ago",
    stages: [
      { id: "s1", name: "BUILD", status: "SUCCESS", duration: "45s" },
      { id: "s2", name: "TEST", status: "SUCCESS", duration: "1m 20s" },
      { id: "s3", name: "SCAN", status: "SUCCESS", duration: "30s" },
      { id: "s4", name: "DEPLOY", status: "SUCCESS", duration: "15s" }
    ]
  },
  {
    id: "pipe-002",
    name: "staging-integration",
    branch: "develop",
    commit: "3c4d5e6 (feat: new dashboard widgets)",
    author: "frontend_ninja",
    status: "FAILED",
    lastRun: "5 mins ago",
    stages: [
      { id: "s1", name: "BUILD", status: "SUCCESS", duration: "42s" },
      { id: "s2", name: "TEST", status: "FAILED", duration: "3s" },
      { id: "s3", name: "SCAN", status: "PENDING", duration: "-" },
      { id: "s4", name: "DEPLOY", status: "PENDING", duration: "-" }
    ]
  },
  {
    id: "pipe-003",
    name: "nightly-cleanup",
    branch: "cron/nightly",
    commit: "system_trigger",
    author: "SYSTEM",
    status: "RUNNING",
    lastRun: "Running...",
    stages: [
      { id: "s1", name: "PRUNE", status: "SUCCESS", duration: "10s" },
      { id: "s2", name: "BACKUP", status: "RUNNING", duration: "12s..." },
      { id: "s3", name: "NOTIFY", status: "PENDING", duration: "-" }
    ]
  }
];