import { useInitLayout } from "@/hooks/useInitLayout";
import { PageLoader } from "@/workbench/ui/components";

interface MainStateProps {
  children: React.ReactNode;
}

export const MainState = ({ children }: MainStateProps) => {
  const { isInit } = useInitLayout();

  if (!isInit) {
    return <PageLoader className="bg-green-200" />;
  }

  return <>{children}</>;
};
