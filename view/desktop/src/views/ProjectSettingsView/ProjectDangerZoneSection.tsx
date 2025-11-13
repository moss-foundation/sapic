import { Button } from "@/lib/ui";

import { SectionTitle } from "./SectionTitle";

interface ProjectDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const ProjectDangerZoneSection = ({ onDeleteClick }: ProjectDangerZoneSectionProps) => {
  return (
    <div className="text-(--moss-primary-foreground) mt-8">
      <SectionTitle>Danger Zone</SectionTitle>

      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this project</p>
          <p className="text-(--moss-secondary-foreground) text-sm">
            Once you delete a project, there is no going back. Please be certain.
          </p>
        </div>

        <Button intent="danger" onClick={onDeleteClick}>
          Delete
        </Button>
      </div>
    </div>
  );
};
