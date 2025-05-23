import { ReactNode } from "react";
import { PageLoader } from "@/components/PageLoader";
import Provider from "./Provider";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  const { isLoading } = useDescribeAppState();

  if (isLoading) {
    return <PageLoader />;
  }

  return <Provider>{children}</Provider>;
};

export default App;
