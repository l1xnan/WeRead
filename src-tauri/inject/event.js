window.addEventListener("DOMContentLoaded", (_event) => {
  // https://github.com/tauri-apps/tauri/issues/8476
  const {
    core: { invoke },
    event,
  } = window.__TAURI__;
  event.emit("location", {
    href: location.href,
  });

  invoke("change_route", { href: location.href });
});
