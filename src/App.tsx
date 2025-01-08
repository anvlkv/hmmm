import { createSignal, onCleanup, onMount } from "solid-js";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { RouteSectionProps, useNavigate } from "@solidjs/router";
import { platform } from "@tauri-apps/plugin-os";

const currentPlatform = platform();

function App(props: RouteSectionProps) {
  const navigate = useNavigate();

  const listeners: UnlistenFn[] = [];

  const onNavigate = ({ payload }: { payload: string }) => {
    navigate(`/${payload}`);
  };

  onMount(async () => {
    listeners.push(await listen<string>("navigate", onNavigate));

    await invoke("mount");
    await invoke("greet", { name: "tauri launched" });
  });

  onCleanup(() => {
    listeners.forEach((unlisten) => unlisten());
  });

  return (
    <main class="container">
      <h1 data-tauri-drag-region>
        Welcome to Tauri + Solid, running on {currentPlatform}
      </h1>
      {props.children}
    </main>
  );
}

export default App;
