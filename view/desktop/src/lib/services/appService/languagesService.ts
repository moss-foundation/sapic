import { invokeTauriServiceIpc } from "@/lib/backend/tauri";
import { GetTranslationNamespaceInput, GetTranslationNamespaceOutput, ListLocalesOutput } from "@repo/moss-app";

export const languagesService = {
  listLocales: async () => {
    return await invokeTauriServiceIpc<void, ListLocalesOutput>({ cmd: "list_languages" });
  },

  getTranslationNamespace: async (input: GetTranslationNamespaceInput) => {
    return await invokeTauriServiceIpc<GetTranslationNamespaceInput, GetTranslationNamespaceOutput>({
      cmd: "get_translation_namespace",
      args: {
        input,
      },
    });
  },
};
