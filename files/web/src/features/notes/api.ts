/** HTTP calls of the notes service (`/api/notes`, docs/api.md). */
import { http } from '@/shared/api/http.ts';

export interface Note {
  id: string;
  title: string;
  body: string;
  pinned: boolean;
  createdAt: number;
  updatedAt: number;
}

export interface NoteInput {
  title: string;
  body: string;
  pinned?: boolean;
}

export const notesApi = {
  list: async () => (await http.get<{ items: Note[] }>('/api/notes')).items,
  create: (input: NoteInput) => http.post<Note>('/api/notes', input),
  update: (id: string, patch: Partial<NoteInput>) => http.patch<Note>(`/api/notes/${id}`, patch),
  remove: (id: string) => http.delete(`/api/notes/${id}`),
};
