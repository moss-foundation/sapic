import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { SectionTitle } from "./SectionTitle";

interface CollectionDangerZoneSectionProps {
  onDeleteClick: () => void;
}

export const CollectionDangerZoneSection = ({ onDeleteClick }: CollectionDangerZoneSectionProps) => {
  return (
    <div className="text-(--moss-primary-text)">
      <SectionTitle>Danger Zone</SectionTitle>
      <div className="flex items-center justify-between">
        <div className="mr-4 flex-1">
          <p className="mb-1 font-medium">Delete this collection</p>
          <p className="text-sm text-(--moss-secondary-text)">
            Once you delete a collection, there is no going back. Please be certain.
          </p>
        </div>
        <ButtonNeutralOutlined
          size="md"
          onClick={onDeleteClick}
          className="!background-(--moss-button-background-delete) hover:!background--(--moss-button-background-delete-hover) !h-7 flex-shrink-0 !text-(--moss-button-text-delete)"
        >
          Delete
        </ButtonNeutralOutlined>
      </div>
    </div>
  );
};
