// Create a new router instance
import NotFound from "@/app/NotFound";
import Providers from "@/app/Providers";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({ notFoundComponent: NotFound });
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: Providers,
});

const workspaceIdRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/$workspaceId",
  component: Providers,
});

export const router = createRouter({
  routeTree: rootRoute.addChildren([indexRoute, workspaceIdRoute]),
  history: createHashHistory(),
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
