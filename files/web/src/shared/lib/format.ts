/** Formatting helpers that follow the UI language (shared/i18n.ts). */
import { locale } from '@/shared/i18n.ts';

/** Date and time of a Unix-ms timestamp ("20 Oct 2026, 17:35"). */
export function formatDateTime(ms: number | null | undefined): string {
  if (!ms) return '—';
  return new Intl.DateTimeFormat(locale.value, { dateStyle: 'medium', timeStyle: 'short' }).format(
    new Date(ms),
  );
}

/** Relative time of a Unix-ms timestamp ("5 min ago", "in 2 min"). */
export function formatRelative(ms: number | null | undefined, now = Date.now()): string {
  if (!ms) return '—';
  const seconds = Math.round((ms - now) / 1000);
  const rtf = new Intl.RelativeTimeFormat(locale.value, { numeric: 'auto' });
  const abs = Math.abs(seconds);
  if (abs < 60) return rtf.format(seconds, 'second');
  if (abs < 3600) return rtf.format(Math.round(seconds / 60), 'minute');
  if (abs < 86400) return rtf.format(Math.round(seconds / 3600), 'hour');
  return rtf.format(Math.round(seconds / 86400), 'day');
}

/** Milliseconds as "850 ms" / "1.2 s". */
export function formatMs(ms: number | null | undefined): string {
  if (ms === null || ms === undefined) return '—';
  return ms < 1000 ? `${Math.round(ms)} ms` : `${(ms / 1000).toFixed(1)} s`;
}

/** Compact numbers: 1 234 / 12.3K / 4.5M. */
export function formatNumber(value: number | null | undefined): string {
  if (value === null || value === undefined) return '—';
  return new Intl.NumberFormat(locale.value, {
    notation: 'compact',
    maximumFractionDigits: 1,
  }).format(value);
}

/** 0..1 as a percentage. */
export function formatPercent(value: number | null | undefined): string {
  if (value === null || value === undefined) return '—';
  return `${Math.round(value * 100)}%`;
}
