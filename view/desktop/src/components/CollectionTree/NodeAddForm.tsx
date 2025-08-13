import { useRef, useState } from "react";

import { useFocusInputOnMount, useValidateInput } from "@/hooks";

interface NodeRenamingFormProps {
  onSubmit: (name: string) => void;
  onCancel: () => void;
  restrictedNames?: string[];
}

export const NodeAddForm = ({ onSubmit, onCancel, restrictedNames }: NodeRenamingFormProps) => {
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
    <form onSubmit={handleSubmit} className="w-full grow">
      <input
        ref={inputRef}
        value={value}
        onChange={(e) => setValue(e.target.value)}
        autoFocus
        minLength={1}
        maxLength={100}
        className="relative flex w-[calc(100%-1px)] min-w-0 grow items-center gap-1 rounded-xs bg-white outline outline-(--moss-primary)"
        onKeyUp={handleKeyUp}
        onBlur={handleBlur}
        required
      />
    </form>
  );
};
