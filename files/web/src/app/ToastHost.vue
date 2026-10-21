<script setup lang="ts">
import { CircleAlert, CircleCheck, Info, X } from '@lucide/vue';
import { dismiss, toasts } from '@/shared/toast.ts';

const icons = { success: CircleCheck, error: CircleAlert, info: Info };
const colors = { success: 'text-emerald-500', error: 'text-red-500', info: 'text-accent-500' };
</script>

<template>
  <div
    class="pointer-events-none fixed right-4 bottom-4 z-[60] flex w-full max-w-sm flex-col gap-2"
  >
    <TransitionGroup
      enter-active-class="transition duration-200"
      enter-from-class="translate-y-2 opacity-0"
      leave-active-class="transition duration-150"
      leave-to-class="opacity-0"
    >
      <div
        v-for="item in toasts"
        :key="item.id"
        class="glass-strong pointer-events-auto flex items-start gap-3 px-4 py-3 text-sm"
      >
        <component
          :is="icons[item.kind]"
          class="mt-0.5 size-4 shrink-0"
          :class="colors[item.kind]"
        />
        <p class="flex-1">{{ item.message }}</p>
        <button
          type="button"
          class="text-stone-400 hover:text-stone-700 dark:hover:text-white"
          @click="dismiss(item.id)"
        >
          <X class="size-4" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>
