import { useState } from "react";

import { Icon, Icons } from "@/lib/ui";
import Input from "@/lib/ui/Input";

interface EditableHeaderProps {
  icon: Icons;
  title: string;
  isRenamingResourceDetails: boolean;
  setIsRenamingResourceDetails: (isRenamingResource: boolean) => void;
  handleRenamingResourceDetailsSubmit: (newName: string) => void;
  handleRenamingResourceDetailsCancel: () => void;
  editable: boolean;
}

export const EditableHeader = ({
  icon,
  title: initialTitle,
  isRenamingResourceDetails,
  setIsRenamingResourceDetails,
  handleRenamingResourceDetailsSubmit,
  handleRenamingResourceDetailsCancel,
  editable = false,
}: EditableHeaderProps) => {
  const [newTitle, setNewTitle] = useState(initialTitle);

  const handleBlur = () => {
    handleRenamingResourceDetailsSubmit(newTitle);
    setIsRenamingResourceDetails(false);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    handleRenamingResourceDetailsSubmit(newTitle);
    setIsRenamingResourceDetails(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") {
      handleRenamingResourceDetailsCancel();
    }
    if (e.key === "Enter") {
      handleRenamingResourceDetailsSubmit(newTitle);
      setIsRenamingResourceDetails(false);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <div className="border-(--moss-border) rounded-md border p-[3px]">
        <Icon icon={icon} className="size-[18px]" />
      </div>

      {isRenamingResourceDetails ? (
        <form onSubmit={handleSubmit} className="-mx-1 w-full max-w-[200px] px-1">
          <Input
            intent="plain"
            autoFocus
            value={newTitle}
            onChange={(event) => setNewTitle(event.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            className="text-(--moss-primary-foreground) w-full rounded-md border-none py-0 has-[input:focus-within]:outline-offset-1"
            inputFieldClassName="-mx-2 py-0 font-bold text-lg"
          />
        </form>
      ) : (
        <h2
          onClick={editable ? () => setIsRenamingResourceDetails(true) : undefined}
          className="hover:background-(--moss-secondary-background-hover) text-(--moss-primary-foreground) -mx-1 w-full max-w-[200px] cursor-text truncate rounded-md px-1 py-0.5 text-lg font-bold leading-6 transition-colors"
        >
          {newTitle}
        </h2>
      )}
    </div>
  );
};
