<script setup lang="ts">
/** Notes: a grid of cards with create, edit, pin and delete. */
import { NotebookPen, Plus } from '@lucide/vue';
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { errorMessage } from '@/shared/api/http.ts';
import { toast } from '@/shared/toast.ts';
import {
  ConfirmDialog,
  EmptyState,
  GlassCard,
  PageHeader,
  UiButton,
  UiLoader,
} from '@/shared/ui/index.ts';
import type { Note, NoteInput } from '../api.ts';
import NoteCard from '../components/NoteCard.vue';
import NoteForm from '../components/NoteForm.vue';
import { useNotesStore } from '../store.ts';

const { t } = useI18n();
const store = useNotesStore();
const formOpen = ref(false);
const editing = ref<Note | null>(null);
const saving = ref(false);
const deleting = ref<Note | null>(null);
const confirmOpen = ref(false);

onMounted(() => store.load());

function openForm(note: Note | null) {
  editing.value = note;
  formOpen.value = true;
}

async function save(input: NoteInput) {
  saving.value = true;
  try {
    if (editing.value) await store.update(editing.value.id, input);
    else await store.create(input);
    formOpen.value = false;
    toast.success(t('common.saved'));
  } catch (e) {
    toast.error(errorMessage(e));
  } finally {
    saving.value = false;
  }
}

async function togglePin(note: Note) {
  await store.update(note.id, { pinned: !note.pinned }).catch((e) => toast.error(errorMessage(e)));
}

function askDelete(note: Note) {
  deleting.value = note;
  confirmOpen.value = true;
}

async function confirmDelete() {
  const note = deleting.value;
  confirmOpen.value = false;
  if (!note) return;
  try {
    await store.remove(note.id);
    toast.success(t('common.deleted'));
  } catch (e) {
    toast.error(errorMessage(e));
  }
}
</script>

<template>
  <div class="space-y-6">
    <PageHeader :title="t('notes.title')" :description="t('notes.description')">
      <UiButton @click="openForm(null)"><Plus class="size-4" />{{ t('notes.new') }}</UiButton>
    </PageHeader>
    <p v-if="store.error" class="text-sm text-red-600">{{ store.error }}</p>
    <UiLoader v-if="!store.loaded" />
    <GlassCard v-else-if="!store.count">
      <EmptyState :icon="NotebookPen" :title="t('notes.emptyTitle')" :text="t('notes.emptyText')">
        <UiButton class="mt-4" @click="openForm(null)">{{ t('notes.new') }}</UiButton>
      </EmptyState>
    </GlassCard>
    <div v-else class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      <NoteCard
        v-for="note in store.items"
        :key="note.id"
        :note="note"
        @edit="openForm(note)"
        @remove="askDelete(note)"
        @toggle-pin="togglePin(note)"
      />
    </div>
    <NoteForm v-model:open="formOpen" :note="editing" :saving="saving" @save="save" />
    <ConfirmDialog
      v-model:open="confirmOpen"
      :text="t('notes.deleteText', { title: deleting?.title ?? '' })"
      :confirm-label="t('common.delete')"
      danger
      @confirm="confirmDelete"
    />
  </div>
</template>
