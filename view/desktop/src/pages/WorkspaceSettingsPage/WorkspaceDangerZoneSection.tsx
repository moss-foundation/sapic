import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { SectionTitle } from "./SectionTitle";

interface WorkspaceDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const WorkspaceDangerZoneSection = ({ onDeleteClick }: WorkspaceDangerZoneSectionProps) => {
  return (
    <div className="mt-6 text-(--moss-primary-text)">
      <SectionTitle>Danger Zone</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this workspace</p>
          <p className="text-sm text-(--moss-secondary-text)">
            Once you delete a workspace, there is no going back. Please be certain.
          </p>
        </div>
        <ButtonNeutralOutlined
          size="md"
          onClick={onDeleteClick}
          className="!h-7 !bg-(--moss-button-background-delete) !text-(--moss-button-text-delete) hover:!bg-(--moss-button-background-delete-hover)"
        >
          Delete
        </ButtonNeutralOutlined>
      </div>
    </div>
  );
};
