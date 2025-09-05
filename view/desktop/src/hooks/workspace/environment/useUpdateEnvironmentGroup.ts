import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateEnvironmentGroupInput } from "@repo/moss-workspace";
import { useMutation } from "@tanstack/react-query";

const UPDATE_ENVIRONMENT_GROUP_QUERY_KEY = "updateEnvironmentGroup";
const updateEnvironmentGroup = async (input: UpdateEnvironmentGroupInput) => {
  const result = await invokeTauriIpc("update_environment_group", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateEnvironmentGroup = () => {
  return useMutation({
    mutationKey: [UPDATE_ENVIRONMENT_GROUP_QUERY_KEY],
    mutationFn: updateEnvironmentGroup,
  });
};
