<script setup lang="ts">
/** Create or edit a note in a dialog. Emits `save` with the input; the page calls the store. */
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { UiButton, UiInput, UiModal, UiTextarea } from '@/shared/ui/index.ts';
import type { Note, NoteInput } from '../api.ts';

const props = defineProps<{ note: Note | null; saving: boolean }>();
const emit = defineEmits<{ save: [input: NoteInput] }>();
const open = defineModel<boolean>('open', { default: false });
const { t } = useI18n();

const title = ref('');
const body = ref('');
watch(open, (value) => {
  if (!value) return;
  title.value = props.note?.title ?? '';
  body.value = props.note?.body ?? '';
});

function submit() {
  if (title.value.trim()) emit('save', { title: title.value.trim(), body: body.value });
}
</script>

<template>
  <UiModal v-model:open="open" :title="note ? t('notes.editTitle') : t('notes.newTitle')">
    <form class="space-y-4" @submit.prevent="submit">
      <UiInput v-model="title" :label="t('notes.fields.title')" required />
      <UiTextarea v-model="body" :label="t('notes.fields.body')" :rows="8" />
      <div class="flex justify-end gap-2">
        <UiButton variant="ghost" @click="open = false">{{ t('common.cancel') }}</UiButton>
        <UiButton type="submit" :loading="saving" :disabled="!title.trim()">
          {{ t('common.save') }}
        </UiButton>
      </div>
    </form>
  </UiModal>
</template>
