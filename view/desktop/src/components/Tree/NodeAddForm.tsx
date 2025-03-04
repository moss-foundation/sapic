import { useEffect, useRef, useState } from "react";

import { NodeProps } from "./types";

interface NodeRenamingFormProps {
  onSubmit: (newNode: NodeProps) => void;
  onCancel: () => void;
  restrictedNames: (string | number)[];
  isFolder: boolean;
}

const createSubtree = (path: string, isFolder: boolean): NodeProps => {
  const lastNodeIsFolder = path.endsWith("/") || isFolder;

  if (path.endsWith("/")) {
    path = path.slice(0, -1);
  }

  const parts = path.split("/").filter((part) => part !== "");

  if (parts.length === 0) throw new Error("Invalid path");

  return buildNode(parts, lastNodeIsFolder);
};

const buildNode = (parts: string[], isLastFolder: boolean): NodeProps => {
  const name = parts[0];
  const isFolder = parts.length > 1 || isLastFolder;
  const childNodes = parts.length > 1 ? [buildNode(parts.slice(1), isLastFolder)] : [];

  return {
    id: name,
    type: isFolder ? "folder" : "file",
    order: 0,
    isFolder,
    isExpanded: isFolder,
    childNodes: childNodes,
  };
};

const validateName = (
  name: string,
  restrictedNames: (string | number)[]
): {
  isValid: boolean;
  message: string;
} => {
  const names = name.split("/");
  if (restrictedNames.includes(names[0])) {
    return {
      isValid: false,
      message: `The name "${names[0]}" is already exists in this location`,
    };
  }

  return {
    isValid: true,
    message: "",
  };
};

export const NodeAddForm = ({ onSubmit, onCancel, restrictedNames, isFolder }: NodeRenamingFormProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState("");

  const { message, isValid } = validateName(value, restrictedNames);

  inputRef.current?.setCustomValidity(message);
  inputRef.current?.reportValidity();

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };

  const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") onCancel();
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    if (!isValid) return;

    const node = createSubtree(value, isFolder);

    console.log(node);

    onSubmit(node);
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
    <form onSubmit={handleSubmit} className="grow w-full">
      <input
        ref={inputRef}
        value={value}
        onChange={handleChange}
        autoFocus
        minLength={1}
        maxLength={100}
        className="flex gap-1 w-full min-w-0 grow items-center focus-within:outline-none relative bg-transparent"
        onKeyUp={handleKeyUp}
        onBlur={handleSubmit}
        required
      />
    </form>
  );
};
