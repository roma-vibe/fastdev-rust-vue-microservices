/** HTTP calls of personal API keys (identity service `/api/keys`, docs/api.md). */
import { http } from '@/shared/api/http.ts';

export interface ApiKey {
  id: string;
  name: string;
  prefix: string;
  createdAt: number;
  lastUsedAt: number | null;
  revokedAt: number | null;
}

export interface CreatedKey {
  key: ApiKey;
  /** The full secret; the server never shows it again. */
  secret: string;
}

export const keysApi = {
  list: async () => (await http.get<{ items: ApiKey[] }>('/api/keys')).items,
  create: (name: string) => http.post<CreatedKey>('/api/keys', { name }),
  revoke: (id: string) => http.delete(`/api/keys/${id}`),
};
