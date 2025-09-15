import { ReactNode } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <LanguageProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </LanguageProvider>
    </ErrorBoundary>
  );
};

export default Providers;
