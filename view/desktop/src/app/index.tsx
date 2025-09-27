import { ReactNode } from "react";

import { NotificationContainer } from "@/components/NotificationContainer";
import Providers from "./Providers";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  return (
    <Providers>
      {children}
      <NotificationContainer />
    </Providers>
  );
};

export default App;
