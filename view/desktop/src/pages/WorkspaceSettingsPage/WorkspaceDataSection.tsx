import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { SectionTitle } from "./SectionTitle";

export const WorkspaceDataSection = () => {
  return (
    <div className="mt-10 text-(--moss-primary-text)">
      <SectionTitle>Data and Storage</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this workspace</p>
          <p className="text-sm text-(--moss-secondary-text)">Last checked: 25 MB</p>
        </div>
        <ButtonNeutralOutlined size="md" disabled>
          Clear
        </ButtonNeutralOutlined>
      </div>
    </div>
  );
};
