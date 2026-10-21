/** Health of the backend services through the edge (`GET /api/health`, docs/api.md). */
import { http } from '@/shared/api/http.ts';

export interface ServiceHealth {
  name: string;
  ok: boolean;
  latencyMs: number;
  version: string | null;
}

export interface Health {
  status: 'ok' | 'degraded';
  appName: string;
  services: ServiceHealth[];
}

export const fetchHealth = () => http.get<Health>('/api/health');
