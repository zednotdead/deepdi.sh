import type { MetaFunction } from 'react-router';
import { Link, useLoaderData } from 'react-router';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';

import { Centered } from '~/components/centered';
import { Title } from '~/components/headings';
import { makeTitle } from '~/utils/makeTitle';

export const meta: MetaFunction<typeof loader> = () => {
  return [
    { title: makeTitle('Ingredients') },
  ];
};

export async function loader() {
  const res = await fetch('http://localhost:8111/ingredient', {
  });
  const ingredients: IngredientDTO[] = await res.json();

  return {
    ingredients,
  };
}

export default function IngredientList() {
  const { ingredients } = useLoaderData<typeof loader>();

  return (
    <Centered>
      <Title>Ingredients</Title>
      {ingredients.map(ingredient => (
        <ul key={ingredient.id}>
          <li>
            <Link to={`/ingredient/${ingredient.id}`}>{ingredient.name}</Link>
          </li>
        </ul>
      ))}
    </Centered>
  );
}
