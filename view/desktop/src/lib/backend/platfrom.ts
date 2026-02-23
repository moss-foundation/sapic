import { invoke, InvokeArgs } from "@tauri-apps/api/core";

export const invokeMossCommand = async <T>(cmd: string, args?: InvokeArgs): Promise<T> => {
  return await invoke<T>("execute_command", {
    cmd,
    args,
  });
};
