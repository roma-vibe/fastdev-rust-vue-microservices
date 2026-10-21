<template>
  <!-- Coloured light behind the glass: soft orange/amber blobs and a fine grain. -->
  <div class="pointer-events-none fixed inset-0 -z-10 overflow-hidden" aria-hidden="true">
    <div
      class="absolute -top-40 -left-32 size-[38rem] rounded-full bg-accent-400/35 blur-3xl dark:bg-accent-600/25"
    />
    <div
      class="absolute top-1/3 -right-40 size-[34rem] rounded-full bg-amber-300/40 blur-3xl dark:bg-amber-700/15"
    />
    <div
      class="absolute -bottom-48 left-1/4 size-[40rem] rounded-full bg-rose-300/25 blur-3xl dark:bg-rose-900/20"
    />
    <div
      class="absolute inset-0 opacity-[0.035] mix-blend-multiply dark:opacity-[0.06] dark:mix-blend-screen"
      style="
        background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='160' height='160'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='.8' numOctaves='3'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E&quot;);
      "
    />
  </div>
</template>
