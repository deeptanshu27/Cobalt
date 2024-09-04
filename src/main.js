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

async function add()  {
  let name = prompt("Name of game");
  let id = name.replace(/[^a-zA-Z0-9]/g, "")

  let url = prompt("Image URL");

  tauri_.dialog.open()
  .then((res) => {
    tauri_.path.resolveResource("resources/games.json")
    .then((resourcePath) => {
      invoke("add", { loc: resourcePath, id: id, name: name, img: url, shortcut: res })
      .then((res_) => {})
    })
  })
}