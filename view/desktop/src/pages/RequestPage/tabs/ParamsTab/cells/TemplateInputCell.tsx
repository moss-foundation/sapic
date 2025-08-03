import { useState, useRef, useEffect } from "react";
import { ParameterData } from "@/components/Table";
import { InputTemplating } from "@/components/InputTemplating";
import { ExtendedCellContext } from "../types";

export const TemplateInputCell = ({ info }: { info: ExtendedCellContext<ParameterData, string> }) => {
  const [value, setValue] = useState(info.getValue());
  const isDisabled = info.row.original.properties.disabled;
  const containerRef = useRef<HTMLDivElement>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };

  const handleTemplateChange = (newValue: string) => {
    setValue(newValue);
    info.table.options.meta?.updateData(info.row.index, info.column.id, newValue);
  };

  const handleBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  // Handle focus when component should be focused
  useEffect(() => {
    if (info.focusOnMount && containerRef.current) {
      // Find the contentEditable div inside InputTemplating and focus it
      const editableDiv = containerRef.current.querySelector('[contenteditable="true"]') as HTMLElement;
      if (editableDiv) {
        editableDiv.focus();
        // Move cursor to end of text
        const range = document.createRange();
        const selection = window.getSelection();
        if (editableDiv.textContent) {
          range.setStart(editableDiv.childNodes[0] || editableDiv, editableDiv.textContent.length);
          range.collapse(true);
          selection?.removeAllRanges();
          selection?.addRange(range);
        }
      }
    }
  }, [info.focusOnMount]);

  // Check if value contains template variables
  const hasTemplateVariables = /\{\{[^}]+\}\}/.test(value);

  return (
    <div ref={containerRef} className="w-full">
      <InputTemplating
        value={value}
        onChange={handleChange}
        onTemplateChange={handleTemplateChange}
        onBlur={handleBlur}
        placeholder={info.column.id}
        size="sm"
        className={`w-full rounded-none border-none focus-within:!outline-1 focus-within:!outline-offset-0 focus-within:!outline-(--moss-primary) ${
          isDisabled
            ? "text-(--moss-requestpage-text-disabled)"
            : hasTemplateVariables
              ? "" // Let InputTemplating handle template variable colors
              : "text-(--moss-primary-text)"
        }`}
      />
    </div>
  );
};
