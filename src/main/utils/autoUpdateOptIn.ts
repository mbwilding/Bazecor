import { BrowserWindow, dialog, MessageBoxOptions } from "electron";
import Store from "../managers/Store";
import { platform } from "@tauri-apps/plugin-os";

const os = platform();

const store = Store.getStore();

const autoUpdateOptIn = async (mainWindow: BrowserWindow) => {
  const autoUpdate = await store.get<boolean>("settings.autoUpdate");

  if (autoUpdate === undefined && os !== "linux") {
    let dialogOpts: MessageBoxOptions;
    if (os !== "macos") {
      dialogOpts = {
        type: "question",
        buttons: ["Decline", "Consent"],
        cancelId: 0,
        defaultId: 1,
        title: "Bazecor auto update tool",
        message: "Bazecor will update automatically whenever possible",
        detail:
          "Do you want to make Bazecor able to auto update automatically?, click consent to authorize automatic downloads of new versions, if you decline this feature, you can enable it again in preferences.",
      };
      dialog.showMessageBox(mainWindow, dialogOpts).then(response => {
        if (response.response === 1) {
          store.set("settings.autoUpdate", true);
        } else {
          store.set("settings.autoUpdate", false);
        }
      });
    } else {
      dialogOpts = {
        type: "question",
        buttons: ["Consent", "Decline"],
        cancelId: 1,
        defaultId: 0,
        title: "Bazecor auto update tool",
        message: "Bazecor will update automatically whenever possible",
        detail:
          "Do you want to make Bazecor able to auto update automatically?, click consent to authorize automatic downloads of new versions, if you decline this feature, you can enable it again in preferences.",
      };
      dialog.showMessageBox(mainWindow, dialogOpts).then(response => {
        if (response.response === 0) {
          store.set("settings.autoUpdate", true);
        } else {
          store.set("settings.autoUpdate", false);
        }
      });
    }
  }
};

export default autoUpdateOptIn;
