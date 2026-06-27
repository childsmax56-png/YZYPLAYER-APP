# Installing YZYPlayer App

All installers are hosted in the **Downloads** section of the YZYPlayer site itself — no GitHub account needed.

## macOS

1. **Download** `yzyplayer-app_0.1.0_aarch64.dmg` from the site's Downloads section.
2. **Double-click** the `.dmg` file — a window opens showing the app icon and an arrow to the Applications folder.
3. **Drag** the `yzyplayer-app` icon into **Applications**.
4. **Eject** the mounted disk image in Finder's sidebar — the `.dmg` isn't needed after this.
5. **Open** the app from Applications or Spotlight (`Cmd+Space`).

### First-time security warning
macOS will likely block the app the first time since it isn't signed with an Apple Developer certificate (*"yzyplayer-app" can't be opened because it is from an unidentified developer*). To get past this:
- **Right-click** (or Control-click) the app icon → **Open** → click **Open** again in the dialog.
- This is only needed the first launch.

> Note: this `.dmg` is built for Apple Silicon (M1/M2/M3/M4) Macs. Intel Macs would need a separate build.

## Windows

1. **Download** the `.msi` (or `.exe`) installer from the site's Downloads section.
2. Double-click it and follow the install wizard.
3. Windows will likely show a SmartScreen warning since the app isn't code-signed — click **More info** → **Run anyway**.
4. Launch from the Start menu.

## Linux

1. **Download** the file from the site's Downloads section — depending on what's offered:
   - **`.AppImage`**: `chmod +x yzyplayer-app*.AppImage` then double-click or run it — no install needed.
   - **`.deb`** (Debian/Ubuntu): `sudo dpkg -i yzyplayer-app*.deb`
   - **`.rpm`** (Fedora): `sudo rpm -i yzyplayer-app*.rpm`

## Using the app (all platforms)

- It loads the live site directly, so content updates automatically — no need to reinstall when the site changes.
- Clicking song/project links opens them in a separate in-app window.
- `Cmd+W` (Mac) / `Ctrl+W` (Windows/Linux) closes that pop-up and returns to the main view.
- `Cmd+←` / `Cmd+→` (or `Ctrl+` equivalents) go back/forward; `Cmd+Shift+H` jumps back to the homepage.
