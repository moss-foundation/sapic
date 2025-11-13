import { useState } from "react";

import { useListWorkspaces, useOpenWorkspace } from "@/hooks/workbench";
import { Button } from "@/lib/ui";

import WelcomeViewLink from "./WelcomeViewLink";

export const WelcomeViewRecentWorkspaces = () => {
  const { data: workspaces } = useListWorkspaces();
  const { mutate: openWorkspace } = useOpenWorkspace();

  const [showAll, setShowAll] = useState(false);

  const workspacesToShow = !showAll ? workspaces?.slice(0, 3) : workspaces;

  return (
    <div className="flex flex-col gap-2">
      <h2 className="text-lg">Recent</h2>
      <div className="flex flex-col items-start gap-1.5">
        {workspacesToShow?.map((workspace) => (
          <WelcomeViewLink key={workspace.id} label={workspace.name} onClick={() => openWorkspace(workspace.id)} />
        ))}

        {workspaces?.length === 0 && <span className="text-(--moss-secondary-foreground)">No recent workspaces</span>}
      </div>

      {!showAll && workspaces && workspaces.length > 3 && (
        <div>
          <Button intent="outlined" onClick={() => setShowAll(true)}>
            More
          </Button>
        </div>
      )}
    </div>
  );
};

export default WelcomeViewRecentWorkspaces;
