import { HeadingNode, QuoteNode } from '@lexical/rich-text';
import { CodeNode } from '@lexical/code';
import { ListItemNode, ListNode } from '@lexical/list';
import { LinkNode } from '@lexical/link';
import { HorizontalRuleNode } from '@lexical/react/LexicalHorizontalRuleNode';
import type { EditorThemeClasses } from 'lexical';
import { ParagraphNode } from 'lexical';

export const EDITOR_NODES = [
  HeadingNode,
  QuoteNode,
  ParagraphNode,
  HorizontalRuleNode,
  CodeNode,
  ListNode,
  ListItemNode,
  LinkNode,
];

export const theme: EditorThemeClasses = {
  paragraph: 'mb-1 last:mb-0 font-body',
  heading: {
    h1: 'font-heading mb-1 text-2xl',
    h2: 'font-heading mb-1 text-xl',
    h3: 'font-heading mb-1 text-lg font-extrabold',
    h4: 'font-heading mb-1 text-lg',
  },
  hr: 'background-red',
  link: 'underline decoration-wavy decoration-1 text-primary-300',
};
