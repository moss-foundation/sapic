import { useActivityRouter } from "../../../../../hooks/app/useActivityRouter";

export const useWindowActivityEvents = () => {
  const { windowEvents } = useActivityRouter();

  return windowEvents;
};
