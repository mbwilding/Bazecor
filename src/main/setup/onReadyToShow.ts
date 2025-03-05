import { checkUdev, installUdev } from "../utils/udev";
import Window from "../managers/Window";
import autoUpdateOptIn from "../utils/autoUpdateOptIn";
import { platform } from "@tauri-apps/plugin-os";

const onReadyToShow = () => {
  const window = Window.getWindow();
  window.once("ready-to-show", () => {
    window.show();
    if (platform() === "linux") {
      if (!checkUdev()) {
        installUdev(window);
      }
    }
    autoUpdateOptIn(window);
  });
};

export default onReadyToShow;
