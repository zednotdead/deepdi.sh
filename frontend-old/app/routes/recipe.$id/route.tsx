import type { LoaderFunctionArgs, MetaFunction } from 'react-router';
import { redirect, useLoaderData } from 'react-router';
import { type RecipeDTO } from 'common/bindings/RecipeDTO';

import { Recipe } from '~/components/recipe/recipe';
import { Centered } from '~/components/centered';
import { makeTitle } from '~/utils/makeTitle';
import { DietList } from '~/components/ingredients/dietList';
import typia from 'typia';

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  return [
    { title: makeTitle(data?.recipe.name) },
  ];
};

export async function loader({ params }: LoaderFunctionArgs) {
  if (!params.id) return redirect('/');

  const recipe: RecipeDTO | undefined = await fetch(`http://localhost:8111/recipe/${params.id}`)
    .then((res) => {
      if (res.status !== 200) {
        return undefined;
      }
      return typia.assert<RecipeDTO>(res.json());
    });

  if (!recipe) return redirect('/');

  return {
    id: params.id,
    recipe,
  };
}

export default function RecipeRoute() {
  const { recipe } = useLoaderData<typeof loader>();

  return (
    <Centered>
      <DietList
        className="2xl:absolute top-24 left-[calc(50%_-_768px_+_2rem)] mt-2 2xl:mt-0 w-full 2xl:w-80"
        diets={recipe.diet_violations}
        type="recipe"
      />
      <Recipe recipe={recipe} />
    </Centered>
  );
}
