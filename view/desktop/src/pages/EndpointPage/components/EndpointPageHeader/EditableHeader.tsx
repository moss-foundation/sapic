import { useState } from "react";

import { Icon, Icons } from "@/lib/ui";
import Input from "@/lib/ui/Input";

interface EditableHeaderProps {
  icon: Icons;
  title: string;
  isRenamingResource: boolean;
  setIsRenamingResource: (isRenamingResource: boolean) => void;
  handleRenamingResourceSubmit: (newName: string) => void;
  handleRenamingResourceCancel: () => void;
  editable: boolean;
}

export const EditableHeader = ({
  icon,
  title: initialTitle,
  isRenamingResource,
  setIsRenamingResource,
  handleRenamingResourceSubmit,
  handleRenamingResourceCancel,
  editable = false,
}: EditableHeaderProps) => {
  const [newTitle, setNewTitle] = useState(initialTitle);

  const handleBlur = () => {
    handleRenamingResourceSubmit(newTitle);
    setIsRenamingResource(false);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    handleRenamingResourceSubmit(newTitle);
    setIsRenamingResource(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") {
      handleRenamingResourceCancel();
    }
    if (e.key === "Enter") {
      handleRenamingResourceSubmit(newTitle);
      setIsRenamingResource(false);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <div className="border-(--moss-border) rounded-md border p-[3px]">
        <Icon icon={icon} className="size-[18px]" />
      </div>

      {isRenamingResource ? (
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
          onClick={editable ? () => setIsRenamingResource(true) : undefined}
          className="hover:background-(--moss-secondary-background-hover) text-(--moss-primary-foreground) -mx-1 w-full max-w-[200px] cursor-text truncate rounded-md px-1 py-0.5 text-lg font-bold leading-6 transition-colors"
        >
          {newTitle}
        </h2>
      )}
    </div>
  );
};
