import React, { useState } from "react";

interface Header {
  key: string;
  value: string;
}

export const HeadersTabContent = () => {
  const [headers, setHeaders] = useState<Header[]>([
    { key: "Content-Type", value: "application/json" },
    { key: "Accept", value: "application/json" },
    { key: "User-Agent", value: "Sapic/1.0.0" },
  ]);

  const handleHeaderChange = (index: number, field: "key" | "value", newValue: string) => {
    setHeaders((prev) => prev.map((header, i) => (i === index ? { ...header, [field]: newValue } : header)));
  };

  const handleAddHeader = () => {
    setHeaders((prev) => [...prev, { key: "", value: "" }]);
  };

  const handleRemoveHeader = (index: number) => {
    setHeaders((prev) => prev.filter((_, i) => i !== index));
  };

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
                  onChange={(e) => handleHeaderChange(index, "key", e.target.value)}
                  placeholder="Header name"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <div className="flex-1">
                <input
                  type="text"
                  value={header.value}
                  onChange={(e) => handleHeaderChange(index, "value", e.target.value)}
                  placeholder="Header value"
                  className="background-(--moss-primary-background) w-full rounded border border-(--moss-border-color) p-2 text-(--moss-primary-text)"
                />
              </div>
              <button
                onClick={() => handleRemoveHeader(index)}
                className="p-2 text-(--moss-secondary-text) hover:text-(--moss-error)"
              >
                ×
              </button>
            </div>
          ))}
          <button
            onClick={handleAddHeader}
            className="w-full rounded border-2 border-dashed border-(--moss-border-color) p-3 text-(--moss-secondary-text) hover:border-(--moss-primary) hover:text-(--moss-primary-text)"
          >
            + Add Header
          </button>
        </div>
      </div>
    </div>
  );
};
