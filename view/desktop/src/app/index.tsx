import { ReactNode } from "react";

import { Outlet } from "@tanstack/react-router";

import Providers from "./Providers";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  return (
    <Providers>
      <Outlet />
    </Providers>
  );
};

export default App;
