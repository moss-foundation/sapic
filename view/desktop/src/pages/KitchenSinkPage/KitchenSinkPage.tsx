import { PageContent } from "@/components";
import { useNotifications } from "@/app/NotificationProvider";
import { Button } from "@/lib/ui";

import { ActionMenusSection } from "./Sections/ActionMenusSection";
import { ButtonsSection } from "./Sections/ButtonsSection";
import { CommandSection } from "./Sections/CommandSection";
import { IconsSection } from "./Sections/IconsSections";
import { InputTemplatingSection } from "./Sections/InputTemplatingSection";
import { NotificationsSection } from "./Sections/NotificationsSection";
import { TableSection } from "./Sections/TableSection";
import { TabsSection } from "./Sections/TabsSection";

export const KitchenSink = () => {
  const { addNotification, clearAllNotifications } = useNotifications();

  const showInfoNotification = () => {
    addNotification({
      variant: "info",
      title: "JDK 18 required",
      description: "You need to install JDK 18 in order to run this project.",
      buttonText: "Install JDK 18",
      linkText: "Remind me later",
      onButtonClick: () => alert("Install JDK 18 clicked!"),
      onLinkClick: () => alert("Remind me later clicked!"),
      duration: 0, // Don't auto-dismiss
    });
  };

  const showWarningNotification = () => {
    addNotification({
      variant: "warning",
      title: "Low memory",
      description: "The IDE is running low on memory and this might affect performance.",
      buttonText: "Analyze memory use",
      onButtonClick: () => alert("Analyze memory use clicked!"),
      duration: 8000, // Auto-dismiss after 8 seconds
    });
  };

  const showErrorNotification = () => {
    addNotification({
      variant: "error",
      title: "Build failed",
      description: "The compilation process encountered errors. Please check your code.",
      buttonText: "View errors",
      linkText: "Ignore",
      onButtonClick: () => alert("View errors clicked!"),
      onLinkClick: () => alert("Ignore clicked!"),
      duration: 0, // Don't auto-dismiss
    });
  };

  const showSuccessNotification = () => {
    addNotification({
      variant: "info",
      icon: "GreenCheckmark",
      title: "2,662 files updated",
      description: "Successfully updated 2,662 files in 844 commits",
      linkText: "View commits",
      onLinkClick: () => alert("View commits clicked!"),
      duration: 5000, // Auto-dismiss after 5 seconds
    });
  };

  return (
    <PageContent className="mx-auto flex max-w-6xl flex-col gap-10">
      {/* Notification Test Buttons */}
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 capitalize dark:text-gray-100">
          Global Notifications Test
        </h2>
        <p className="mb-6 text-gray-600 dark:text-gray-300">
          Click these buttons to trigger notifications that appear in the bottom-right corner like IntelliJ.
        </p>

        <div className="mb-4 flex flex-wrap gap-3">
          <Button onClick={showInfoNotification} className="bg-blue-600 text-white hover:bg-blue-700">
            Show Info Notification
          </Button>
          <Button onClick={showWarningNotification} className="bg-yellow-600 text-white hover:bg-yellow-700">
            Show Warning Notification
          </Button>
          <Button onClick={showErrorNotification} className="bg-red-600 text-white hover:bg-red-700">
            Show Error Notification
          </Button>
          <Button onClick={showSuccessNotification} className="bg-green-600 text-white hover:bg-green-700">
            Show Success Notification
          </Button>
        </div>

        <Button onClick={clearAllNotifications} className="bg-gray-600 text-white hover:bg-gray-700">
          Clear All Notifications
        </Button>
      </section>

      <TabsSection />

      <TableSection />

      <ActionMenusSection />

      <ButtonsSection />

      <NotificationsSection />

      <InputTemplatingSection />

      <CommandSection />

      <IconsSection />
    </PageContent>
  );
};
