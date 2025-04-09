import type { MetaFunction } from 'react-router';
import { Link } from 'react-router';

import { Centered } from '~/components/centered';
import { Title } from '~/components/headings';
import { makeTitle } from '~/utils/makeTitle';

export const meta: MetaFunction<typeof loader> = () => {
  return [
    { title: makeTitle('Ingredients') },
  ];
};

export function loader() {
  // TODO: Fill in after we implement listing recipes
  return {
  };
}

export default function RecipeList() {
  // const { recipes } = useLoaderData<typeof loader>();

  return (
    <Centered>
      <Title className="text-center">Recipes</Title>
      <p className="font-body text-xl">Recipe listing is currently not implemented yet</p>
      <p className="font-body text-xl">Check back in later!</p>
      <Link
        to="/recipe/create"
        className="font-body font-semibold text-2xl text-center mt-4 bg-background-800 text-text-200 block"
      >
        Click here to add a new recipe
      </Link>
    </Centered>
  );
}
