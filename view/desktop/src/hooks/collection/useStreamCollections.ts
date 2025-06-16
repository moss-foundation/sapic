import { listen } from "@tauri-apps/api/event";

export const USE_STREAM_COLLECTIONS_QUERY_KEY = "streamCollections";

export const useStreamCollections = () => {
  //   return useQuery({
  //     queryKey: [USE_STREAM_COLLECTIONS_QUERY_KEY],
  //     queryFn: () => {
  //       return invokeTauriIpc<StreamCollectionsEvent>("stream_collections");
  //     },
  //   });
  console.log("stream_collections");
  listen("stream_collections", (event) => {
    console.log(event.payload);
  });
};
