/** Desktop sidebar: collapsed or not, remembered in localStorage. */
import { ref, watch } from 'vue';

const KEY = 'sidebar-collapsed';

function read(): boolean {
  try {
    return localStorage.getItem(KEY) === '1';
  } catch {
    return false;
  }
}

const collapsed = ref(read());

watch(collapsed, (value) => {
  try {
    localStorage.setItem(KEY, value ? '1' : '0');
  } catch {
    // Storage is unavailable (private mode): the choice lasts until reload.
  }
});

export function useSidebar() {
  return { collapsed, toggle: () => (collapsed.value = !collapsed.value) };
}
