import { useEffect, useRef, useState } from "react";

import { NodeProps } from "./types";

export const useNodeRedacting = (node: NodeProps, onNodeUpdate: (node: NodeProps) => void) => {
  const [redacting, setRedacting] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleButtonKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "F2" && document.activeElement === e.currentTarget) {
      setRedacting(true);
    }
  };

  const handleInputKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") {
      setRedacting(false);
    }
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    const newName = inputRef.current?.value.trim();
    if (newName && newName !== node.name) {
      onNodeUpdate({ ...node, name: newName });
    }
    setRedacting(false);
  };

  useEffect(() => {
    if (redacting && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.value = node.name;
      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : node.name.length);
    }
  }, [redacting, node.name]);

  return {
    redacting,
    setRedacting,
    inputRef,
    handleButtonKeyUp,
    handleInputKeyUp,
    handleSubmit,
  };
};
