import { invokeMossCommand } from "@/lib/backend/platfrom";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const CommandSection = () => {
  return (
    <KitchenSinkSection
      header="Command Example"
      description="Demonstrates invoking a command through the platform API."
    >
      <button
        className="rounded-md bg-indigo-600 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:bg-indigo-500 dark:hover:bg-indigo-600"
        onClick={() => {
          invokeMossCommand("example.generateLog", {});
        }}
      >
        Example Command
      </button>
    </KitchenSinkSection>
  );
};
