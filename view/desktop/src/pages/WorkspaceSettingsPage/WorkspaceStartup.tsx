interface WorkspaceStartupProps {
  reopenOnNextSession: boolean;
  setReopenOnNextSession: (value: boolean) => void;
  openPreviousWindows: boolean;
  setOpenPreviousWindows: (value: boolean) => void;
}

export const WorkspaceStartup = ({
  reopenOnNextSession,
  setReopenOnNextSession,
  openPreviousWindows,
  setOpenPreviousWindows,
}: WorkspaceStartupProps) => {
  return (
    <div className="mt-6">
      <h3 className="mb-4 font-medium text-[var(--moss-select-text-outlined)]">Startup</h3>
      <div className="space-y-3">
        <label className="flex items-center gap-3">
          <input
            type="checkbox"
            checked={reopenOnNextSession}
            onChange={(e) => setReopenOnNextSession(e.target.checked)}
            className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span className="text-sm text-[var(--moss-select-text-outlined)]">Reopen this workspace on next session</span>
        </label>
        <label className="flex items-center gap-3">
          <input
            type="checkbox"
            checked={openPreviousWindows}
            onChange={(e) => setOpenPreviousWindows(e.target.checked)}
            className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span className="text-sm text-[var(--moss-select-text-outlined)]">Open previous windows and tabs</span>
        </label>
      </div>
    </div>
  );
};
