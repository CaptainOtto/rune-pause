export function applyTheme(mode: 'system'|'dark'|'light'){
  const root = document.documentElement;
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  const isLight = mode==='light' || (mode==='system' && !prefersDark);
  root.classList.toggle('light', isLight);
}
