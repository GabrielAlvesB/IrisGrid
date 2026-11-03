# IrisGrid app icon

A spreadsheet page with a header row, a grid and a selected cell with its fill handle, on a
green-to-teal tile. It is the same drawing the Iris Workspace shows for IrisGrid
(`Iris/src/shared/types/workspace.types.ts`, entry `irisgrid`, hue 152), so the app looks the same
in the Workspace, on the taskbar and in the Start menu.

## Palette

The gradient is the Workspace's `oklch(0.66 0.2 152)` → `oklch(0.6 0.21 194)`, written in sRGB so
every renderer draws the same colours:

| Colour | Hex | Used for |
|---|---|---|
| Green | `#00b24e` | Gradient start (top left) |
| Teal | `#00a2a5` | Gradient end (bottom right) |
| White | `#ffffff` | The page (95% opacity) and the fill-handle outline |

The update prompt's accent (`UPDATE_APP.accent` in `crates/ui-egui/src/lib.rs`) is `#008f4f`.

## Geometry

A 64 × 64 viewBox with the tile at 4..60 (`rx=15`). Windows and Linux use it as is. The macOS
icons (`.icns`, `gridcraft-macos-512.png`) sit on Apple's grid: an 824/1024 body with a
transparent margin.

## Files

| File | Use |
|---|---|
| `gridcraft.svg` | Canonical artwork; also `hicolor/scalable` |
| `gridcraft-1024.png` | 1024 px render of the full tile |
| `gridcraft-macos-512.png` | Runtime Dock icon on macOS (for `apps/gridcraft` to embed) |
| `gridcraft.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `gridcraft.ico` | Windows exe icon, 16–256 px (embedded by `apps/gridcraft/build.rs`) |
| `hicolor/<size>/apps/com.iris.irisgrid.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/com.iris.irisgrid.svg` | Linux scalable icon |
| `LICENSE.txt` | Terms for these files |

The file names keep the `gridcraft` stem on purpose (see `CLAUDE.md`); the app ID is
`com.iris.irisgrid`.

## Regenerate

Edit `gridcraft.svg`, then run `packaging/icons.sh`. It needs `resvg`, and `iconutil` for the
`.icns` (macOS only). It packs the `.ico` with `cargo xtask ico`.
