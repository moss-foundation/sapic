import { useEffect, useState } from "react";

// Responsive breakpoints (in px) similar to Tailwind
export const BREAKPOINTS = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  "2xl": 1536,
};

// Custom hook for responsive design
export const useResponsive = () => {
  const [breakpoint, setBreakpoint] = useState(() => {
    const width = window.innerWidth;
    if (width < BREAKPOINTS.sm) return "xs";
    if (width < BREAKPOINTS.md) return "sm";
    if (width < BREAKPOINTS.lg) return "md";
    if (width < BREAKPOINTS.xl) return "lg";
    if (width < BREAKPOINTS["2xl"]) return "xl";
    return "2xl";
  });

  const [screenWidth, setScreenWidth] = useState(window.innerWidth);

  useEffect(() => {
    const updateDimensions = () => {
      const width = window.innerWidth;
      setScreenWidth(width);

      if (width < BREAKPOINTS.sm) setBreakpoint("xs");
      else if (width < BREAKPOINTS.md) setBreakpoint("sm");
      else if (width < BREAKPOINTS.lg) setBreakpoint("md");
      else if (width < BREAKPOINTS.xl) setBreakpoint("lg");
      else if (width < BREAKPOINTS["2xl"]) setBreakpoint("xl");
      else setBreakpoint("2xl");
    };

    window.addEventListener("resize", updateDimensions);
    return () => window.removeEventListener("resize", updateDimensions);
  }, []);

  return {
    breakpoint,
    screenWidth,
    isSmall: breakpoint === "xs" || breakpoint === "sm",
    isMedium: breakpoint === "xs" || breakpoint === "sm" || breakpoint === "md",
    isLarge: breakpoint === "xs" || breakpoint === "sm" || breakpoint === "md" || breakpoint === "lg",
    isXLarge:
      breakpoint === "xs" || breakpoint === "sm" || breakpoint === "md" || breakpoint === "lg" || breakpoint === "xl",
    is2XLarge:
      breakpoint === "xs" ||
      breakpoint === "sm" ||
      breakpoint === "md" ||
      breakpoint === "lg" ||
      breakpoint === "xl" ||
      breakpoint === "2xl",
  };
};
