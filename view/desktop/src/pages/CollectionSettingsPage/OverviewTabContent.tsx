export const OverviewTabContent = () => {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Collection Overview</h3>
        <div className="grid grid-cols-2 gap-4">
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <h4 className="mb-2 font-medium text-(--moss-primary-text)">Total Requests</h4>
            <p className="text-2xl font-bold text-(--moss-primary)">4</p>
          </div>
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <h4 className="mb-2 font-medium text-(--moss-primary-text)">Last Modified</h4>
            <p className="text-sm text-(--moss-secondary-text)">24.05.2025</p>
          </div>
        </div>
      </div>

      <div>
        <h4 className="mb-3 font-medium text-(--moss-primary-text)">Recent Activity</h4>
        <div className="space-y-2">
          <div className="background-(--moss-secondary-background) rounded-md p-3">
            <p className="text-sm text-(--moss-primary-text)">Collection created</p>
            <p className="text-xs text-(--moss-secondary-text)">24.05.2025</p>
          </div>
        </div>
      </div>
    </div>
  );
};
