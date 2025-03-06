import { app } from "electron";
import fs from "fs";
import path from "path";
import moment from "moment";
const log = console;
import Store from "../managers/Store";

function deleteOldFiles(backupPath: string, period: number) {
  // Get the current date
  const currentDate = moment();

  // Get the list of files in the directory
  const files = fs.readdirSync(backupPath);

  // Iterate over each file
  for (const file of files) {
    const filePath = path.join(backupPath, file);
    const fileStats = fs.statSync(filePath);

    // Check if the file is a directory
    if (fileStats.isDirectory()) {
      // Recursively delete files in subdirectories
      deleteOldFiles(filePath, period);
    } else {
      // Get the file's last modified date
      const lastModifiedDate = moment(fileStats.mtime);

      // Calculate the difference in months between the current date and the file's last modified date
      const monthsDifference = currentDate.diff(lastModifiedDate, "months");

      // Check if the file is older than 6 months
      if (monthsDifference >= period) {
        // Delete the file
        fs.unlinkSync(filePath);
        log.log(`Deleted file: ${filePath}`);
      }
    }
  }
}

const setBackup = async () => {
  const store = Store.getStore();
  const bfolder = await store.get("settings.backupFolder") as string;
  const bfrequency = await store.get("settings.backupFrequency") as number;
  log.log("** Checking backup folder value **");
  log.log(bfolder);
  if (bfolder === "" || bfolder === undefined) {
    const defaultPath = path.join(app.getPath("home"), "Dygma", "Backups");
    log.log(defaultPath);
    store.set("settings.backupFolder", defaultPath);
    fs.mkdir(defaultPath, { recursive: true }, err => {
      if (err) {
        log.error(err);
      }
      log.log("Directory created successfully!");
    });
  } else if (bfrequency > 0 && bfrequency < 13) {
    log.log(`** Going to erase backups older than: ${bfrequency} months **`);
    deleteOldFiles(bfolder, bfrequency);
  }
};

export default setBackup;
