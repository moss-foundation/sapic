import { useEffect, useRef, useState } from "react";

import { CreateEntryInput } from "@repo/moss-collection";

import { TreeCollectionNode } from "./types";

interface NodeRenamingFormProps {
  onSubmit: (newEntry: CreateEntryInput) => void;
  onCancel: () => void;
  isAddingFolder: boolean;
  parentNode: TreeCollectionNode;
}

const createEntry = (parentNode: TreeCollectionNode, name: string, isAddingFolder: boolean): CreateEntryInput => {
  if (isAddingFolder) {
    return {
      dir: {
        name,
        path: parentNode.path,
        configuration: {
          request: {
            http: {},
          },
        },
      },
    };
  }

  return {
    item: {
      name,
      path: parentNode.path,
      configuration: {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      },
    },
  };
};

export const NodeAddForm = ({ onSubmit, onCancel, isAddingFolder, parentNode }: NodeRenamingFormProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const isInitialized = useRef(false);

  const [value, setValue] = useState("");

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    if (!value) return;

    const newEntry = createEntry(parentNode, value, isAddingFolder);

    onSubmit(newEntry);
  };

  const handleBlur = () => {
    if (!isInitialized.current) return;

    if (!value) {
      onCancel();
      return;
    }

    const newEntry = createEntry(parentNode, value, isAddingFolder);

    onSubmit(newEntry);
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
