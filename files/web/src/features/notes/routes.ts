import { NotebookPen } from '@lucide/vue';
import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/app/notes',
    name: 'notes',
    component: () => import('./pages/NotesPage.vue'),
    meta: { titleKey: 'notes.title', nav: { icon: NotebookPen, order: 10 } },
  },
] satisfies FeatureRoute[];
