/** What the auth feature adds to the shell: the sign-in guard, the admin check and the account widgets. */
import type { ShellExtension } from '@/app/types.ts';
import AccountMenu from './components/AccountMenu.vue';
import SignInButton from './components/SignInButton.vue';
import { useAuthStore } from '@/shared/auth.ts';
import { authGuard } from './guard.ts';

export default {
  guard: authGuard,
  isAdmin: () => useAuthStore().isAdmin,
  sidebarFooter: AccountMenu,
  publicActions: SignInButton,
} satisfies ShellExtension;
