export const PostRequestTabContent = () => {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Post-request Script</h3>
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium text-(--moss-primary-text)">
              JavaScript code to execute after receiving responses
            </label>
            <textarea
              rows={10}
              placeholder="// Add your post-request script here
// Examples:
// pm.test('Status code is 200', function () {
//     pm.response.to.have.status(200);
// });
// pm.globals.set('response_data', pm.response.json());"
              className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-3 font-mono text-sm text-(--moss-primary-text)"
            />
          </div>

          <div className="flex gap-2">
            <button className="background-(--moss-info-background) rounded px-4 py-2 text-(--moss-primary) hover:opacity-80">
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
