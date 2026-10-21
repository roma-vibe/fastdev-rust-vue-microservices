<script setup lang="ts">
/** Dialog over a blurred backdrop. `v-model:open`; closes on Escape and backdrop click. */
import { X } from '@lucide/vue';
import { onBeforeUnmount, watch } from 'vue';

const props = withDefaults(defineProps<{ title?: string; size?: 'sm' | 'md' | 'lg' | 'xl' }>(), {
  size: 'md',
});
const open = defineModel<boolean>('open', { default: false });
const widths = { sm: 'max-w-sm', md: 'max-w-lg', lg: 'max-w-2xl', xl: 'max-w-4xl' };

function onKey(event: KeyboardEvent) {
  if (event.key === 'Escape') open.value = false;
}
watch(open, (value) => {
  if (value) window.addEventListener('keydown', onKey);
  else window.removeEventListener('keydown', onKey);
});
onBeforeUnmount(() => window.removeEventListener('keydown', onKey));
</script>

<template>
  <Teleport to="body">
    <Transition name="modal" :duration="{ enter: 240, leave: 180 }">
      <div v-if="open" class="fixed inset-0 z-50 flex items-end justify-center p-4 sm:items-center">
        <div class="modal-backdrop absolute inset-0" @click="open = false" />
        <div
          role="dialog"
          aria-modal="true"
          class="modal-panel glass-strong relative flex max-h-[90vh] w-full flex-col"
          :class="widths[props.size]"
        >
          <header
            v-if="props.title || $slots.header"
            class="flex items-center justify-between gap-4 px-6 pt-5"
          >
            <slot name="header">
              <h2 class="text-lg font-semibold">{{ props.title }}</h2>
            </slot>
            <button
              type="button"
              class="rounded-lg p-1 text-stone-500 hover:bg-stone-900/5 dark:hover:bg-white/10"
              @click="open = false"
            >
              <X class="size-5" />
            </button>
          </header>
          <div class="overflow-y-auto px-6 py-5"><slot /></div>
          <footer
            v-if="$slots.footer"
            class="flex justify-end gap-2 border-t border-stone-900/5 px-6 py-4 dark:border-white/10"
          >
            <slot name="footer" />
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
