import { Toaster } from "sonner";

export const NotificationsProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <Toaster position="bottom-right" richColors={false} />
      {children}
    </>
  );
};
