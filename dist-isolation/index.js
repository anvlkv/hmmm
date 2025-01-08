window.__TAURI_ISOLATION_HOOK__ = (payload, options) => {
  console.log("hook", payload, options);
  return payload;
};
