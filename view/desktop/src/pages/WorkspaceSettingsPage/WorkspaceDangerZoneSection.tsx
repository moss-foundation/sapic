import { Button } from "@/lib/ui";

import { SectionTitle } from "./SectionTitle";

interface WorkspaceDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const WorkspaceDangerZoneSection = ({ onDeleteClick }: WorkspaceDangerZoneSectionProps) => {
  return (
    <div className="mt-8 text-(--moss-primary-foreground)">
      <SectionTitle>Danger Zone</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this workspace</p>
          <p className="text-sm text-(--moss-secondary-foreground)">
            Once you delete a workspace, there is no going back. Please be certain.
          </p>
        </div>
        <Button intent="danger" onClick={onDeleteClick}>
          Delete
        </Button>
      </div>
    </div>
  );
};
