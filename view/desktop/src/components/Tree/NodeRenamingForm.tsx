import { useEffect, useRef, useState } from "react";

interface NodeRenamingFormProps {
  onSubmit: (newName: string) => void;
  onCancel: () => void;
  restrictedNames: (string | number)[];
  currentName: string | number;
}

export const NodeRenamingForm = ({ onSubmit, onCancel, restrictedNames, currentName }: NodeRenamingFormProps) => {
  const ref = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(String(currentName));

  const handleInputKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    const newName = value.trim();

    if (currentName === newName) {
      onCancel();
      return;
    }

    if (restrictedNames.includes(String(newName))) {
      ref.current?.setCustomValidity(`The name "${newName}" is already exists in this location`);
      ref.current?.reportValidity();
      return;
    }

    onSubmit(newName);
  };

  useEffect(() => {
    if (ref.current) {
      ref.current.focus();
      ref.current.value = value;
      const dotIndex = ref.current.value.indexOf(".");
      ref.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : value.length);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <form onSubmit={handleSubmit} className="grow w-full">
      <input
        ref={ref}
        value={value}
        onChange={(e) => setValue(e.target.value)}
        autoFocus
        minLength={1}
        maxLength={100}
        className="flex gap-1 w-full min-w-0 grow items-center focus-within:outline-none relative"
        onKeyUp={handleInputKeyUp}
        onBlur={restrictedNames.includes(String(value)) ? onCancel : handleSubmit}
        required
      />
    </form>
  );
};
