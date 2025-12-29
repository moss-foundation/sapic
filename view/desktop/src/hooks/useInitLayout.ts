import { useGetLayout } from "@/workbench/adapters/tanstackQuery/layout";
import { useParams } from "@tanstack/react-router";

export const useInitLayout = () => {
  const { workspaceId } = useParams({ strict: false });
  const { isSuccess: isSuccessLayout } = useGetLayout(workspaceId);

  return { isInit: isSuccessLayout };
};
