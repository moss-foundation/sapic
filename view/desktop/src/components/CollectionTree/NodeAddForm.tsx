import { useEffect, useRef, useState } from "react";

import { validateName } from "./utils/Form";

interface NodeRenamingFormProps {
  onSubmit: (name: string) => void;
  onCancel: () => void;
  restrictedNames?: (string | number)[];
}

export const NodeAddForm = ({ onSubmit, onCancel, restrictedNames }: NodeRenamingFormProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const isInitialized = useRef(false);

  const [value, setValue] = useState("");

  const { isValid, message } = validateName(value, restrictedNames ?? []);

  useEffect(() => {
    if (!inputRef.current || !isInitialized.current) return;

    inputRef.current.setCustomValidity(message);
    inputRef.current.reportValidity();
  }, [message]);

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

  useEffect(() => {
    if (!inputRef.current) return;

    // Timer is set because of MacOS focus bug
    const timer = setTimeout(() => {
      if (inputRef.current) {
        inputRef.current.focus();
        inputRef.current.value = value;
        const dotIndex = inputRef.current.value.indexOf(".");
        inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : value.length);
        isInitialized.current = true;
      }
    }, 100);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
