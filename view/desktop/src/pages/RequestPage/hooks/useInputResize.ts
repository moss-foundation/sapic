import { useEffect } from "react";

interface UseInputResizeProps {
  ref: React.RefObject<HTMLInputElement>;
  enabled?: boolean;
}

const useInputResize = ({ ref, enabled = true }: UseInputResizeProps) => {
  useEffect(() => {
    const input = ref.current;
    if (!input || input.nodeName.toLowerCase() !== "input" || !enabled) return;

    const resize = () => {
      const computedStyle = getComputedStyle(input);
      let offset = 0;

      // Check if input is visible
      if (input.getBoundingClientRect().width) {
        // Reset width to calculate scrollWidth accurately
        input.style.width = "0";

        // Calculate offset based on box-sizing
        switch (computedStyle.boxSizing) {
          case "padding-box":
            offset = input.clientWidth;
            break;
          case "content-box":
            offset = parseFloat(computedStyle.minWidth) || 0;
            break;
          case "border-box":
            offset = input.offsetWidth;
            break;
          default:
            break;
        }

        // Calculate width, accounting for scrollWidth
        let width = Math.max(offset, input.scrollWidth - input.clientWidth);
        input.style.width = `${width}px`;

        // Handle scrollWidth polyfill for browsers like IE11
        for (let i = 0; i < 10; i++) {
          input.scrollLeft = 1000000;
          if (input.scrollLeft === 0) break;
          width += input.scrollLeft;
          input.style.width = `${width}px`;
        }
      } else {
        // Fallback for non-visible inputs
        input.style.width = `${input.value.length}ch`;
      }
    };

    resize();

    input.addEventListener("input", resize);
    return () => {
      input.removeEventListener("input", resize);
    };
  }, [enabled, ref]);
};

export default useInputResize;
