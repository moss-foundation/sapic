import { useEffect, useRef, useState } from "react";

import { platform } from "@tauri-apps/plugin-os";

import { useClickOutside } from "./hooks/useClickOutside";
import { validateName } from "./utils/FormUtils";

interface NodeRenamingFormProps {
  onSubmit: (newName: string) => void;
  onCancel: () => void;
  restrictedNames?: (string | number)[];
  currentName: string | number;
}

export const NodeRenamingForm = ({ onSubmit, onCancel, restrictedNames, currentName }: NodeRenamingFormProps) => {
  const isMac = platform() === "macos";
  const isLinux = platform() === "linux";
  // HACK: Adding leading-[19px] class for Linux and macOS to prevent slight shifting of list items during edit mode.
  const leadingClass = isMac || isLinux ? "leading-[19px]" : "";

  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const isInitialized = useRef(false);
  const [value, setValue] = useState(String(currentName));

  const { isValid, message } = validateName(value, restrictedNames ?? []);

  useEffect(() => {
    if (!inputRef.current || !isInitialized.current) return;

    inputRef.current.setCustomValidity(message);
    inputRef.current.reportValidity();
  }, [message]);

  const finishEditing = () => {
    if (!isValid) {
      onCancel();
      return;
    }

    onSubmit(value);
  };

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
        isInitialized.current = true;
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
          className={`flex w-[calc(100%-8px)] min-w-0 grow items-center gap-1 rounded-xs bg-white outline outline-offset-1 outline-(--moss-primary) ${leadingClass}`}
          onKeyUp={handleKeyUp}
          onBlur={isMac ? undefined : handleBlur}
          required
        />
      </div>
    </form>
  );
};
