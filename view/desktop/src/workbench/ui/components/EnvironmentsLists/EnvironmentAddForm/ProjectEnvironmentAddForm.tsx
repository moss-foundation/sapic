import { useRef, useState } from "react";

import { useFocusInputOnMount, useValidateInput } from "@/hooks";
import { cn } from "@/utils";

interface ProjectEnvironmentAddFormProps {
  offsetLeft: number;
  onSubmit: (name: string) => void;
  onCancel?: () => void;
  restrictedNames?: string[];
  className?: string;
}

export const ProjectEnvironmentAddForm = ({
  offsetLeft,
  onSubmit,
  onCancel,
  restrictedNames,
  className,
}: ProjectEnvironmentAddFormProps) => {
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
    inputRef.current?.blur();
  };

  return (
    <form
      onSubmit={handleSubmit}
      className={cn("flex h-full w-full grow items-center gap-1.5 py-1 pr-1", className)}
      style={{ paddingLeft: offsetLeft }}
    >
      <input
        ref={inputRef}
        value={value}
        onChange={(e) => setValue(e.target.value)}
        autoFocus
        minLength={1}
        maxLength={100}
        className="rounded-xs focus-visible:outline-(--moss-accent) relative flex h-full w-[calc(100%-3px)] min-w-0 grow items-center py-0.5 focus-visible:outline-1 focus-visible:outline-offset-0"
        onKeyUp={handleKeyUp}
        onBlur={handleBlur}
        placeholder="New Environment"
        required
      />
    </form>
  );
};
