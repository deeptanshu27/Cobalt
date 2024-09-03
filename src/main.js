const { invoke } = window.__TAURI__.tauri;

async function run_(loc) {
  await invoke("run", { name: loc })
}

async function settings() {
  tauri_.path.resolveResource("resources/games.json")
    .then((resourcePath) => {
      invoke("settings", { name: resourcePath })
      .then((res) => {})
    })
}