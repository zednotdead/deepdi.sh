import "react-router";

declare module "react-router" {
  interface Register {
    params: Params;
  }
}

type Params = {
  "/": {};
  "/ingredient": {};
  "/ingredient/create": {};
  "/ingredient/:id": {
    "id": string;
  };
  "/recipe": {};
  "/recipe/create": {};
  "/recipe/:id": {
    "id": string;
  };
};