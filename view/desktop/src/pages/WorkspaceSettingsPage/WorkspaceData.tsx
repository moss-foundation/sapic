import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";

export const WorkspaceData = () => {
  return (
    <div className="mt-6">
      <h3 className="mb-4 font-medium text-[var(--moss-select-text-outlined)]">Data and Storage</h3>
      <div className="space-y-3">
        <div>
          <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
          <p className="text-xs text-gray-500">Last checked: 25 MB</p>
        </div>
        <div>
          <ButtonNeutralOutlined size="md" disabled>
            Clear
          </ButtonNeutralOutlined>
        </div>
      </div>
    </div>
  );
};
