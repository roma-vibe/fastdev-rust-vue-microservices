<script setup lang="ts">
/** One note: title, text, update time and actions. */
import { Pencil, Pin, PinOff, Trash2 } from '@lucide/vue';
import { useI18n } from 'vue-i18n';
import { formatRelative } from '@/shared/lib/format.ts';
import { GlassCard } from '@/shared/ui/index.ts';
import type { Note } from '../api.ts';

defineProps<{ note: Note }>();
const emit = defineEmits<{ edit: []; remove: []; togglePin: [] }>();
const { t } = useI18n();
</script>

<template>
  <GlassCard interactive class="flex flex-col gap-2">
    <div class="flex items-start gap-2">
      <h3 class="min-w-0 flex-1 font-semibold break-words">{{ note.title }}</h3>
      <button
        type="button"
        class="rounded-lg p-1 text-stone-500 hover:text-accent-600"
        :title="note.pinned ? t('notes.unpin') : t('notes.pin')"
        @click="emit('togglePin')"
      >
        <component :is="note.pinned ? PinOff : Pin" class="size-4" />
      </button>
    </div>
    <p
      v-if="note.body"
      class="line-clamp-6 text-sm whitespace-pre-line text-stone-600 dark:text-stone-300"
    >
      {{ note.body }}
    </p>
    <div class="mt-auto flex items-center gap-1 pt-2 text-xs text-stone-500">
      <span>{{ formatRelative(note.updatedAt) }}</span>
      <button
        type="button"
        class="ml-auto rounded-lg p-1 hover:text-accent-600"
        :title="t('common.edit')"
        @click="emit('edit')"
      >
        <Pencil class="size-4" />
      </button>
      <button
        type="button"
        class="rounded-lg p-1 hover:text-red-600"
        :title="t('common.delete')"
        @click="emit('remove')"
      >
        <Trash2 class="size-4" />
      </button>
    </div>
  </GlassCard>
</template>
