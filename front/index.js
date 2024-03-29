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
    let file = chosen_file;
    let r = new FileReader();
    const chunk_size = 1024 * 1024;
    let blobs = [];
    for (let i = 0; i < Math.ceil(file.size / chunk_size); i+=1) {
        let start = i * chunk_size;
        let end = Math.min((i+1) * chunk_size, file.size);
        blobs.push(file.slice(start, end));
    }
    
    function readBlob(i) {
        if (i >= blobs.length) {
            startGame(rle_parser.result());
            return;
        }
        const reader = new FileReader();
        reader.addEventListener("load", () => {
            rle_parser.add_chunk(reader.result);
            readBlob(i+1);
        });
        reader.readAsText(blobs[i]);
    }
    readBlob(0);
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
