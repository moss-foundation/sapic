import { MutableRefObject, RefObject, useEffect } from "react";

import { validateValue } from "@/utils/validateValue";

interface UseValidateInputProps {
  value: string;
  restrictedValues?: string[];
  inputRef?: RefObject<HTMLInputElement | null>;
  isInitialized?: MutableRefObject<boolean> | null;
}

export function useValidateInput({ value, restrictedValues, inputRef, isInitialized = null }: UseValidateInputProps) {
  const { isValid, message } = validateValue(value, restrictedValues ?? []);

  useEffect(() => {
    console.log("useEffect", {
      inputRef: inputRef?.current,
      isInitialized: isInitialized?.current,
    });
    if (!inputRef?.current) return;
    if (isInitialized !== null && isInitialized.current === false) return;

    console.log("reportValidity", message);
    inputRef.current.setCustomValidity(message);
    inputRef.current.reportValidity();
  }, [message, inputRef, isInitialized]);

  return { isValid, message };
}
