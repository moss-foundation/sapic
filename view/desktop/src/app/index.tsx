import { ReactNode } from "react";

import { Outlet, useParams, useSearch } from "@tanstack/react-router";

import Providers from "./Providers";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  const params = useParams({ strict: false });
  const search = useSearch({ strict: false });
  console.log({
    params,
    search,
  });
  return (
    <Providers>
      <Outlet />
    </Providers>
  );
};

export default App;
