import { ITokenizerIpc } from "@/shared/tokenizer/ipc";

import { invokeTauriIpc } from "./tauri";

export const tokenizerIpc: ITokenizerIpc = {
  getTokens: async (url: string) => {
    return await invokeTauriIpc("plugin:template-parser|parse_url", {
      input: { url },
    });
  },
};
