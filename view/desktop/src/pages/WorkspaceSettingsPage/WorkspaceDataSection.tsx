import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { SectionTitle } from "./SectionTitle";

export const WorkspaceDataSection = () => {
  return (
    <div className="mt-6">
      <SectionTitle className="text-[var(--moss-select-text-outlined)]">Data and Storage</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
          <p className="text-xs text-gray-500">Last checked: 25 MB</p>
        </div>
        <ButtonNeutralOutlined size="md" disabled>
          Clear
        </ButtonNeutralOutlined>
      </div>
    </div>
  );
};
