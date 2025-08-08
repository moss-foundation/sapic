import { ReactNode } from "react";

import Provider from "./Provider";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  return <Provider>{children}</Provider>;
};

export default App;
