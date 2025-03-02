import { useEffect, useRef, useState } from "react";

import { NodeProps, TreeNodeProps } from "./types";

export const useNodeRedacting = (node: TreeNodeProps, onNodeUpdate: (node: TreeNodeProps, oldId?: string | number) => void) => {
  const [redacting, setRedacting] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleButtonKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "F2" && document.activeElement === e.currentTarget) {
      setRedacting(true);
    }
  };

  const handleInputKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") setRedacting(false)
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    console.log("handleSubmit")
    if ("preventDefault" in e) e.preventDefault();

    const newId = inputRef.current?.value.trim();
    if (newId && newId !== node.id) {
      onNodeUpdate({ ...node, id: newId, }, node.id);
    }

    setRedacting(false);
  };


  useEffect(() => {
    if (redacting && inputRef.current) {
      setFocusOnInput(inputRef, node);
    }

  }, [redacting, node.id, inputRef, node]);



  const setFocusOnInput = (inputRef: React.RefObject<HTMLInputElement>, node: NodeProps) => {
    if (inputRef?.current) {
      inputRef.current.focus();
      inputRef.current.value = String(node.id);
      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex >= 0 ? dotIndex : String(node.id).length);
    }
  };

  return {
    redacting,
    setRedacting,
    inputRef,
    handleButtonKeyUp,
    handleInputKeyUp,
    handleSubmit,
    setFocusOnInput
  };
};
