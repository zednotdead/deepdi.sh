import type { SerializedHeadingNode } from '@lexical/rich-text';
import type { SerializedEditorState, SerializedLexicalNode, SerializedParagraphNode, SerializedTextNode } from 'lexical';
import { IS_BOLD, IS_CODE, IS_HIGHLIGHT, IS_ITALIC, IS_SUBSCRIPT, IS_SUPERSCRIPT } from 'lexical';
import type { ReactElement } from 'react';
import { Fragment } from 'react/jsx-runtime';
import { is } from 'typia';
import type { SerializedLinkNode } from '@lexical/link';

import { Heading } from '../headings';

import { theme } from './settings';

export function LexicalToReact<T extends object>({ data }: { data?: T }): ReactElement {
  const mapChildrenToElement = (d: SerializedLexicalNode, i: number) => (
    <LexicalToReact data={d} key={d.type + d.version + i} />
  );

  if (is<SerializedEditorState>(data)) {
    return (
      <Fragment>
        {data.root.children.map(mapChildrenToElement)}
      </Fragment>
    );
  }
  else if (is<SerializedTextNode>(data)) {
    const OuterTag = (() => {
      const format = data.format;
      if (format & IS_CODE) {
        return 'code';
      }
      if (format & IS_HIGHLIGHT) {
        return 'mark';
      }
      if (format & IS_SUBSCRIPT) {
        return 'sub';
      }
      if (format & IS_SUPERSCRIPT) {
        return 'sup';
      }

      return Fragment;
    })();

    const InnerTag = (() => {
      const format = data.format;
      if (format & IS_BOLD) {
        return 'strong';
      }
      if (format & IS_ITALIC) {
        return 'em';
      }
      return 'span';
    })();

    return (
      <OuterTag>
        <InnerTag>{data.text}</InnerTag>
      </OuterTag>
    );
  }
  else if (is<SerializedHeadingNode>(data)) {
    return (
      <Heading as={data.tag} className={theme?.heading?.[data.tag] || 'text-xl font-heading'}>
        {data.children.map(mapChildrenToElement)}
      </Heading>
    );
  }
  else if (is<SerializedParagraphNode>(data)) {
    return (
      <p className={theme.paragraph}>
        {data.children.map(mapChildrenToElement)}
      </p>
    );
  }
  else if (is<SerializedLinkNode>(data)) {
    return (
      <a href={data.url} className={theme.link}>
        {data.children.map(mapChildrenToElement)}
      </a>
    );
  }

  console.error('variant not implemented', data);
  return <></>;
}
