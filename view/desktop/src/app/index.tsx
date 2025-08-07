import { ReactNode } from "react";

import Provider from "./Provider";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  return (
    <Provider>
      {/* <div className="absolute top-0 left-1/3 z-50 h-full w-px bg-[royalblue]" />
      <div className="absolute top-0 left-2/3 z-50 h-full w-px bg-[royalblue]" /> */}
      {children}
    </Provider>
  );
};

export default App;
