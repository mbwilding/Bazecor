export interface StorageType {
  settings: {
    backupFolder: string;
    backupFrequency: number;
    language: string;
    darkMode: string;
    hideBluetoothExperimental?: boolean;
    showDefaults: boolean;
    autoUpdate: boolean;
    verbose: boolean;
    version: string;
  };
  neurons: unknown[];
}
