import { LanguageSection } from "./sections/LanguageSection";
import { ThemeSection } from "./sections/ThemeSection";
import { WorkspaceLayoutSection } from "./sections/WokspaceLayoutSection";

export const Settings = () => {
  return (
    <div className="flex flex-col gap-4">
      <LanguageSection />
      <ThemeSection />
      <WorkspaceLayoutSection />
    </div>
  );
};
