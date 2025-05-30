import { useState } from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { useListWorkspaces, useOpenWorkspace } from "@/hooks/workbench";

import WelcomePageLink from "./WelcomePageLink";

export const WelcomePageRecentWorkspaces = () => {
  const { data: workspaces } = useListWorkspaces();
  const { mutate: openWorkspace } = useOpenWorkspace();

  const [showAll, setShowAll] = useState(false);

  const workspacesToShow = !showAll ? workspaces?.slice(0, 3) : workspaces;

  return (
    <div className="flex flex-col gap-2">
      <h2 className="text-lg">Recent</h2>
      <div className="flex flex-col items-start gap-1.5">
        {workspacesToShow?.map((workspace) => (
          <WelcomePageLink
            key={workspace.displayName}
            label={workspace.displayName}
            onClick={() => openWorkspace(workspace.id)}
          />
        ))}
      </div>

      {!showAll && workspaces?.length && workspaces.length > 3 && (
        <div>
          <ButtonNeutralOutlined onClick={() => setShowAll(true)}>More</ButtonNeutralOutlined>
        </div>
      )}
    </div>
  );
};

export default WelcomePageRecentWorkspaces;
