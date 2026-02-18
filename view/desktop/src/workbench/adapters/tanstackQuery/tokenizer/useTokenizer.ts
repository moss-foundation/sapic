import { tokenizerService } from "@/workbench/services/tokenizerService";
import { useMutation } from "@tanstack/react-query";

export const useTokenizer = () => {
  return useMutation({
    mutationKey: ["parse-url"],
    mutationFn: async (url: string) => await tokenizerService.getTokens(url),
  });
};
