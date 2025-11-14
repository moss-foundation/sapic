import { useState } from "react";

import { InputTemplating } from "@/workbench/ui/components";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const InputTemplatingSection = () => {
  const [value, setValue] = useState("Hello {{name}}, your order {{orderId}} is ready!");
  const [variables, setVariables] = useState<string[]>([]);

  const handleTemplateChange = (newValue: string, extractedVariables: string[]) => {
    setValue(newValue);
    setVariables(extractedVariables);
  };

  return (
    <KitchenSinkSection
      header="Input Templating"
      description="Input component with template variable highlighting using double curly braces syntax."
    >
      <div className="space-y-4">
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            Template Input (try typing {"{{"} variable {"}}"} syntax):
          </label>
          <InputTemplating
            value={value}
            onChange={(e) => setValue(e.target.value)}
            onTemplateChange={handleTemplateChange}
            placeholder="Type something with {{variables}} here..."
            className="w-full"
            size="md"
          />
        </div>

        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Size Variants:</label>
          <div className="space-y-2">
            <InputTemplating placeholder="Small (sm) - default" className="w-full" size="sm" />
            <InputTemplating placeholder="Medium (md)" className="w-full" size="md" />
          </div>
        </div>

        {variables.length > 0 && (
          <div className="mt-4 rounded-md bg-gray-50 p-3 dark:bg-gray-800">
            <h4 className="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Detected Variables:</h4>
            <div className="flex flex-wrap gap-2">
              {variables.map((variable, index) => (
                <span
                  key={index}
                  className="rounded bg-blue-100 px-2 py-1 text-sm text-blue-800 dark:bg-blue-900 dark:text-blue-200"
                >
                  {variable}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    </KitchenSinkSection>
  );
};
