/** HTTP call of the admin user list (identity service `/api/admin/users`, docs/api.md). */
import { http } from '@/shared/api/http.ts';
import type { User } from '@/shared/auth.ts';

export interface AdminUser extends User {
  disabled: boolean;
  keyCount: number;
}

export const adminUsersApi = {
  list: async () => (await http.get<{ items: AdminUser[] }>('/api/admin/users')).items,
};
