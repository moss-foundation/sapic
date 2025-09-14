import ButtonDanger from "@/components/ButtonDanger";

import { SectionTitle } from "./SectionTitle";

interface ProjectDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const ProjectDangerZoneSection = ({ onDeleteClick }: ProjectDangerZoneSectionProps) => {
  return (
    <div className="mt-8 text-(--moss-primary-text)">
      <SectionTitle>Danger Zone</SectionTitle>

      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this project</p>
          <p className="text-sm text-(--moss-secondary-text)">
            Once you delete a project, there is no going back. Please be certain.
          </p>
        </div>

        <ButtonDanger size="md" onClick={onDeleteClick}>
          Delete
        </ButtonDanger>
      </div>
    </div>
  );
};
