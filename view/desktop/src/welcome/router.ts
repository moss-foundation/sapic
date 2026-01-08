import WelcomeIndex from "@/welcome/index";
import { WelcomePage } from "@/welcome/pages";
import { ExtensionsPage } from "@/welcome/pages/ExtensionsPage";
import NotFoundPage from "@/welcome/pages/NotFoundPage";
import { SettingsPage } from "@/welcome/pages/SettingsPage";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({
  component: WelcomeIndex,
  notFoundComponent: NotFoundPage,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: WelcomePage,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  component: SettingsPage,
});

const extensionsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/extensions",
  component: ExtensionsPage,
});

export const welcomeRouter = createRouter({
  routeTree: rootRoute.addChildren([indexRoute, settingsRoute, extensionsRoute]),
  history: createHashHistory(),
});
