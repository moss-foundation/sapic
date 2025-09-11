import { useState } from "react";

import { InputPlain } from "@/components";
import { Icon, Icons } from "@/lib/ui";

interface EditableHeaderProps {
  icon: Icons;
  title: string;
  isRenamingEntry: boolean;
  setIsRenamingEntry: (isRenamingEntry: boolean) => void;
  handleRenamingEntrySubmit: (newName: string) => void;
  handleRenamingEntryCancel: () => void;
  editable: boolean;
}

export const EditableHeader = ({
  icon,
  title: initialTitle,
  isRenamingEntry,
  setIsRenamingEntry,
  handleRenamingEntrySubmit,
  handleRenamingEntryCancel,
  editable = false,
}: EditableHeaderProps) => {
  const [newTitle, setNewTitle] = useState(initialTitle);

  const handleBlur = () => {
    handleRenamingEntrySubmit(newTitle);
    setIsRenamingEntry(false);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    handleRenamingEntrySubmit(newTitle);
    setIsRenamingEntry(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") {
      handleRenamingEntryCancel();
    }
    if (e.key === "Enter") {
      handleRenamingEntrySubmit(newTitle);
      setIsRenamingEntry(false);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <div className="rounded-md border border-(--moss-border-color) p-1">
        <Icon icon={icon} className="size-[18px]" />
      </div>
      {isRenamingEntry ? (
        <form onSubmit={handleSubmit} className="-mx-1 w-full max-w-[200px] px-1">
          <InputPlain
            autoFocus
            value={newTitle}
            onChange={(event) => setNewTitle(event.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            className="w-full rounded-md py-0 text-lg leading-6 font-bold text-(--moss-primary-text) has-[input:focus-within]:outline-offset-1"
            inputFieldClassName="-mx-2"
          />
        </form>
      ) : (
        <span
          onClick={editable ? () => setIsRenamingEntry(true) : undefined}
          className="hover:background-(--moss-secondary-background-hover) -mx-1 w-full max-w-[200px] cursor-text truncate rounded-md px-1 py-0.5 text-lg leading-6 font-bold text-(--moss-primary-text) transition-colors"
        >
          {newTitle}
        </span>
      )}
    </div>
  );
};
