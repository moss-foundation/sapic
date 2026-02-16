import { useRef, useState } from "react";

import { useFocusInputOnMount, useValidateInput } from "@/hooks";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

interface EnvironmentAddFormProps {
  onSubmit: (name: string) => void;
  onCancel?: () => void;
  restrictedNames?: string[];
  className?: string;
}

export const EnvironmentAddForm = ({ onSubmit, onCancel, restrictedNames, className }: EnvironmentAddFormProps) => {
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
    if (e.key === "Escape") onCancel?.();
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    if (!isValid) return;

    onSubmit(value);
    setValue("");
    inputRef.current?.blur();
  };

  const handleBlur = async () => {
    if (!isInitialized.current) return;

    if (!isValid) {
      onCancel?.();
      return;
    }

    onSubmit(value);
    setValue("");
  };

  return (
    <form
      onSubmit={handleSubmit}
      className={cn("pl-5.5 flex h-full w-full grow items-center gap-1.5 py-1 pr-1", className)}
    >
      <Icon icon="Add" className="shrink-0 opacity-50" />
      <input
        ref={inputRef}
        value={value}
        onChange={(e) => setValue(e.target.value)}
        autoFocus
        minLength={1}
        maxLength={100}
        className="rounded-xs focus-visible:outline-(--moss-accent) relative flex h-full w-[calc(100%-3px)] min-w-0 grow items-center bg-white py-0.5 focus-visible:outline-1 focus-visible:outline-offset-0"
        onKeyUp={handleKeyUp}
        onBlur={handleBlur}
        placeholder="New Environment"
        required
      />
    </form>
  );
};
