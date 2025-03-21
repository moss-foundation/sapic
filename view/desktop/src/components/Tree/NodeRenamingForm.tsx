import { useEffect, useRef, useState } from "react";

import { platform } from "@tauri-apps/plugin-os";

interface NodeRenamingFormProps {
  onSubmit: (newName: string) => void;
  onCancel: () => void;
  restrictedNames?: (string | number)[];
  currentName: string | number;
}

export const NodeRenamingForm = ({ onSubmit, onCancel, restrictedNames, currentName }: NodeRenamingFormProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(String(currentName));

  const isMac = platform() === "macos";

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    const newName = value.trim();

    if (currentName === newName) {
      onCancel();
      return;
    }

    if (restrictedNames?.includes(newName)) {
      inputRef.current?.setCustomValidity(`The name "${newName}" is already exists in this location`);
      inputRef.current?.reportValidity();
      return;
    }

    onSubmit(newName);
  };

  const handleBlur = () => {
    const newName = value.trim();

    if (restrictedNames?.includes(newName)) {
      onCancel();
    } else {
      onSubmit(newName);
    }
  };

  useEffect(() => {
    // Timer is set because of MacOS focus bug
    const timer = setTimeout(() => {
      if (inputRef.current) {
        inputRef.current.focus();
        inputRef.current.value = value;
        const dotIndex = inputRef.current.value.indexOf(".");
        inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : value.length);
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
        className="relative flex w-full min-w-0 grow items-center gap-1 bg-transparent focus-within:outline-none"
        onKeyUp={handleKeyUp}
        onBlur={isMac ? undefined : handleBlur}
        required
      />
    </form>
  );
};
