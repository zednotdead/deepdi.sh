import type { SerializedEditorState, SerializedTextNode } from 'lexical';
import { is } from 'typia';

export function renderToPlaintext<T extends object>(data?: T): string | null {
  if (!data) return null;

  if (is<SerializedEditorState>(data)) {
    return (
      data.root.children.map(renderToPlaintext).join('')
    );
  }
  else if ('children' in data && Array.isArray(data.children)) {
    return data.children.map(renderToPlaintext).join('');
  }
  else if (is<SerializedTextNode>(data)) {
    return data.text;
  }
  else {
    return '';
  }
}
