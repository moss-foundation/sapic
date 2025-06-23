export const HeadersTabContent = () => {
  const headers = [
    { key: "Content-Type", value: "application/json" },
    { key: "Accept", value: "application/json" },
    { key: "User-Agent", value: "Sapic/1.0.0" },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Headers</h3>
        <div className="space-y-3">
          {headers.map((header, index) => (
            <div key={index} className="flex gap-4 rounded-md border border-(--moss-border-color) p-3">
              <div className="flex-1">
                <input
                  type="text"
                  value={header.key}
                  placeholder="Header name"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <div className="flex-1">
                <input
                  type="text"
                  value={header.value}
                  placeholder="Header value"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <button className="p-2 text-(--moss-secondary-text) hover:text-(--moss-error)">×</button>
            </div>
          ))}
          <button className="w-full rounded border-2 border-dashed border-(--moss-border-color) p-3 text-(--moss-secondary-text) hover:border-(--moss-primary) hover:text-(--moss-primary-text)">
            + Add Header
          </button>
        </div>
      </div>
    </div>
  );
};
