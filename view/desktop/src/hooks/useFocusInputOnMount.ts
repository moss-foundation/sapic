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

      if (initialValue) {
        el.value = initialValue;
      }

      el.focus();
      el.select();
      isInitialized.current = true;
    }, 100);

    return () => clearTimeout(timer);
    // intentionally run only once on mount
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { isInitialized };
}
