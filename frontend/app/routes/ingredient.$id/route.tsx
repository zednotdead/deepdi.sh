import type { LoaderFunctionArgs, MetaFunction } from 'react-router';
import { redirect, useLoaderData } from 'react-router';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';
import { trace, Span } from '@opentelemetry/api';

import { Centered } from '~/components/centered';
import { DietList } from '~/components/ingredients/dietList';
import { Title } from '~/components/headings';
import { LexicalToReact } from '~/components/editor/renderReact';
import { makeTitle } from '~/utils/makeTitle';
import { safeEditorStateParse } from '~/components/editor/utils';

const tracer = trace.getTracer('dice-lib');

export async function loader({ params }: LoaderFunctionArgs) {
  if (!params.id) return redirect('/');

  tracer.startActiveSpan('load ingredients', (s: Span) => {
    console.log(2 + 2);
    s.setAttribute('test', 4);
    s.end();

    return;
  });

  const ingredient: IngredientDTO | undefined = await fetch(`http://localhost:8111/ingredient/${params.id}`)
    .then((res) => {
      if (res.status !== 200) {
        return undefined;
      }

      return res.json();
    });

  if (!ingredient) return redirect('/');

  return {
    id: params.id,
    ingredient,
  };
}

export const meta: MetaFunction<typeof loader> = ({ data }) => [
  { title: makeTitle(data?.ingredient.name) },
];

export default function IngredientRoute() {
  const { ingredient } = useLoaderData<typeof loader>();
  const description = safeEditorStateParse(ingredient.description);

  return (
    <Centered className="p-2">
      <Title>{ingredient.name}</Title>
      <DietList
        className="2xl:absolute top-24 left-[calc(50%_-_768px_+_2rem)] mt-2 2xl:mt-0 w-full 2xl:w-80"
        diets={ingredient.diet_violations}
      />
      <LexicalToReact data={description} />
    </Centered>
  );
}
