import { invoke } from "@tauri-apps/api/core";
import Store from "../managers/Store";
import sendToRenderer from "../utils/sendToRenderer";

const onThemeChange = () => () => {
  sendToRenderer("darkTheme-update", nativeTheme.shouldUseDarkColors);
};

const configureNativeTheme = () => {
  nativeTheme.on("updated", onThemeChange());
};

const setTheme = async () => {
  const store = Store.getStore();
  let darkMode = await store.get<string>("settings.darkMode");
  if (typeof darkMode === "boolean" || darkMode === undefined) {
    darkMode = "auto";
    await store.set("settings.darkMode", "auto");
  }

  await invoke("plugin:theme|set_theme", {
    theme: darkMode
  });
};

export { configureNativeTheme, setTheme, onThemeChange };
