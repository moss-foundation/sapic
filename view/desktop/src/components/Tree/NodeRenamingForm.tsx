import { useCallback, useEffect, useRef, useState } from "react";

import { platform } from "@tauri-apps/plugin-os";

import { useClickOutside } from "./hooks/useClickOutside";

interface NodeRenamingFormProps {
  onSubmit: (newName: string) => void;
  onCancel: () => void;
  restrictedNames?: (string | number)[];
  currentName: string | number;
}

export const NodeRenamingForm = ({ onSubmit, onCancel, restrictedNames, currentName }: NodeRenamingFormProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(String(currentName));
  const isMac = platform() === "macos";

  const finishEditing = useCallback(() => {
    const newName = value.trim();
    if (restrictedNames?.includes(newName)) {
      onCancel();
    } else {
      onSubmit(newName);
    }
  }, [value, restrictedNames, onCancel, onSubmit]);

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    finishEditing();
  };

  // We use onBlur for Windows and useClickOutside for macOS
  const handleBlur = () => finishEditing();

  useClickOutside(containerRef, () => {
    if (isMac) {
      finishEditing();
    }
  });

  useEffect(() => {
    // Delay to avoid focus bug on macOS
    const timer = setTimeout(() => {
      if (inputRef.current) {
        inputRef.current.focus();
        const dotIndex = inputRef.current.value.indexOf(".");
        inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : inputRef.current.value.length);
      }
    }, 100);
    return () => clearTimeout(timer);
  }, []);

  return (
    <form onSubmit={handleSubmit} className="w-full grow">
      <div ref={containerRef}>
        <input
          ref={inputRef}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          autoFocus
          minLength={1}
          maxLength={100}
          className="flex w-[calc(100%-8px)] min-w-0 grow items-center gap-1 rounded-xs bg-white outline outline-offset-1 outline-(--moss-primary)"
          onKeyUp={handleKeyUp}
          onBlur={isMac ? undefined : handleBlur}
          required
        />
      </div>
    </form>
  );
};
