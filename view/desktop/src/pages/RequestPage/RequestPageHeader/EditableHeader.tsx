import { useState } from "react";

import { InputPlain } from "@/components";

interface EditableHeaderProps {
  title: string;
  isRenamingEntry: boolean;
  setIsRenamingEntry: (isRenamingEntry: boolean) => void;
  handleRenamingEntrySubmit: (newName: string) => void;
  handleRenamingEntryCancel: () => void;
  editable: boolean;
}

export const EditableHeader = ({
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
    <div className="flex items-center gap-2" onClick={editable ? () => setIsRenamingEntry(true) : undefined}>
      <div className="rounded-md border border-(--moss-border-color) p-1">
        <HttpSvg />
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
        <span className="hover:background-(--moss-secondary-background-hover) -mx-1 w-full max-w-[200px] cursor-text truncate rounded-md px-1 py-0.5 text-lg leading-6 font-bold text-(--moss-primary-text)">
          {newTitle}
        </span>
      )}
    </div>
  );
};

const HttpSvg = () => {
  return (
    <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="8.99844" cy="9.00039" r="7.45" fill="#EDF3FF" stroke="#3574F0" strokeWidth="1.3" />
      <path
        d="M9 1.55078C9.89939 1.55078 10.8508 2.21462 11.6143 3.58887C12.3653 4.94098 12.8496 6.85286 12.8496 9C12.8496 11.1474 12.3654 13.0599 11.6143 14.4121C10.8508 15.7864 9.89939 16.4502 9 16.4502C8.10061 16.4502 7.14921 15.7864 6.38574 14.4121C5.63461 13.0599 5.15039 11.1474 5.15039 9C5.15045 6.85286 5.63471 4.94098 6.38574 3.58887C7.14921 2.21462 8.10061 1.55078 9 1.55078Z"
        stroke="#3574F0"
        strokeWidth="1.3"
      />
      <path d="M15.2953 5.40039L2.69531 5.40039" stroke="#3574F0" strokeWidth="1.3" />
      <path d="M15.2953 12.5996L2.69531 12.5996" stroke="#3574F0" strokeWidth="1.3" />
      <path
        d="M16.463 5.84961C16.8721 6.81792 17.0984 7.88232 17.0984 8.99961C17.0984 10.1169 16.8721 11.1813 16.463 12.1496H1.53389C1.12476 11.1813 0.898438 10.1169 0.898438 8.99961C0.898438 7.88232 1.12476 6.81792 1.53389 5.84961H16.463Z"
        fill="white"
      />
      <path
        d="M0.898438 11.2503V6.75H1.95366L1.96659 8.55H3.14844L3.13551 6.75H4.19074V11.2503H3.13551L3.14844 9.54863H1.96659L1.95366 11.2503H0.898438Z"
        fill="#208A3C"
      />
      <path d="M6.23512 11.2503V7.74863H4.93368V6.75H8.59179V7.74863H7.29035V11.2503H6.23512Z" fill="#208A3C" />
      <path d="M10.4533 11.2503V7.74863H9.15183V6.75H12.8099V7.74863H11.5085V11.2503H10.4533Z" fill="#208A3C" />
      <path
        d="M13.5529 11.2503V6.75H15.3046C15.6657 6.75 15.9799 6.82151 16.2472 6.96452C16.5192 7.10753 16.7279 7.30726 16.8733 7.5637C17.0234 7.82014 17.0984 8.12096 17.0984 8.46616C17.0984 8.80644 17.0234 9.10726 16.8733 9.36863C16.7232 9.62507 16.5145 9.82479 16.2472 9.96781C15.9799 10.1108 15.6657 10.1823 15.3046 10.1823H14.6081V11.2503H13.5529ZM14.6081 9.22068H15.3046C15.5297 9.22068 15.7079 9.14918 15.8392 9.00616C15.9752 8.86315 16.0432 8.68315 16.0432 8.46616C16.0432 8.24918 15.9752 8.06918 15.8392 7.92616C15.7079 7.78315 15.5297 7.71164 15.3046 7.71164H14.6081V9.22068Z"
        fill="#208A3C"
      />
    </svg>
  );
};
