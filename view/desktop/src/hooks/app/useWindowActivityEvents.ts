import { useActivityRouter } from "./useActivityRouter";

export const useWindowActivityEvents = () => {
  const { windowEvents } = useActivityRouter();

  return windowEvents;
};
