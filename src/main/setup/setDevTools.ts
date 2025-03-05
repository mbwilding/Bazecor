import log from "electron-log/main";

const setDevTools = async () => {
  if (process.env.NODE_ENV !== "production") {
    try {
      const devTools = await import("electron-devtools-installer");
      const extensionLoaded = await devTools.default(devTools.REACT_DEVELOPER_TOOLS);
      log.log(`Extension loaded ${extensionLoaded}`);
    } catch (err) {
      log.log("Error while loading dev tools: ", err);
    }
  }
};

export default setDevTools;
