/* @refresh reload */
import { render } from "solid-js/web";
import { RouteDefinition, Router } from "@solidjs/router";
import { lazy } from "solid-js";

const routes: RouteDefinition[] = [
  {
    path: "/",
    component: lazy(() => import("./App")),
    children: [
      { path: "/", component: () => <h2>Home</h2> },
      { path: "/greet", component: lazy(() => import("./Greet")) },
    ],
  },
];

render(
  () => <Router>{routes}</Router>,
  document.getElementById("root") as HTMLElement,
);
