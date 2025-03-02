import { useEffect, useRef, useState } from "react";

import { NodeProps, TreeNodeProps } from "./types";

export const useNodeRename = (node: TreeNodeProps, onNodeUpdate: (node: TreeNodeProps, oldId?: string | number) => void) => {
  const [renaming, setRenaming] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleButtonKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "F2" && document.activeElement === e.currentTarget) {
      setRenaming(true);
    }
  };

  const handleInputKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") setRenaming(false)
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    const newId = inputRef.current?.value.trim();
    if (newId && newId !== node.id) {
      onNodeUpdate({ ...node, id: newId, }, node.id);
    }

    setRenaming(false);
  };

  useEffect(() => {
    if (renaming && inputRef.current) {
      setFocusOnInput(inputRef, node);
    }

  }, [renaming, node.id, inputRef, node]);

  const setFocusOnInput = (inputRef: React.RefObject<HTMLInputElement>, node: NodeProps) => {
    if (inputRef?.current) {
      inputRef.current.focus();
      inputRef.current.value = String(node.id);
      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : String(node.id).length);
    }
  };

  return {
    renaming,
    setRenaming,
    inputRef,
    handleButtonKeyUp,
    handleInputKeyUp,
    handleSubmit,
    setFocusOnInput
  };
};
