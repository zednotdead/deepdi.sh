import { type MetaFunction } from 'react-router';

export const meta: MetaFunction = () => {
  return [
    { title: 'New Remix App' },
    { name: 'description', content: 'Welcome to Remix!' },
  ];
};

export default function Index() {
  return (
    <div className="font-sans p-4 text-text bg-background-950 h-[200vh]">
      <h1 className="text-3xl">Recipes</h1>
      <p>Listing all recipes not implemented yet</p>
    </div>
  );
}
