import { useListWorkspaces } from "@/hooks";
import { useGetLayout } from "@/workbench/adapters/tanstackQuery/layout";
import { Outlet, useParams } from "@tanstack/react-router";

const App = () => {
  const { workspaceId } = useParams({ strict: false });

  const { data: layout } = useGetLayout({ workspaceId });
  const { data: workspaces } = useListWorkspaces();

  console.log({ params: { workspaceId }, tanstackData: { layout, workspaces } });

  return <Outlet />;
};

export default App;
