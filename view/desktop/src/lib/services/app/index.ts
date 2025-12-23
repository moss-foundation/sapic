import { languagesService } from "./languagesService";
import { themesService } from "./themesService";

/**
 * @deprecated this should be moved to domain
 */
export const AppService = {
  ...languagesService,
  ...themesService,
};
