import NotFoundPage from "@/pages/NotFoundPage";
import { WelcomePage } from "@/pages/welcome";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({ notFoundComponent: NotFoundPage });
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: WelcomePage,
});

export const welcomeRouter = createRouter({
  routeTree: rootRoute.addChildren([indexRoute]),
  history: createHashHistory(),
});
