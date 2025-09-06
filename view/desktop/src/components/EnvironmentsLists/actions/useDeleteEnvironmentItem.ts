import { useDeleteEnvironment } from "@/hooks";

export const useDeleteEnvironmentItem = () => {
  const { mutate: deleteEnvironment } = useDeleteEnvironment();
};
