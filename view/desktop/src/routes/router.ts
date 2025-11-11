// Create a new router instance
import App from "@/app";
import NotFound from "@/app/NotFound";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: App,
});

const welcomeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/welcome.html",
  component: App,
});

const rootRoute = createRootRoute({ notFoundComponent: NotFound });

const workspaceIdRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/$workspaceId",
  component: App,
});

export const router = createRouter({
  routeTree: rootRoute.addChildren([indexRoute, welcomeRoute, workspaceIdRoute]),
  history: createHashHistory(),
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
