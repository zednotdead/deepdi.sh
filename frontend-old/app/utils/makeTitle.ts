export function makeTitle(text?: string): string {
  if (!text) {
    return 'deepdi.sh';
  }

  return `${text} — deepdi.sh`;
}
