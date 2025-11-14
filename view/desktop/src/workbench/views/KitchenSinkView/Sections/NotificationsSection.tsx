import { toast } from "sonner";

import { Button, Icon, createNotificationContent } from "@/lib/ui";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const NotificationsSection = () => {
  const showSimpleToast = () => {
    toast("This is a simple toast message");
  };

  const showInfoNotification = () => {
    const toastId = toast(
      createNotificationContent({
        title: "JDK 18 required",
        description: "You need to install JDK 18 in order to run this project.",
        icon: "Info",
        buttonText: "Install JDK 18",
        onButtonClick: () => {
          alert("Install JDK 18 clicked!");
          toast.dismiss(toastId);
        },
        linkText: "Remind me later",
        onLinkClick: () => {
          alert("Remind me later clicked!");
          toast.dismiss(toastId);
        },
        onClose: () => toast.dismiss(toastId),
      }),
      { duration: Infinity }
    );
  };

  const showWarningToast = () => {
    const toastId = toast(
      createNotificationContent({
        title: "Low memory",
        description: "The IDE is running low on memory and this might affect performance.",
        icon: "Warning",
        onClose: () => toast.dismiss(toastId),
      }),
      { duration: 5000 }
    );
  };

  const showErrorNotification = () => {
    const toastId = toast(
      createNotificationContent({
        title: "Build failed",
        description: "The compilation process encountered errors.",
        icon: "Failed",
        buttonText: "View errors",
        onButtonClick: () => {
          alert("View errors clicked!");
          toast.dismiss(toastId);
        },
        linkText: "Ignore",
        onLinkClick: () => {
          alert("Ignore clicked!");
          toast.dismiss(toastId);
        },
        onClose: () => toast.dismiss(toastId),
      }),
      { duration: Infinity }
    );
  };

  return (
    <KitchenSinkSection
      header="Sonner Toast Notifications"
      description="Demo buttons to trigger Sonner toast notifications with our custom design."
    >
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <button
          onClick={showSimpleToast}
          className="rounded-lg border border-gray-300 bg-gray-50 p-4 text-left transition-colors hover:bg-gray-100 dark:border-gray-600 dark:bg-gray-700 dark:hover:bg-gray-600"
        >
          <div className="font-medium">Simple Toast</div>
          <div className="text-sm text-gray-500 dark:text-gray-400">Basic message</div>
        </button>

        <button
          onClick={showInfoNotification}
          className="rounded-lg border border-blue-300 bg-blue-50 p-4 text-left transition-colors hover:bg-blue-100 dark:border-blue-600 dark:bg-blue-900 dark:hover:bg-blue-800"
        >
          <div className="font-medium text-blue-800 dark:text-blue-200">Info Notification</div>
          <div className="text-sm text-blue-600 dark:text-blue-300">Persistent with actions</div>
        </button>

        <button
          onClick={showWarningToast}
          className="rounded-lg border border-yellow-300 bg-yellow-50 p-4 text-left transition-colors hover:bg-yellow-100 dark:border-yellow-600 dark:bg-yellow-900 dark:hover:bg-yellow-800"
        >
          <div className="font-medium text-yellow-800 dark:text-yellow-200">Warning Toast</div>
          <div className="text-sm text-yellow-600 dark:text-yellow-300">Auto-dismiss 5s</div>
        </button>

        <button
          onClick={showErrorNotification}
          className="rounded-lg border border-red-300 bg-red-50 p-4 text-left transition-colors hover:bg-red-100 dark:border-red-600 dark:bg-red-900 dark:hover:bg-red-800"
        >
          <div className="font-medium text-red-800 dark:text-red-200">Error Notification</div>
          <div className="text-sm text-red-600 dark:text-red-300">Persistent with actions</div>
        </button>
      </div>

      <div className="mt-6 flex justify-center">
        <Button
          onClick={() => toast.dismiss()}
          className="flex items-center gap-2 rounded-lg border border-gray-400 bg-gray-100 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-200 dark:border-gray-500 dark:bg-gray-600 dark:text-gray-300 dark:hover:bg-gray-500"
        >
          <Icon icon="Delete" className="h-4 w-4" />
          Clear All Notifications
        </Button>
      </div>
    </KitchenSinkSection>
  );
};
