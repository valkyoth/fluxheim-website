(function () {
  const storageKey = "fluxheim-theme";
  const windowNameKey = "fluxheim-theme=";
  const root = document.documentElement;

  function validTheme(theme) {
    return theme === "light" || theme === "dark" ? theme : null;
  }

  function readLocalStorage() {
    try {
      const stored = localStorage.getItem(storageKey);
      return validTheme(stored);
    } catch (_) {}
    return null;
  }

  function writeLocalStorage(theme) {
    try {
      localStorage.setItem(storageKey, theme);
    } catch (_) {}
  }

  function readCookie() {
    if (!document.cookie) return null;
    const match = document.cookie
      .split(";")
      .map((item) => item.trim())
      .find((item) => item.startsWith(storageKey + "="));
    return validTheme(match && decodeURIComponent(match.split("=").slice(1).join("=")));
  }

  function writeCookie(theme) {
    try {
      document.cookie =
        storageKey +
        "=" +
        encodeURIComponent(theme) +
        "; path=/; max-age=31536000; SameSite=Lax";
    } catch (_) {}
  }

  function readWindowName() {
    const match = (window.name || "")
      .split(";")
      .map((item) => item.trim())
      .find((item) => item.startsWith(windowNameKey));
    return validTheme(match && match.slice(windowNameKey.length));
  }

  function writeWindowName(theme) {
    const parts = (window.name || "")
      .split(";")
      .map((item) => item.trim())
      .filter((item) => item && !item.startsWith(windowNameKey));
    parts.push(windowNameKey + theme);
    window.name = parts.join("; ");
  }

  function preferredTheme() {
    return (
      readLocalStorage() ||
      readCookie() ||
      readWindowName() ||
      (window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: light)").matches
        ? "light"
        : "dark")
    );
  }

  function applyTheme(theme) {
    const isDark = theme !== "light";
    root.classList.toggle("dark", isDark);
    root.classList.toggle("light", !isDark);
    root.style.colorScheme = isDark ? "dark" : "light";

    if (document.body) {
      document.body.classList.remove("dark", "light");
      document.body.classList.add(isDark ? "dark" : "light");
    }

    document.querySelectorAll("[data-theme-toggle]").forEach((button) => {
      button.setAttribute(
        "aria-label",
        isDark ? "Switch to light mode" : "Switch to dark mode",
      );
      button.setAttribute(
        "title",
        isDark ? "Switch to light mode" : "Switch to dark mode",
      );
    });
  }

  function setTheme(theme) {
    writeLocalStorage(theme);
    writeCookie(theme);
    writeWindowName(theme);
    applyTheme(theme);
  }

  applyTheme(preferredTheme());

  window.FluxheimTheme = {
    apply: setTheme,
    current: function () {
      return root.classList.contains("light") ? "light" : "dark";
    },
    toggle: function () {
      setTheme(root.classList.contains("dark") ? "light" : "dark");
    },
  };

  document.addEventListener("DOMContentLoaded", function () {
    applyTheme(preferredTheme());
    document.querySelectorAll("[data-theme-toggle]").forEach((button) => {
      button.addEventListener("click", window.FluxheimTheme.toggle);
    });
  });
})();
