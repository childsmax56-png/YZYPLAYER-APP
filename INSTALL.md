# Installing YZYPlayer App

## macOS (available now)

1. **Download** `yzyplayer-app_0.1.0_aarch64.dmg`.
2. **Double-click** the `.dmg` file — a window opens showing the app icon and an arrow to the Applications folder.
3. **Drag** the `yzyplayer-app` icon into **Applications**.
4. **Eject** the mounted disk image in Finder's sidebar — the `.dmg` isn't needed after this.
5. **Open** the app from Applications or Spotlight (`Cmd+Space`).

### First-time security warning
macOS will likely block the app the first time since it isn't signed with an Apple Developer certificate (*"yzyplayer-app" can't be opened because it is from an unidentified developer*). To get past this:
- **Right-click** (or Control-click) the app icon → **Open** → click **Open** again in the dialog.
- This is only needed the first launch.

> Note: this `.dmg` is built for Apple Silicon (M1/M2/M3/M4) Macs. Intel Macs would need a separate build.

## Windows (not yet built)

A `.msi` or `.exe` installer hasn't been produced yet — building one requires either a Windows machine or a CI pipeline (Tauri can't cross-compile a Windows installer from macOS). Once built, the steps would be:

1. Download the `.msi` (or `.exe`) installer.
2. Double-click it and follow the install wizard.
3. Windows may show a SmartScreen warning since the app isn't code-signed — click **More info** → **Run anyway**.
4. Launch from the Start menu.

## Linux (not yet built)

Not yet built either — would produce a `.deb`, `.rpm`, or `.AppImage` depending on distro. Once built:

- **`.AppImage`**: make it executable (`chmod +x yzyplayer-app.AppImage`) and double-click or run it — no install needed.
- **`.deb`** (Debian/Ubuntu): `sudo dpkg -i yzyplayer-app.deb`
- **`.rpm`** (Fedora): `sudo rpm -i yzyplayer-app.rpm`

## Using the app (all platforms)

- It loads the live site directly, so content updates automatically — no need to reinstall when the site changes.
- Clicking song/project links opens them in a separate in-app window.
- `Cmd+W` (Mac) / `Ctrl+W` (Windows/Linux) closes that pop-up and returns to the main view.
- `Cmd+←` / `Cmd+→` (or `Ctrl+` equivalents) go back/forward; `Cmd+Shift+H` jumps back to the homepage.
