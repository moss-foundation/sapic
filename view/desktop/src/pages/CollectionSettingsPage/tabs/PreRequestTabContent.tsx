export const PreRequestTabContent = () => {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Pre-request Script</h3>
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium text-(--moss-primary-text)">
              JavaScript code to execute before sending requests
            </label>
            <textarea
              rows={10}
              placeholder="// Add your pre-request script here
// Examples:
// pm.globals.set('timestamp', Date.now());
// pm.environment.set('auth_token', pm.response.json().token);"
              className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-3 font-mono text-sm text-(--moss-primary-text)"
            />
          </div>

          <div className="flex gap-2">
            <button className="rounded bg-(--moss-info-background) px-4 py-2 text-(--moss-primary) hover:opacity-80">
              Save Script
            </button>
            <button className="hover:background-(--moss-secondary-background) rounded border border-(--moss-border-color) px-4 py-2 text-(--moss-primary-text)">
              Test Script
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
