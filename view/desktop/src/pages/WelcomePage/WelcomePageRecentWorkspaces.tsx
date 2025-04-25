import { useEffect, useState } from "react";

import { Button } from "@/components";
import { useGetWorkspaces } from "@/hooks/useGetWorkspaces";
import { useOpenWorkspace } from "@/hooks/useOpenWorkspace";
import { useWorkspaceStore } from "@/store/workspace";

import WelcomePageLink from "./WelcomePageLink";

export const WelcomePageRecentWorkspaces = () => {
  const { data: workspaces } = useGetWorkspaces();
  const { mutate: openWorkspace, data: currentWorkspace } = useOpenWorkspace();
  const { setWorkspace } = useWorkspaceStore();

  const [showAll, setShowAll] = useState(false);

  const workspacesToShow = !showAll ? workspaces?.slice(0, 3) : workspaces;

  useEffect(() => {
    if (currentWorkspace?.path) setWorkspace(currentWorkspace.path);
  }, [currentWorkspace, setWorkspace]);

  return (
    <div className="flex flex-col gap-2">
      <h2 className="text-lg">Recent</h2>
      <div className="flex flex-col items-start gap-1.5">
        {workspacesToShow?.map((workspace) => (
          <WelcomePageLink key={workspace.name} label={workspace.name} onClick={() => openWorkspace(workspace.name)} />
        ))}
      </div>

      {!showAll && (
        <div>
          <Button variant="outlined" intent="neutral" onClick={() => setShowAll(true)}>
            More
          </Button>
        </div>
      )}
    </div>
  );
};

export default WelcomePageRecentWorkspaces;
