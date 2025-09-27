import { useNotifications } from "../../../app/NotificationProvider";
import { Button } from "@/lib/ui";

export const GlobalNotificationsTestSection = () => {
  const { addNotification, clearAllNotifications, removeNotification } = useNotifications();

  const showInfoNotification = () => {
    const notificationId = addNotification({
      title: "JDK 18 required",
      description: "You need to install JDK 18 in order to run this project.",
      buttonText: "Install JDK 18",
      linkText: "Remind me later",
      onButtonClick: () => {
        alert("Install JDK 18 clicked!");
        removeNotification(notificationId);
      },
      onLinkClick: () => {
        alert("Remind me later clicked!");
        removeNotification(notificationId);
      },
      duration: 0, // Don't auto-dismiss
    });
  };

  const showWarningToastDelayed = () => {
    const notificationId = addNotification({
      icon: "Warning",
      title: "Low memory",
      description: "The IDE is running low on memory and this might affect performance.",
      buttonText: "Analyze memory use",
      onButtonClick: () => {
        alert("Analyze memory use clicked!");
        removeNotification(notificationId);
      },
      duration: 5000, // Auto-dismiss after 5 seconds
    });
  };

  const showErrorNotification = () => {
    const notificationId = addNotification({
      icon: "Error",
      title: "Build failed",
      description: "The compilation process encountered errors. Please check your code.",
      buttonText: "View errors",
      linkText: "Ignore",
      onButtonClick: () => {
        alert("View errors clicked!");
        removeNotification(notificationId);
      },
      onLinkClick: () => {
        alert("Ignore clicked!");
        removeNotification(notificationId);
      },
      duration: 0, // Don't auto-dismiss
    });
  };

  const showSuccessToast = () => {
    const notificationId = addNotification({
      icon: "GreenCheckmark",
      title: "2,662 files updated",
      description: "Successfully updated 2,662 files in 844 commits",
      linkText: "View commits",
      onLinkClick: () => {
        alert("View commits clicked!");
        removeNotification(notificationId);
      },
    });
  };

  return (
    <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
      <h2 className="mb-4 text-2xl font-bold text-gray-800 capitalize dark:text-gray-100">Global Notifications Test</h2>
      <p className="mb-6 text-gray-600 dark:text-gray-300">
        Click these buttons to trigger notifications that appear in the bottom-right corner.
      </p>

      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Button
          onClick={showInfoNotification}
          className="group relative h-12 overflow-hidden bg-gradient-to-r from-blue-500 to-blue-600 px-6 py-3 text-white shadow-lg transition-all duration-200 hover:scale-105 hover:from-blue-600 hover:to-blue-700 hover:shadow-xl active:scale-95"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-blue-400 to-blue-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
          <span className="relative flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            Show Info Notification
          </span>
        </Button>

        <Button
          onClick={showWarningToastDelayed}
          className="group relative h-12 overflow-hidden bg-gradient-to-r from-yellow-500 to-yellow-600 px-6 py-3 text-white shadow-lg transition-all duration-200 hover:scale-105 hover:from-yellow-600 hover:to-yellow-700 hover:shadow-xl active:scale-95"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-yellow-400 to-yellow-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
          <span className="relative flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
              />
            </svg>
            Show Warning Toast 5 seconds
          </span>
        </Button>

        <Button
          onClick={showErrorNotification}
          className="group relative h-12 overflow-hidden bg-gradient-to-r from-red-500 to-red-600 px-6 py-3 text-white shadow-lg transition-all duration-200 hover:scale-105 hover:from-red-600 hover:to-red-700 hover:shadow-xl active:scale-95"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-red-400 to-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
          <span className="relative flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            Show Error Notification
          </span>
        </Button>

        <Button
          onClick={showSuccessToast}
          className="group relative h-12 overflow-hidden bg-gradient-to-r from-green-500 to-green-600 px-6 py-3 text-white shadow-lg transition-all duration-200 hover:scale-105 hover:from-green-600 hover:to-green-700 hover:shadow-xl active:scale-95"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-green-400 to-green-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
          <span className="relative flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
            Show Success Toast
          </span>
        </Button>
      </div>

      <div className="flex justify-center">
        <Button
          onClick={clearAllNotifications}
          className="group relative h-12 overflow-hidden bg-gradient-to-r from-gray-600 to-gray-700 px-8 py-3 text-white shadow-lg transition-all duration-200 hover:scale-105 hover:from-gray-700 hover:to-gray-800 hover:shadow-xl active:scale-95"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-gray-500 to-gray-600 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
          <span className="relative flex items-center gap-2">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
              />
            </svg>
            Clear All Notifications
          </span>
        </Button>
      </div>
    </section>
  );
};
