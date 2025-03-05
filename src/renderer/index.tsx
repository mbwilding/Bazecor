// -*- mode: js-jsx -*-
/* Bazecor -- Kaleidoscope Command Center
 * Copyright (C) 2018, 2019  Keyboardio, Inc.
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import React from "react";
import { MemoryRouter } from "react-router-dom";
import { createRoot } from "react-dom/client";
import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import App from "./App";
import Error from "./Error";
import "./theme/styles.css";
import { DeviceProvider } from "./DeviceContext";
import ErrorBoundary from "./ErrorBoundary";

// Log pass-through to Rust
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
function forwardConsole(fnName: "log" | "debug" | "info" | "warn" | "error", rust: (message: string) => Promise<void>) {
  const java = console[fnName];
  console[fnName] = message => {
    java(message);
    rust(message);
  };
}
forwardConsole("log", trace);
forwardConsole("debug", debug);
forwardConsole("info", info);
forwardConsole("warn", warn);
forwardConsole("error", error);

const container = document.getElementById("root");
const root = createRoot(container);
try {
  root.render(
    <ErrorBoundary>
      <MemoryRouter>
        <DeviceProvider>
          <div>TEST</div>
        </DeviceProvider>
      </MemoryRouter>
    </ErrorBoundary>
  );
  // root.render(
  //   <ErrorBoundary>
  //     <MemoryRouter>
  //       <DeviceProvider>
  //         <App />
  //         <ToastContainer
  //           position="top-right"
  //           autoClose={false}
  //           hideProgressBar={false}
  //           newestOnTop={false}
  //           draggable={false}
  //           closeOnClick
  //           pauseOnHover
  //           pauseOnFocusLoss
  //         />
  //       </DeviceProvider>
  //     </MemoryRouter>
  //   </ErrorBoundary>,
  // );
} catch (e) {
  root.render(<Error error={e} />);
}
