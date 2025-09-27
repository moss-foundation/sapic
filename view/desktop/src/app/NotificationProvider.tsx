import React, { createContext, useCallback, useContext, useState } from "react";

import { NotificationProps } from "@/lib/ui";

export interface NotificationData extends Omit<NotificationProps, "onButtonClick" | "onLinkClick"> {
  id: string;
  duration?: number; // Auto-dismiss duration in ms
  onButtonClick?: () => void;
  onLinkClick?: () => void;
}

interface NotificationContextType {
  notifications: NotificationData[];
  addNotification: (notification: Omit<NotificationData, "id">) => string;
  removeNotification: (id: string) => void;
  clearAllNotifications: () => void;
}

const NotificationContext = createContext<NotificationContextType>({
  notifications: [],
  addNotification: () => "",
  removeNotification: () => {},
  clearAllNotifications: () => {},
});

export const useNotifications = () => useContext(NotificationContext);

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [notifications, setNotifications] = useState<NotificationData[]>([]);

  const addNotification = useCallback((notification: Omit<NotificationData, "id">) => {
    const id = Math.random().toString(36).substr(2, 9);
    const newNotification: NotificationData = {
      ...notification,
      id,
      duration: notification.duration !== undefined ? notification.duration : 2000, // Default 2 seconds
    };

    setNotifications((prev) => [...prev, newNotification]);

    // Auto-dismiss if duration is set and greater than 0
    if (newNotification.duration && newNotification.duration > 0) {
      setTimeout(() => {
        removeNotification(id);
      }, newNotification.duration);
    }

    return id;
  }, []);

  const removeNotification = useCallback((id: string) => {
    setNotifications((prev) => prev.filter((notification) => notification.id !== id));
  }, []);

  const clearAllNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  const contextValue = {
    notifications,
    addNotification,
    removeNotification,
    clearAllNotifications,
  };

  return <NotificationContext.Provider value={contextValue}>{children}</NotificationContext.Provider>;
};
