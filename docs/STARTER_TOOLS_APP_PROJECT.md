# Setup a Leptos Starter Project

1. Open a terminal window, and change in to the `projects` folder.

```bash
cd projects
```

2. Within the `projects` folder, run the following command.

```bash
cargo leptos new --git leptos-rs/start
```

3. The setup tool will ask for a project name, type `tools-app`.

4. Add the project path `projects/tools-app` to the `members` list in the root folder `Cargo.toml` file.

5. Change into the `tools-app` folder. Type the following command to run the project in watch mode.

```bash
cargo leptos watch
```

6. Once compilation is complete, open a web browser, and navigate to `http://localhost:3000`.
