let backend = globalThis.backend;
import {startGame} from "./game.js";

let chosen_file;
let play_button;
let rle_parser = new backend.RleParser();

function setLoading() {
    document.getElementById("loading").hidden = false;
    document.getElementById("menu").hidden = true;
    document.getElementById("canvas").hidden = true; 
    document.getElementById("game-menu").hidden = true; 
}

function onPlay() {
    setLoading();

    let reader = new FileReader();
    reader.addEventListener("load", ()=>{
        rle_parser.read(reader.result);
        startGame(rle_parser.result());
    })
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

function main() {
    play_button = document.getElementById('play-button');
    play_button.disabled = true;
    play_button.addEventListener("click", onPlay);

    let file_chooser = document.getElementById('file-input');
    file_chooser.addEventListener("change", e => onInputChange(e));
}

main();
