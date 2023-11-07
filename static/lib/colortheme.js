function addSchemeToHeader(is_dark) {
    let is_dark_as_bool = (is_dark === "true");
    let link = document.createElement('link');
    link.rel = "stylesheet";
    link.href = is_dark_as_bool ? "/static/styles/dark_mode.css" : "/static/styles/light_mode.css";
    link.id = "color-theme-link";
    document.head.appendChild(link);
}

function loadColorScheme() {
    let saved_theme = localStorage.getItem("coatesweb-is-dark-mode");
    if (saved_theme) {
        addSchemeToHeader(saved_theme);
    } else {
        addSchemeToHeader(window.matchMedia("prefers-color-scheme: dark").matches);
    }
}

function setColorScheme(is_dark) {
    let link = document.getElementById("color-theme-link");
    link.remove();
    localStorage.setItem("coatesweb-is-dark-mode", is_dark);
    loadColorScheme();
}

loadColorScheme();