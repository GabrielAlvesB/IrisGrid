# IrisGrid — instructions for agents

## LEIA PRIMEIRO: o IrisGrid é uma cópia modificada do GridCraft

**Esta seção vale mais que todo o resto do arquivo.** O resto veio do GridCraft; onde ele contradizer esta seção, esta seção vence.

O IrisGrid é uma cópia (fork) do [GridCraft](https://github.com/storytold/gridcraft), da equipe ArtCraft, adaptada para o ecossistema Iris por **Gabriel Alves Tech**. O repositório é `GabrielAlvesB/IrisGrid` (remote `origin`); o GridCraft original é o remote `upstream`, usado **só para trazer atualizações**: nunca envie nada para ele. O IrisWord (`../IrisWord`) é a cópia irmã do WordCraft e segue as mesmas regras.

Todos os ajustes listados abaixo foram feitos de propósito. **Nenhuma atualização vinda do GridCraft pode desfazê-los.** Ao trazer uma versão nova, o trabalho é incorporar as melhorias do GridCraft **mantendo o IrisGrid como ele é**. Em qualquer conflito entre o código do GridCraft e um ajuste do IrisGrid, fica o do IrisGrid. Na dúvida, pergunte ao dono antes de mudar.

### O que é do IrisGrid e não pode ser revertido

1. **Nome visível "IrisGrid"** em tudo que o usuário vê: título da janela, Sobre, mensagens, PDF e `.xlsx` gerados (`Application`), README, instaladores, pacotes. "GridCraft" só aparece nos créditos e avisos de copyright (`LICENSE-MIT`, `NOTICE`, as linhas "Based on GridCraft" e de copyright em `crates/ui-egui/src/dialogs/about.rs`, a seção de licença do README e as frases "GridCraft contributors" / "GridCraft authors"); esses avisos são exigidos pela licença e também não podem ser removidos.
2. **ID do app `com.iris.irisgrid`** (nunca `ai.storyteller.gridcraft`): `packaging/linux/com.iris.irisgrid.*`, `packaging/linux/flatpak/com.iris.irisgrid*.yml`, `packaging/macos/Info.plist.in`, os ícones em `assets/app-icon/hicolor/*/apps/`, `omastore.toml`.
3. **Ícone do IrisGrid** (folha de planilha com linha de cabeçalho, grade e célula selecionada com a alça de preencher, gradiente `#00b24e` → `#00a2a5`, o mesmo desenho do Workspace do Iris): a fonte é `assets/app-icon/gridcraft.svg`; todo PNG, `.ico` e `.icns` de `assets/app-icon/` é gerado dela (`packaging/icons.sh`, viewBox 64). Nunca aceite ícones do GridCraft por cima destes.
4. **Sem marca ArtCraft**: sem `docs/brand/`, sem logos ArtCraft, sem links para `getartcraft.com` ou `discord.gg/artcraft`, sem itens de Discord (removidos do Sobre, do menu Ajuda do macOS em `apps/gridcraft/src/native_menu.rs` e das traduções), sem a lista "Crafting Apps" no README. Se uma atualização trouxer menções novas à ArtCraft ou à suíte Crafting Apps na interface, retire-as. A licença de marca da ArtCraft exige isso de forks.
5. **Atualizador** (`UPDATE_APP` em `crates/ui-egui/src/lib.rs`): `name: "IrisGrid"`, `repo: "GabrielAlvesB/IrisGrid"` (exatamente assim: o atualizador compara os links diferenciando maiúsculas), `developer: "Gabriel Alves Tech"`, `accent` `#008f4f`. Nunca volte para `craft_update::suite::GRIDCRAFT` puro: o app passaria a baixar o GridCraft por cima do IrisGrid. A verificação ao abrir fica ligada (`app.start_update_check()` em `apps/gridcraft/src/main.rs`).
6. **Instalador do Windows** (`packaging/windows/`): `UpgradeCode` `D6F1A0C2-6E3B-4B8E-9C47-2A51E0B7F3D9` (em `gridcraft.wxs` e `install-policy.json`) e GUID do componente `GridcraftApp` `B3C8E4F1-27D5-4A6C-8E19-5F0D2C7A9B64` (nunca os do GridCraft, senão um instalador substitui o outro), fabricante `Gabriel Alves Tech`, pasta `Programs\Iris\IrisGrid`, chaves de registro `Software\IrisGrid`. Em `apps/gridcraft/build.rs` e `packaging/windows/app-build.rs.in`, `CompanyName` é `Gabriel Alves Tech`.
7. **Traduções**: as traduções ficam no próprio repositório (`crates/ui-egui/src/i18n/*.rs`) e as chaves já dizem IrisGrid. O português do Brasil (`pt.rs`) tem as linhas novas do Sobre; ao trazer textos novos com "GridCraft", troque nas chaves **e** nas traduções, senão a tradução deixa de casar.
8. **CI próprio**: `.github/workflows/ci.yml` é autocontido (sem segredos, token só de leitura, actions fixadas por SHA). **Nunca restaure** os workflows do GridCraft (`storytold/craft-common-actions@...`, SonarCloud, release, FreeBSD, nix, packaging-lint, windows-arm64), nem `sonar-project.properties` ou `.coderabbit.yaml`: eles rodam código de outra organização e repassam segredos. O Dependabot roda semanalmente (`America/Sao_Paulo`).
9. **Segurança**: `zip` na versão 9 ou mais nova (`Cargo.toml` do workspace) e `webpki-root-certs` 1.0.10 ou mais nova (`Cargo.lock`). Não volte para versões antigas (RUSTSEC-2026-0367/0368/0369/0429 e RUSTSEC-2026-0383).
10. **Nomes internos mantidos de propósito**: crates `gridcraft-*`, binários `gridcraft.exe` e `gridcraft-cli`, pastas `apps/gridcraft*`, arquivos `assets/app-icon/gridcraft.*`, `packaging/windows/gridcraft.wxs`, variáveis `GRIDCRAFT_*` e os nomes dos arquivos de release (`gridcraft-<versão>-windows-<arq>.msi`, `...-portable.zip`). O atualizador e o Workspace do Iris (`../Iris/src/shared/types/workspace.types.ts`) procuram exatamente esses nomes. Não renomeie.

### Como trazer uma atualização do GridCraft

1. `git fetch upstream --tags` e escolha a tag nova (por exemplo `v0.17.0`). Faça o merge numa branch: `git switch -c update-gridcraft-0.17.0 && git merge v0.17.0`.
2. Resolva os conflitos mantendo **tudo** da lista acima. Arquivos removidos no IrisGrid (`docs/brand/`, workflows antigos, `sonar-project.properties`, `.coderabbit.yaml`) continuam removidos.
3. Procure o que entrou de novo com a marca antiga e adapte: `git grep -n -e GridCraft -e ArtCraft -e getartcraft -e discord.gg -e ai.storyteller -e "storytold/gridcraft/releases" -e "Learning Machines"`. Texto visível novo vira IrisGrid; avisos de copyright e créditos ficam como estão.
4. Arquivos novos em `.github/workflows/` vindos do GridCraft: apague. Ícones ou imagens novas de marca: não adote.
5. Se a revisão de `craft-libs` mudou, confira se `craft_update::app::App` ainda aceita os campos do `UPDATE_APP`.
6. Rode `cargo audit`, `cargo xtask assets` e os testes: `cargo test --no-fail-fast -p gridcraft-core -p gridcraft-numfmt -p gridcraft-formula -p gridcraft-functions -p gridcraft-model -p gridcraft-calc -p gridcraft-xlsx -p gridcraft-engine -p gridcraft-ui-egui -p xtask`. Sem `CRAFT_FONTS_DIR` (clone de `storytold/craft-fonts` no commit do `ci.yml`, fora da pasta do ecossistema) dois testes de interface falham por falta de fonte (ẞ, ẻ), e no Windows `engineering::tests::complex_numbers` falha na 15ª casa decimal; nenhum dos três tem relação com as mudanças do IrisGrid (o código deles não foi tocado).
7. **Versão**: a versão do IrisGrid (`[workspace.package]` no `Cargo.toml`) precisa ser **maior que a da última release do IrisGrid**, senão o atualizador não oferece a nova. Rode `cargo update -w` depois de mudar.
8. Mostre ao dono o que mudou antes de fazer merge na `main` e de publicar release.

### Como publicar uma release

1. Versão nova no `Cargo.toml`, `cargo update -w`, commit e push na `main`.
2. Rode `packaging/windows/package.ps1 -Arch x64` num processo `powershell.exe` separado, com a saída indo para arquivo (no PowerShell 5.1, redirecionar dentro do mesmo processo transforma avisos do cargo em erro fatal). Precisa do WiX 5 (`dotnet tool install --global wix --version 5.0.2`). Saída em `dist/release/`.
3. Gere `dist/release/SHA256SUMS.txt` no formato `hash  nome` (dois espaços, sem `*`). O Workspace do Iris não instala sem ele.
4. `gh release create v<versão> -R GabrielAlvesB/IrisGrid --title "IrisGrid v<versão>" --notes-file <notas.md> --latest` com o `.msi`, o `-portable.zip` e o `SHA256SUMS.txt`. Notas em português.
5. Os builds não têm assinatura Authenticode, então o próprio IrisGrid só avisa da versão nova e abre a página da release; a instalação automática acontece pelo Workspace do Iris.

IrisGrid is a clean-room, open-source, Rust-native spreadsheet targeting Microsoft Excel parity — and superiority (speed, openness, agent control). It runs natively on macOS, Windows, Linux and BSD, and on the web via WASM. Siblings with the same conventions: `../designcraft` (InDesign-class, the closest reference), `../photocraft` (Photoshop), `../vectorcraft` (Illustrator), `../pdfcraft` (Acrobat), `../filmcraft` (Premiere), `../lightcraft` (Lightroom), `../effectcraft` (After Effects).

## Start every session here
1. Read `plan/STATUS.md` (current milestone, next task), then the task in `plan/execution-plan.md` and the relevant `plan/architecture.md` section. Behaviour reference: `plan/excel/*.md` (`observed-ui.md` holds measured observations of the running app).
2. Follow the autonomous operation protocol (`plan/execution-plan.md` §7). Don't stop to ask unless §7 lists the decision as the user's.

`plan/` is gitignored (local only).

## Assets and iconography — absolute rule
**No Microsoft, Adobe, Avid, Autodesk (or any other vendor's) iconography, images, artwork, fonts, templates, themes or sample files — ever.** This is the most important rule in this repository; breaking it is a serious failure.
- Every asset in the repo is original work by GridCraft contributors, or public domain / CC0, or open source / Creative Commons that allows redistribution (or licensed open source by the contributor who created it).
- Every asset file has a row in [`ATTRIBUTION.md`](ATTRIBUTION.md) (author, source, licence). `cargo xtask assets` enforces it. Adding an asset without a row is a bug.
- Prefer art generated in code: the UI icon set (`crates/ui-egui/src/icons.rs`), table and cell style palettes, themes and sample workbooks are drawn/defined in code and original.
- Never copy Office's icons, ribbon artwork, theme XML, table-style definitions, templates, clip art or wording beyond feature names. Excel feature *names* (menu labels, function names) are fine.
- Screenshots of Microsoft software are never committed (they live only in `plan/excel/screenshots/`).

## Never crash
People trust IrisGrid with their numbers; a crash loses their work. **This outranks feature work.**
- **No panics in non-test code:** no `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`; no `unsafe` (`unsafe_code = "forbid"`).
- **Errors are `Result<T, E>`** through the crate's error type and `?`.
- **Input-derived numbers are hostile** (files, formulas, commands, MCP/control params): `get()` instead of indexing, checked/saturating arithmetic, cap input-sized allocations (huge ranges, `REPT`, `SEQUENCE`, whole-column references).
- **Bound recursion** (formula nesting, dependency chains, names referring to names).
- **Last-resort guard:** `Session::execute` catches escaped panics and restores the workbook. It's a safety net, not a licence.
- **Prove it:** every crash fix lands with a regression test (`tests::every_command_survives_empty_params` fuzzes every command; `crates/engine/tests/it/command_sequence_fuzz.rs` fuzzes random command sequences against undo/redo, XLSX round-trip and full-recalc invariants; soak it with `GRIDCRAFT_FUZZ_SEEDS`/`GRIDCRAFT_FUZZ_STEPS`, replay with `GRIDCRAFT_FUZZ_SEED`).
- Production crate roots carry `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`; every new workspace crate must also include `[lints] workspace = true` in `Cargo.toml` to inherit `[workspace.lints]`.

## Non-negotiables
- **Clean-room.** Microsoft Excel is installed on the dev machine and may be *observed* black-box: run it, use its UI with synthetic workbooks, take screenshots (by window id) stored only under `plan/excel/screenshots/`. Never read, disassemble or copy anything inside the Office bundle, never commit files produced by Excel, never copy GPL/AGPL/LGPL code (LibreOffice, Gnumeric…). Function behaviour comes from public documentation and observation.
- **Fonts live in [`storytold/craft-fonts`](https://github.com/storytold/craft-fonts)**, never in this repo. It is an optional build input (`CRAFT_FONTS_DIR`); without it the app uses system fonts and egui's defaults.
- **Test corpora** (real Excel-authored files) live in a separate corpus repo, fetched pinned and sha256-verified. Never commit large binary fixtures; build test workbooks in code.
- **Everything is a command.** User-visible behaviour = a command in `crates/engine/src/cmd/*` (id, label, ribbon path, shortcut, params doc, `enabled`, `run`) + tests. Ids follow Excel's ribbon (`home.bold`, `insert.chart`, `data.sortAscending`) plus primitives (`cell.set`, `selection.set`, `range.setValues`). UI-only commands live in `crates/ui-egui`. The control channel, CLI and MCP reach all of them.
- **Programmatic calls never open dialogs**: a command with empty params runs with defaults or returns an error; only menu-style invocation opens a dialog (`UiRequest::Dialog`).
- **Layering** is enforced by `cargo xtask layers`: `core`/`numfmt` (L0) → `formula`/`functions` (L1) → `model`/`calc`/`xlsx` (L2) → `chart` (L3) → `engine` (L5) → `ui-egui`/`mcp` (L6) → `apps/*` (L7). Nothing below L6 depends on egui/eframe/winit/rfd. **The UI crate is swappable.**
- **The UI is thin**: panels read engine state and act through `app.run(id, params)`. Colours come from `theme::Tokens`.
- **Rust only** (no handwritten JS/TS). **Never break wasm** (`cargo xtask wasm`).
- **Quality gates** before every commit: `cargo xtask ci` (fmt, clippy -D warnings, tests, assets, layers, wasm). One task id per commit (`M2.1: borders gallery`).

## Running and looking at the app
- `cargo run --release -p gridcraft -- --sample sales --control 7979` (sample workbook + control channel).
- Drive it: `gridcraft-cli send 7979 <method> '<json>'` (reads the token file the app printed on stderr), or JSON lines on `127.0.0.1:7979` whose first line is `{"id":0,"method":"auth","params":{"token":"…"}}` (token: `$XDG_RUNTIME_DIR/gridcraft/control-7979.token`, or your own `GRIDCRAFT_CONTROL_TOKEN`), e.g. `{"id":1,"method":"engine.execute","params":{"command":"cell.set","params":{"cell":"B2","input":"=SUM(A1:A9)"}}}` then `{"id":2,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}`. Methods: `crates/ui-egui/src/control.rs`, docs: `docs/control-protocol.md`.
- **Offscreen UI render** (no window, from a source checkout): `cargo run -p gridcraft-ui-egui --example snapshot -- --sample sales out.png`.
- Headless: `gridcraft-cli eval '=SUM(1,2,3)'`, `gridcraft-cli run --in book.xlsx --cmd 'home.bold={"range":"A1:C1"}' --out book.xlsx`, `gridcraft-cli mcp`.
- **For UI work, look at the result** (snapshot PNG) and compare with `plan/excel/observed-ui.md`.
- Shell gotcha: `mv`/`cp` are aliased interactive here — use `/bin/mv -f` / `/bin/cp -f`. macOS has no `timeout`; use `perl -e 'alarm 60; exec @ARGV' cmd`.
- Parallel agents: separate `CARGO_TARGET_DIR` per agent; edit only the crates you own; delete your target dir when done.

## Roadmap
`ROADMAP.md` (committed) is the one-page summary: stage, headline numbers, dimensions, languages, progress log. It points at the detail in `docs/`: `target-app-parity.md` (authoritative assessment), `gaps.md` (ranked work list: pick from it), `roadmap.md` (current focus, beta gates, milestones), `architecture.md`, and the area checklists `function-parity.md`, `chart-parity.md`, `file-format-parity.md`, `ui-parity.md`, `hardware-parity.md`, `localization-parity.md`. Update the affected docs (and their timestamp lines and revision history) whenever work lands. `cargo xtask parity` regenerates `docs/parity-checklist.md` from `crates/engine/src/catalog.rs`.

## CI workflows
- Workflows in `.github/workflows/` call the shared workflows in [`storytold/craft-common-actions`](https://github.com/storytold/craft-common-actions) at `@main`, with their inputs and secrets. No other jobs: if no shared workflow does what's needed, it doesn't go here.
- Don't add or change CI in a feature or fix PR. If the existing workflows can't build or test a change, link an existing craft-common-actions issue or PR, or open a new issue, or a PR only for a small fix that is safe for every app and does not increase CI time.
- Some workflow files here still run their own jobs. Don't add to them. Moving a job into craft-common-actions is its own task, never part of another PR, and the job stays here until a shared workflow runs it.
