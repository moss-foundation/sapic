import { useRef, useState } from "react";

import { useFocusInputOnMount, useValidateInput } from "@/hooks";
import { cn } from "@/utils";

interface NodeRenamingFormProps {
  onSubmit: (name: string) => void;
  onCancel: () => void;
  restrictedNames?: string[];
  className?: string;
}

export const NodeAddForm = ({ onSubmit, onCancel, restrictedNames, className }: NodeRenamingFormProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const [value, setValue] = useState("");

  const { isInitialized } = useFocusInputOnMount({
    inputRef,
    initialValue: value,
  });

  const { isValid } = useValidateInput({
    value,
    restrictedValues: restrictedNames,
    inputRef,
    isInitialized,
  });

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    if (!isValid) return;

    onSubmit(value);
  };

  const handleBlur = () => {
    if (!isInitialized.current) return;

    if (!isValid) {
      onCancel();
      return;
    }

    onSubmit(value);
  };

  return (
    <form onSubmit={handleSubmit} className={cn("h-full w-full grow py-1 pr-1", className)}>
      <input
        ref={inputRef}
        value={value}
        onChange={(e) => setValue(e.target.value)}
        autoFocus
        minLength={1}
        maxLength={100}
        className="relative flex h-full w-[calc(100%-3px)] min-w-0 grow items-center rounded-xs bg-white py-0.5 focus-visible:outline-1 focus-visible:outline-offset-0 focus-visible:outline-(--moss-accent)"
        onKeyUp={handleKeyUp}
        onBlur={handleBlur}
        required
      />
    </form>
  );
};
