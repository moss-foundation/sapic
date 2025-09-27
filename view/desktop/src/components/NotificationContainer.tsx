import { createPortal } from "react-dom";

import { useNotifications } from "@/app/NotificationProvider";
import { Notification } from "@/lib/ui";
import { cn } from "@/utils";

export const NotificationContainer = () => {
  const { notifications, removeNotification } = useNotifications();

  if (notifications.length === 0) {
    return null;
  }

  return createPortal(
    <div className="fixed right-4 bottom-4 z-[10000] flex max-w-sm flex-col gap-3">
      {notifications.map((notification, index) => (
        <div
          key={notification.id}
          className={cn(
            "transform transition-all duration-300 ease-in-out",
            index === notifications.length - 1 ? "animate-in slide-in-from-right-full" : "animate-in fade-in-0"
          )}
          style={{
            animationDelay: index === notifications.length - 1 ? "0ms" : `${index * 100}ms`,
          }}
        >
          <div className="relative">
            <Notification
              {...notification}
              className={cn(notification.className, "max-w-sm min-w-80 shadow-xl")}
              onButtonClick={() => {
                notification.onButtonClick?.();
                // Keep notification open after button click unless explicitly closed
              }}
              onLinkClick={() => {
                notification.onLinkClick?.();
                // Keep notification open after link click unless explicitly closed
              }}
              onClick={(e) => {
                // Allow clicking anywhere on notification to dismiss it
                if (e.target === e.currentTarget) {
                  removeNotification(notification.id);
                }
              }}
            />
            {/* Close button overlay */}
            <button
              onClick={(e) => {
                e.stopPropagation();
                removeNotification(notification.id);
              }}
              className="absolute top-2 right-2 flex h-5 w-5 cursor-pointer items-center justify-center rounded-full bg-black/20 text-xs text-white opacity-70 transition-opacity hover:opacity-100"
              aria-label="Close notification"
            >
              ✕
            </button>
          </div>
        </div>
      ))}
    </div>,
    document.body
  );
};
