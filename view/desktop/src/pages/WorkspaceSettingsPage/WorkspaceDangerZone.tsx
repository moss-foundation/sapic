import ButtonPrimary from "@/components/ButtonPrimary";

export const WorkspaceDangerZone = () => {
  return (
    <div className="mt-6">
      <h3 className="mb-4 font-medium text-red-600">Danger Zone</h3>
      <div className="space-y-3">
        <div>
          <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
          <p className="text-xs text-gray-500">
            Once you delete a workspace, there is no going back. Please be certain.
          </p>
        </div>
        <div>
          <ButtonPrimary size="md" disabled className="bg-red-600 text-white hover:bg-red-700 disabled:bg-red-400">
            Delete
          </ButtonPrimary>
        </div>
      </div>
    </div>
  );
};
