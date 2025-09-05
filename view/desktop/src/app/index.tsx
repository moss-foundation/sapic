import { ReactNode } from "react";

import Providers from "./Providers";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  return <Providers>{children}</Providers>;
};

export default App;
