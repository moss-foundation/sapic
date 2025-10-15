import { appConfigService } from "./appConfigService";
import { languagesService } from "./languagesService";
import { themesService } from "./themesService";

//FIXME services should take only a Input types ideally
export const AppService = {
  ...appConfigService,
  ...languagesService,
  ...themesService,
};
