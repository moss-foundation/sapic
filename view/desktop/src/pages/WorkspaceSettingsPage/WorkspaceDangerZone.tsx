import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { SectionTitle } from "./SectionTitle";

export const WorkspaceDangerZone = () => {
  return (
    <div className="mt-6">
      <SectionTitle className="text-black">Danger Zone</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
          <p className="text-xs text-gray-500">
            Once you delete a workspace, there is no going back. Please be certain.
          </p>
        </div>
        <ButtonNeutralOutlined
          size="md"
          className="!h-[28px] !border-red-600 !bg-red-600 !text-white hover:!bg-red-700"
        >
          Delete
        </ButtonNeutralOutlined>
      </div>
    </div>
  );
};
