
export enum AgentStatus {
  ONLINE = 'ONLINE',
  OFFLINE = 'OFFLINE',
  HEALING = 'HEALING',
  CRITICAL = 'CRITICAL'
}

export interface Agent {
  id: string;
  name: string;
  ip: string;
  os: string;
  version: string;
  status: AgentStatus;
  cpuLoad: number;
  memoryLoad: number;
  lastSeen: string;
  uptime: string;
  peers: number; // Connected nodes in the mesh/side-net
}

export interface LogEntry {
  id: string;
  timestamp: string;
  level: 'INFO' | 'WARN' | 'ERROR' | 'SUCCESS';
  source: string;
  message: string;
}

export interface Incident {
  id: string;
  service: string;
  node: string;
  issue: string;
  action: string;
  status: 'RESOLVED' | 'PENDING' | 'FAILED';
  time: string;
}

export interface Container {
  id: string;
  name: string;
  image: string;
  state: 'RUNNING' | 'EXITED' | 'RESTARTING';
  status: string;
  node: string;
  ports: string;
}

export interface AutomationRule {
  id: string;
  name: string;
  triggerEvent: string;
  targetService: string;
  script: string;
  lastRun: string;
  active: boolean;
}

export interface DockerLayer {
  hash: string;
  instruction: string;
  size: string;
  isShared: boolean;
  status: 'CACHED' | 'DOWNLOADING' | 'REDUNDANT';
}

export interface ImageManifest {
  id: string;
  name: string;
  tag: string;
  totalSize: string;
  layers: DockerLayer[];
}

export interface PipelineStage {
  id: string;
  name: string;
  status: 'SUCCESS' | 'FAILED' | 'RUNNING' | 'PENDING';
  duration: string;
}

export interface Pipeline {
  id: string;
  name: string;
  branch: string;
  commit: string;
  author: string;
  status: 'SUCCESS' | 'FAILED' | 'RUNNING' | 'PENDING';
  lastRun: string;
  stages: PipelineStage[];
}

export type ViewState = 'DASHBOARD' | 'DYNAMIC_ROUTES' | 'FUNCTIONS' | 'STORAGE' | 'CICD' | 'INCIDENTS' | 'SETTINGS' | 'FLEET' | 'DOCKER' | 'AUTOMATION' | 'DEPENDENCY';