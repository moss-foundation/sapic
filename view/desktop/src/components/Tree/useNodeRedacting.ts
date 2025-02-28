import { useEffect, useRef, useState } from "react";

import { NodeProps } from "./types";

export const useNodeRedacting = (node: NodeProps, onNodeUpdate: (node: NodeProps, oldId?: string | number) => void) => {
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

    const newId = inputRef.current?.value.trim();
    if (newId && newId !== node.id) {
      onNodeUpdate({ ...node, id: newId, }, node.id);
    }
    setRedacting(false);
  };

  useEffect(() => {
    if (redacting && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.value = String(node.id);
      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : String(node.id).length);
    }
  }, [redacting, node.id]);

  return {
    redacting,
    setRedacting,
    inputRef,
    handleButtonKeyUp,
    handleInputKeyUp,
    handleSubmit,
  };
};
