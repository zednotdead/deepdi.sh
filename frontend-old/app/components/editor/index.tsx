import type { EditorState, SerializedEditorState } from 'lexical';
import { AutoFocusPlugin } from '@lexical/react/LexicalAutoFocusPlugin';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import type { InitialConfigType } from '@lexical/react/LexicalComposer';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { MarkdownShortcutPlugin } from '@lexical/react/LexicalMarkdownShortcutPlugin';
import { HorizontalRulePlugin } from '@lexical/react/LexicalHorizontalRulePlugin';
import { clsx } from 'clsx';
import { forwardRef, useImperativeHandle } from 'react';

import { EDITOR_NODES, theme } from './settings';
import { renderToPlaintext } from './renderPlaintext';

type EditorProps<T> = {
  id?: string;
  onChange: (data: T) => void;
  value?: T;
  className?: string;
  editable?: boolean;
  name?: string;
};

const ReactUseFormCompatPlugin = forwardRef<Partial<HTMLInputElement>>(function ReactUseFormCompatPlugin(_props, ref) {
  const [$editor] = useLexicalComposerContext();
  useImperativeHandle(ref, () => {
    return {
      focus() {
        $editor.focus();
      },
      blur() {
        $editor.blur();
      },
    };
  }, [$editor]);
  return null;
});

export const Editor = forwardRef<HTMLInputElement, EditorProps<SerializedEditorState | null>>(
  function Editor({
    value,
    id,
    name,
    onChange,
    editable,
    className,
  }, ref) {
    const initialConfig: InitialConfigType = {
      namespace: 'MyEditor',
      theme,
      onError: console.error,
      editorState(editor) {
        if (!value) return;
        const state = editor.parseEditorState(value);

        editor.setEditorState(state);
      },
      nodes: EDITOR_NODES,
      editable,
    };

    const onChangeHandler = (state: EditorState) => {
      const serializedState = state.toJSON();
      const renderedState = renderToPlaintext(serializedState);
      if (renderedState) {
        onChange(serializedState);
      }
      else {
        onChange(null);
      }
    };

    return (
      <div className={clsx('grid grid-cols-1 grid-rows-1', className)}>
        <LexicalComposer initialConfig={initialConfig}>
          <HorizontalRulePlugin />
          <MarkdownShortcutPlugin />
          <ReactUseFormCompatPlugin ref={ref} />
          <RichTextPlugin
            contentEditable={(
              <ContentEditable
                className="col-start-1 col-end-1 row-start-1 row-end-1 outline-none z-0"
                id={id}
                name={name}
              />
            )}
            placeholder={(
              <div
                className="col-start-1 col-end-1 row-start-1 row-end-1 pointer-events-none font-extrabold text-text-800 -z-10"
              >
                Enter some text...
              </div>
            )}
            ErrorBoundary={LexicalErrorBoundary}
          />
          <HistoryPlugin />
          <AutoFocusPlugin />
          <OnChangePlugin onChange={onChangeHandler} />
        </LexicalComposer>
      </div>
    );
  });
