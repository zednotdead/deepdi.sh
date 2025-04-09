import { SerializedEditorState, SerializedParagraphNode, SerializedTextNode } from 'lexical';

export const EMPTY_RTE = {
  root: {
    children: [
      {
        children: [],
        direction: 'ltr',
        format: '',
        indent: 0,
        type: 'paragraph',
        version: 1,
        textFormat: 0,
        textStyle: '',
      },
    ],
    direction: 'ltr',
    format: '',
    indent: 0,
    type: 'root',
    version: 1,
  },
} as SerializedEditorState<SerializedParagraphNode>;

export function makeEditorStateFromString(text: string): SerializedEditorState<SerializedParagraphNode> {
  return {
    root:
        {
          children: [
            {
              children: [
                {
                  detail: 0,
                  format: 0,
                  mode: 'normal',
                  style: '',
                  text,
                  type: 'text',
                  version: 1,
                } as SerializedTextNode,
              ],
              direction: 'ltr',
              format: '',
              indent: 0,
              type: 'paragraph',
              version: 1,
              textFormat: 0,
              textStyle: '',
            },
          ], direction: 'ltr', format: '', indent: 0, type: 'root', version: 1,
        },
  };
}

export function safeEditorStateParse(text: string): SerializedEditorState<SerializedParagraphNode> {
  try {
    const json = JSON.parse(text) as SerializedEditorState<SerializedParagraphNode>;
    return json;
  }
  catch {
    return makeEditorStateFromString(text);
  }
}
