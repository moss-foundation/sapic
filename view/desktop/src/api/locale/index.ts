import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { GetTranslationsInput } from "@repo/moss-nls";

interface I18nDictionary {
  [key: string]: string;
}

export const getTranslations = async (input: GetTranslationsInput): Promise<IpcResult<I18nDictionary, string>> => {
  return await invokeTauriIpc<I18nDictionary, string>("get_translations", {
    input: input,
  });
};
