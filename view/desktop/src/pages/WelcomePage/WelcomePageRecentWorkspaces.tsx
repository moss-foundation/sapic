import { useState } from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import { useGetWorkspaces } from "@/hooks/workspaces/useGetWorkspaces";

import WelcomePageLink from "./WelcomePageLink";

export const WelcomePageRecentWorkspaces = () => {
  const { data: workspaces } = useGetWorkspaces();
  const { openAndSelectWorkspace } = useWorkspaceContext();

  const [showAll, setShowAll] = useState(false);

  const workspacesToShow = !showAll ? workspaces?.slice(0, 3) : workspaces;

  return (
    <div className="flex flex-col gap-2">
      <h2 className="text-lg">Recent</h2>
      <div className="flex flex-col items-start gap-1.5">
        {workspacesToShow?.map((workspace) => (
          <WelcomePageLink
            key={workspace.name}
            label={workspace.name}
            onClick={() => openAndSelectWorkspace(workspace.name)}
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
