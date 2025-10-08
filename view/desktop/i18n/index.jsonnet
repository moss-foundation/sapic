local i18n = import "../../../i18n/index.libsonnet";

{
    [i18n.Namespace.Main]: i18n.Translation({
        "settings": "Settings",
        "home": "Home",
        "logs": "Logs",
        "noLogs": "No logs received yet",
        "receivedData": "Received data",
        "title": "Moss Studio",
        "selectTheme": "Select theme:",
        "selectLanguage": "Select language:",
        "user": "My name is: {{name}}",
    }),
}