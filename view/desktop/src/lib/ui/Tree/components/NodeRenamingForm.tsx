import { useEffect, useRef, useState } from "react";

import { useClickOutside, useFocusInputOnMount, useValidateInput } from "@/hooks";
import { cn } from "@/utils";
import { platform } from "@tauri-apps/plugin-os";

interface NodeRenamingFormProps {
  onSubmit: (newName: string) => void;
  onCancel: () => void;
  restrictedNames: string[];
  currentName: string;
}

const isMac = platform() === "macos";
const isLinux = platform() === "linux";
// HACK: Adding leading-[19px] class for Linux and macOS to prevent slight shifting of list items during edit mode.
const leadingClass = isMac || isLinux ? "leading-[19px]" : "";

export const NodeRenamingForm = ({ onSubmit, onCancel, restrictedNames, currentName }: NodeRenamingFormProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const [value, setValue] = useState(String(currentName));

  const isSameValue = value.trim().toLowerCase() === currentName.trim().toLowerCase();

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

  const finishEditing = () => {
    if (isValid) onSubmit(value);
    else onCancel();
  };

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  // We use onBlur for Windows and useClickOutside for macOS
  const handleBlur = () => {
    if (!isInitialized.current) return;
    finishEditing();
  };

  useClickOutside(containerRef, () => {
    if (isMac) finishEditing();
  });

  useEffect(() => {
    // Delay to avoid focus bug on macOS
    const timer = setTimeout(() => {
      if (!inputRef.current) return;

      inputRef.current.focus();
      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : inputRef.current.value.length);

      isInitialized.current = true;
    }, 100);

    return () => clearTimeout(timer);
  }, []);

  return (
    <form action={finishEditing} className="w-full grow">
      <div ref={containerRef}>
        <input
          ref={inputRef}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          autoFocus
          minLength={1}
          maxLength={100}
          className={cn(
            `z-1 rounded-xs outline-(--moss-primary) flex w-[calc(100%-8px)] min-w-0 grow items-center gap-1 bg-white outline outline-offset-1`,
            {
              "outline-(--moss-error)": !isValid && !isSameValue,
            },
            leadingClass
          )}
          onKeyUp={handleKeyUp}
          onBlur={isMac ? undefined : handleBlur}
          required
        />
      </div>
    </form>
  );
};
