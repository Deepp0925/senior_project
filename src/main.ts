import "./styles.css";
import App from "./App.svelte";

import feather from "feather-icons/dist/feather";

const app = new App({
  target: document.getElementById("app"),
});

feather.replace();

export default app;
