/// <reference types="vite/client" />

// API Configuration
export const getApiBaseUrl = () => {
  let url = 'http://127.0.0.1:3000';
  if (typeof window !== 'undefined') {
    url = localStorage.getItem('WORPEN_API_URL') || import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';
  } else {
    url = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';
  }
  const finalized = url.replace(/\/$/, "");
  // console.log("WORPEN API URL:", finalized); // Debugging
  return finalized;
};

// API Helper
async function apiRequest<T>(
  endpoint: string,
  options?: RequestInit
): Promise<T> {
  const url = `${getApiBaseUrl()}${endpoint}`;

  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    throw new Error(`API Error: ${response.statusText}`);
  }

  return response.json();
}

// Dashboard API
export const dashboardApi = {
  getStats: () => apiRequest('/api/v1/dashboard/stats'),
  getNetwork: () => apiRequest('/api/v1/dashboard/network'),
  getServices: () => apiRequest('/api/v1/dashboard/services'),
  getLoad: () => apiRequest('/api/v1/dashboard/load'),
};

// Fleet/Agents API
export const agentsApi = {
  list: () => apiRequest('/api/v1/agents'),
  get: (id: string) => apiRequest(`/api/v1/agents/${id}`),
  sync: () => apiRequest('/api/v1/agents/sync', { method: 'POST' }),
  exec: (id: string, command: string, args: string[] = []) =>
    apiRequest(`/api/v1/agents/${id}/exec`, {
      method: 'POST',
      body: JSON.stringify({ command, args, tty: false }),
    }),
  startShell: (id: string) =>
    apiRequest(`/api/v1/agents/${id}/shell`, { method: 'POST' }),
  getProcesses: (id: string) =>
    apiRequest(`/api/v1/agents/${id}/processes`),
  getSystemInfo: (id: string) =>
    apiRequest(`/api/v1/agents/${id}/system`),
};

// Docker/Containers API
export const containersApi = {
  list: () => apiRequest('/api/v1/containers'),
  logs: (id: string) => apiRequest(`/api/v1/containers/${id}/logs`),
  start: (id: string) =>
    apiRequest(`/api/v1/containers/${id}/start`, { method: 'POST' }),
  stop: (id: string) =>
    apiRequest(`/api/v1/containers/${id}/stop`, { method: 'POST' }),
  restart: (id: string) =>
    apiRequest(`/api/v1/containers/${id}/restart`, { method: 'POST' }),
  delete: (id: string) =>
    apiRequest(`/api/v1/containers/${id}`, { method: 'DELETE' }),
  exec: (id: string, command: string, args: string[] = []) =>
    apiRequest(`/api/v1/containers/${id}/exec`, {
      method: 'POST',
      body: JSON.stringify({ command, args, tty: false }),
    }),
  startShell: (id: string) =>
    apiRequest(`/api/v1/containers/${id}/shell`, { method: 'POST' }),
  attach: (id: string) => apiRequest(`/api/v1/containers/${id}/attach`),
  inspect: (id: string) => apiRequest(`/api/v1/containers/${id}/inspect`),
};

// Docker Images API
export const imagesApi = {
  list: () => apiRequest('/api/v1/images'),
  pull: (image: string) =>
    apiRequest('/api/v1/images/pull', {
      method: 'POST',
      body: JSON.stringify({ image }),
    }),
  prune: (all: boolean = false) =>
    apiRequest('/api/v1/docker/prune', {
      method: 'POST',
      body: JSON.stringify({ all }),
    }),
};

// Incidents API
export const incidentsApi = {
  list: () => apiRequest('/api/v1/incidents'),
  resolve: (id: string, method: string = 'MANUAL', notes: string = '') =>
    apiRequest(`/api/v1/incidents/${id}/resolve`, {
      method: 'POST',
      body: JSON.stringify({ method, notes }),
    }),
};

// Automation API
export const automationApi = {
  listRules: () => apiRequest('/api/v1/automation/rules'),
  getRule: (id: string) => apiRequest(`/api/v1/automation/rules/${id}`),
  createRule: (rule: any) =>
    apiRequest('/api/v1/automation/rules', {
      method: 'POST',
      body: JSON.stringify(rule),
    }),
  updateRule: (id: string, updates: any) =>
    apiRequest(`/api/v1/automation/rules/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    }),
  dryRun: (script: string, mockContext: any = {}) =>
    apiRequest('/api/v1/automation/dry-run', {
      method: 'POST',
      body: JSON.stringify({ script, mockContext }),
    }),
};

// Pipelines API
export const pipelinesApi = {
  list: () => apiRequest('/api/v1/pipelines'),
  trigger: (pipelineId: string) =>
    apiRequest('/api/v1/pipelines/trigger', {
      method: 'POST',
      body: JSON.stringify({ pipelineId }),
    }),
};

// Logs API
export const logsApi = {
  list: (level?: string, source?: string) => {
    const params = new URLSearchParams();
    if (level) params.append('level', level);
    if (source) params.append('source', source);
    const query = params.toString();
    return apiRequest(`/api/v1/logs${query ? '?' + query : ''}`);
  },
};

// Terminal API
export const terminalApi = {
  listSessions: () => apiRequest('/api/v1/terminal/sessions'),
  connectContainer: (containerId: string) => {
    const wsUrl = getApiBaseUrl().replace('http', 'ws');
    return new WebSocket(`${wsUrl}/api/v1/terminal/container/${containerId}`);
  },
  connectAgent: (agentId: string) => {
    const wsUrl = getApiBaseUrl().replace('http', 'ws');
    return new WebSocket(`${wsUrl}/api/v1/terminal/agent/${agentId}`);
  },
};

// Health Check
export const healthApi = {
  check: () => fetch(`${getApiBaseUrl()}/health`).then(r => r.text()),
};

// Global Functions API
export interface FunctionDef {
  name: string;
  params: string[];
  logic: any[]; // Array of LogicOperations
}

export interface FunctionRegistryResponse {
  count: number;
  functions: Record<string, FunctionDef>; // Key is function name
}

export const functionsApi = {
  list: () => apiRequest<FunctionRegistryResponse>('/api/v1/global-functions'),
  create: (payload: FunctionDef) =>
    apiRequest<{ success: boolean; message: string }>('/api/v1/global-functions', {
      method: 'POST',
      body: JSON.stringify(payload),
    }),
  delete: (name: string) =>
    apiRequest<{ success: boolean; message: string }>(`/api/v1/global-functions/${name}`, {
      method: 'DELETE',
    }),
};
