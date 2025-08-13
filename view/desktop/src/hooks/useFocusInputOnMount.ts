import { MutableRefObject, RefObject, useEffect, useRef } from "react";

interface UseFocusOnMountProps {
  inputRef: RefObject<HTMLInputElement>;
  initialValue?: string;
}

interface UseFocusInputOnMountReturn {
  isInitialized: MutableRefObject<boolean>;
}

export function useFocusInputOnMount({ inputRef, initialValue }: UseFocusOnMountProps): UseFocusInputOnMountReturn {
  const isInitialized = useRef(false);

  useEffect(() => {
    if (!inputRef.current) return;

    // timer because of MacOS focus bug
    const timer = setTimeout(() => {
      const el = inputRef.current!;
      el.focus();

      if (typeof initialValue === "string") {
        el.value = initialValue;
        const dotIndex = el.value.indexOf(".");
        el.setSelectionRange(0, dotIndex >= 0 ? dotIndex : initialValue.length);
      }

      isInitialized.current = true;
    }, 100);

    return () => clearTimeout(timer);
    // intentionally run only once on mount
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { isInitialized };
}
