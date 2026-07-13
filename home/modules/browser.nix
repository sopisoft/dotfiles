{
  config,
  pkgs,
  ...
}: {
  programs.firefox = {
    enable = true;
    package = pkgs.firefox;
    configPath = "${config.xdg.configHome}/mozilla/firefox";

    profiles.default = {
      id = 0;
      isDefault = true;

      settings = {
        "app.normandy.first_run" = false;
        "browser.aboutConfig.showWarning" = false;
        "browser.newtabpage.activity-stream.feeds.section.topstories" = false;
        "browser.search.suggest.enabled" = false;
        "browser.shell.checkDefaultBrowser" = false;
        "browser.startup.page" = 3;
        "browser.toolbars.bookmarks.visibility" = "never";
        "browser.urlbar.suggest.searches" = false;
        "extensions.pocket.enabled" = false;
        "toolkit.legacyUserProfileCustomizations.stylesheets" = true;
      };
    };
  };
}
