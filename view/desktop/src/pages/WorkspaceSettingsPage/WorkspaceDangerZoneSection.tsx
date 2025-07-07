import ButtonDanger from "@/components/ButtonDanger";

import { SectionTitle } from "./SectionTitle";

interface WorkspaceDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const WorkspaceDangerZoneSection = ({ onDeleteClick }: WorkspaceDangerZoneSectionProps) => {
  return (
    <div className="mt-8 text-(--moss-primary-text)">
      <SectionTitle>Danger Zone</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this workspace</p>
          <p className="text-sm text-(--moss-secondary-text)">
            Once you delete a workspace, there is no going back. Please be certain.
          </p>
        </div>
        <ButtonDanger size="md" onClick={onDeleteClick}>
          Delete
        </ButtonDanger>
      </div>
    </div>
  );
};
