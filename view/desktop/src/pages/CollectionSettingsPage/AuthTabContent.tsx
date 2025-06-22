export const AuthTabContent = () => {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Authentication</h3>
        <div className="space-y-4">
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <h4 className="mb-2 font-medium text-(--moss-primary-text)">Auth Type</h4>
            <select className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)">
              <option>No Auth</option>
              <option>Bearer Token</option>
              <option>Basic Auth</option>
              <option>API Key</option>
            </select>
          </div>

          <div className="flex items-center gap-2">
            <div className="h-2 w-2 rounded-full bg-green-500"></div>
            <span className="text-sm text-(--moss-primary-text)">Authentication configured</span>
          </div>
        </div>
      </div>
    </div>
  );
};
