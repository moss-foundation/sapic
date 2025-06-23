import React, { useState } from "react";

interface Variable {
  key: string;
  value: string;
}

export const VariablesTabContent = () => {
  const [variables, setVariables] = useState<Variable[]>([
    { key: "baseUrl", value: "https://api.example.com" },
    { key: "apiKey", value: "{{API_KEY}}" },
    { key: "version", value: "v1" },
  ]);

  const handleVariableChange = (index: number, field: "key" | "value", newValue: string) => {
    setVariables((prev) => prev.map((variable, i) => (i === index ? { ...variable, [field]: newValue } : variable)));
  };

  const handleAddVariable = () => {
    setVariables((prev) => [...prev, { key: "", value: "" }]);
  };

  const handleRemoveVariable = (index: number) => {
    setVariables((prev) => prev.filter((_, i) => i !== index));
  };

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
                  onChange={(e) => handleVariableChange(index, "key", e.target.value)}
                  placeholder="Variable name"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <div className="flex-1">
                <input
                  type="text"
                  value={variable.value}
                  onChange={(e) => handleVariableChange(index, "value", e.target.value)}
                  placeholder="Variable value"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <button
                onClick={() => handleRemoveVariable(index)}
                className="p-2 text-(--moss-secondary-text) hover:text-(--moss-error)"
              >
                ×
              </button>
            </div>
          ))}
          <button
            onClick={handleAddVariable}
            className="w-full rounded border-2 border-dashed border-(--moss-border-color) p-3 text-(--moss-secondary-text) hover:border-(--moss-primary) hover:text-(--moss-primary-text)"
          >
            + Add Variable
          </button>
        </div>
      </div>
    </div>
  );
};
