import { tokenizerService } from "@/workbench/domains/tokenizer";
import { useMutation } from "@tanstack/react-query";

export const useTokenizer = () => {
  return useMutation({
    mutationKey: ["parse-url"],
    mutationFn: async (url: string) => await tokenizerService.getTokens(url),
  });
};
