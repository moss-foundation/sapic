import { ReactNode } from "react";

import { useListWorkspaces } from "@/hooks";
import { useGetLayout } from "@/workbench/adapters/tanstackQuery/layout";
import { Outlet, useParams } from "@tanstack/react-router";

import Providers from "./Providers";

interface AppProps {
  children?: ReactNode;
}

const App = ({ children }: AppProps) => {
  const { workspaceId } = useParams({ strict: false });

  const { data: layout } = useGetLayout({ workspaceId });
  const { data: workspaces } = useListWorkspaces();

  console.log({ params: { workspaceId }, tanstackData: { layout, workspaces } });

  return (
    <Providers>
      <Outlet />
    </Providers>
  );
};

export default App;
