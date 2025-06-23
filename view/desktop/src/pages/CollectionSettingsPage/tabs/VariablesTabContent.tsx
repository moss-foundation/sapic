export const VariablesTabContent = () => {
  const variables = [
    { key: "baseUrl", value: "https://api.example.com" },
    { key: "apiKey", value: "{{API_KEY}}" },
    { key: "version", value: "v1" },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-(--moss-primary-text)">Variables</h3>
        <div className="space-y-3">
          {variables.map((variable, index) => (
            <div key={index} className="flex gap-4 rounded-md border border-(--moss-border-color) p-3">
              <div className="flex-1">
                <input
                  type="text"
                  value={variable.key}
                  placeholder="Variable name"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <div className="flex-1">
                <input
                  type="text"
                  value={variable.value}
                  placeholder="Variable value"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <button className="p-2 text-(--moss-secondary-text) hover:text-(--moss-danger)">×</button>
            </div>
          ))}
          <button className="w-full rounded border-2 border-dashed border-(--moss-border-color) p-3 text-(--moss-secondary-text) hover:border-(--moss-primary) hover:text-(--moss-primary-text)">
            + Add Variable
          </button>
        </div>
      </div>
    </div>
  );
};
