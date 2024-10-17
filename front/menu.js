import { response as indexResponse } from "./index.js";

let chosen_file;
let play_button;

function setLoading() {
    document.getElementById("loading").hidden = false;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = true; 
    document.getElementById("game-menu").hidden = true; 
}

function onPlay() {
    setLoading();

    let reader = new FileReader();
    reader.addEventListener("load", () => {
        indexResponse({res: "readFileOk", s: reader.result});
    });

    reader.readAsText(chosen_file);
}

function onInputChange(e) {
    let files = e.target.files;
    if (files.length == 0) {
        play_button.disabled = true;
    } else {
        play_button.disabled = false;
        chosen_file = files[0];
    }
}

function showMenu() {
    document.getElementById("loading").hidden = true;
    document.getElementById("menu").hidden = false;
    document.getElementById("canvas").hidden = false;

    play_button = document.getElementById('play-button');
    play_button.disabled = true;
    play_button.addEventListener("click", onPlay);

    let file_chooser = document.getElementById('file-input');
    file_chooser.addEventListener("change", e => onInputChange(e));
}

export {showMenu};