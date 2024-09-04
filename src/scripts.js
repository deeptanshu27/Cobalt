const tauri_ = window.__TAURI__

let jsonData = null;

tauri_.path.resolveResource("resources/games.json")
.then((resourcePath) => {
    tauri_.fs.readTextFile(resourcePath)
    .then((data) => {
        data = JSON.parse(data);
        jsonData = data;
        for (let key in data) {
            addGame(key, data[key]);
        }
    })
})

function addGame(id, { name, img }) {
    let container = document.getElementById("container");
    let gameInfo = document.createElement("div");
    gameInfo.setAttribute("onclick", `load(${id})`);
    gameInfo.className = "game-info";
    let button = document.createElement("button");
    button.id = id;
    let img_ = document.createElement("img");
    img_.setAttribute("src", img);
    button.appendChild(img_);
    gameInfo.appendChild(button);
    let h3 = document.createElement("h3");
    h3.innerText = name;
    gameInfo.appendChild(h3);
    container.appendChild(gameInfo);
}

function load(id) {
    document.getElementById("selected-game").innerText = jsonData[id.id]["name"];
    document.getElementById("selected-game").className = id.id;
    // console.log(jsonData)
    console.log(jsonData[id.id]["name"]);
}

function run() {
    run_(jsonData[document.getElementById("selected-game").className]["loc"]);
}