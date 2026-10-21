/** HTTP calls of the profile page (identity service, docs/api.md). */
import { http } from '@/shared/api/http.ts';

export const profileApi = {
  changePassword: (currentPassword: string, newPassword: string) =>
    http.post<undefined>('/api/me/password', { currentPassword, newPassword }),
};
