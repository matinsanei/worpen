
import { LogEntry } from "./types";

export const MOCK_LOGS: LogEntry[] = [
  { id: '1', timestamp: '10:42:01', level: 'INFO', source: 'CORE', message: 'Heartbeat received from 1,204 agents.' },
  { id: '2', timestamp: '10:42:05', level: 'WARN', source: 'bee-002', message: 'Memory usage spike detected (92%).' },
  { id: '3', timestamp: '10:42:06', level: 'ERROR', source: 'bee-002', message: 'Container [saleor-api] SIGKILL received (OOM).' },
  { id: '4', timestamp: '10:42:06', level: 'INFO', source: 'bee-002', message: 'Strategy matched: [Restart_On_Failure].' },
  { id: '5', timestamp: '10:42:07', level: 'SUCCESS', source: 'bee-002', message: 'Container [saleor-api] successfully restarted. PID: 4521.' },
  { id: '6', timestamp: '10:42:15', level: 'INFO', source: 'CORE', message: 'Fleet synchronization complete.' },
];
