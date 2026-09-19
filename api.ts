/// <reference types="vite/client" />

// API Configuration
export const getApiBaseUrl = () => {
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem('WORPEN_API_URL');
    if (stored) return stored.replace(/\/$/, "");
    const envUrl = (import.meta as any).env?.VITE_API_URL;
    if (envUrl) return String(envUrl).replace(/\/$/, "");
    // Single-binary default: the page is served by Worpen itself,
    // so the API lives on the same origin (any host/port, zero config).
    if (window.location.origin.startsWith('http')) return window.location.origin;
  }
  return 'http://127.0.0.1:3000';
};

// ---- Auth (WORPEN_API_TOKEN) ----
// When the backend sets WORPEN_API_TOKEN, mutating calls + auth-required
// routes need `Authorization: Bearer <token>`. Stored only in localStorage.
export const getToken = (): string | null => {
  if (typeof window === 'undefined') return null;
  const t = localStorage.getItem('WORPEN_TOKEN');
  return t && t.trim() ? t.trim() : null;
};

export const setToken = (token: string | null) => {
  if (typeof window === 'undefined') return;
  if (token && token.trim()) localStorage.setItem('WORPEN_TOKEN', token.trim());
  else localStorage.removeItem('WORPEN_TOKEN');
  window.dispatchEvent(new CustomEvent('worpen:auth-changed'));
};

export const authHeaders = (): Record<string, string> => {
  const t = getToken();
  return t ? { 'Authorization': `Bearer ${t}` } : {};
};

/** fetch() wrapper with auth headers + global 401 event. Use for views with raw fetch calls. */
export const authFetch = (url: string, options?: RequestInit): Promise<Response> => {
  const merged: RequestInit = {
    ...options,
    headers: {
      ...authHeaders(),
      ...options?.headers,
    },
  };
  return fetch(url, merged).then((res) => {
    if (res.status === 401 && typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('worpen:unauthorized', { detail: { endpoint: url } }));
    }
    return res;
  });
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
      ...authHeaders(),
      ...options?.headers,
    },
  });

  if (response.status === 401) {
    // Notify the app (header badge + redirect to Settings)
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('worpen:unauthorized', { detail: { endpoint } }));
    }
    throw new Error(`Unauthorized (401): set your API token in Settings`);
  }

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

// Pipelines API (real multi-stage engine)
export type StageRunStatus = 'SUCCESS' | 'FAILED' | 'SKIPPED';

export interface PipelineStageView {
  name: string;
  allow_failure: boolean;
  last_status?: StageRunStatus | null;
  last_duration_ms?: number | null;
}

export interface PipelineRunResult {
  run_id: string;
  pipeline_id: string;
  pipeline_name: string;
  success: boolean;
  stages: Array<{
    name: string;
    status: StageRunStatus;
    result?: any;
    error?: string | null;
    execution_time_ms: number;
  }>;
  total_time_ms: number;
  finished_at: string;
}

export interface PipelineListItem {
  id: string;
  name: string;
  description: string;
  version: string;
  enabled: boolean;
  stage_count: number;
  stages: PipelineStageView[];
  last_run?: PipelineRunResult | null;
}

export const pipelinesApi = {
  list: () => apiRequest<PipelineListItem[]>('/api/v1/pipelines'),
  trigger: (pipelineId: string, payload: any = {}) =>
    apiRequest<PipelineRunResult>('/api/v1/pipelines/trigger', {
      method: 'POST',
      body: JSON.stringify({ pipelineId, payload }),
    }),
  runs: (pipeline_id?: string, limit: number = 20) =>
    apiRequest<{ count: number; runs: PipelineRunResult[] }>(
      `/api/v1/pipelines/runs?limit=${limit}${pipeline_id ? `&pipeline_id=${pipeline_id}` : ''}`
    ),
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

// Execution audit trail (real backend data)
export interface AuditEntry {
  ts: string;
  route_id: string;
  route_name: string;
  path: string;
  success: boolean;
  error?: string | null;
  execution_time_ms: number;
  via: string;
}

export const auditApi = {
  recent: (limit: number = 50) =>
    apiRequest<{ count: number; entries: AuditEntry[] }>(`/api/v1/dynamic-routes/audit?limit=${limit}`),
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
