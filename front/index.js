import {startGame} from "./game.js";

let chosen_file;
let play_button;

function onPlay() {
    let file = chosen_file;
    let r = new FileReader();
    const chunk_size = 8 * 1024 * 1024;
    let blobs = [];
    for (let i = 0; i < Math.ceil(file.size / chunk_size); i+=1) {
        let start = i * chunk_size;
        let end = Math.min((i+1) * chunk_size, file.size);
        blobs.push(file.slice(start, end));
    }
     
    var dest = new WritableStream({
      write (str) {
        console.log(str)
      }
    });

    let blob = blobs[0];
    (blob.stream ? blob.stream() : new Response(blob).body)
      // Decode the binary-encoded response to string
      .pipeThrough(new TextDecoderStream())
      .pipeTo(dest)
      .then(() => {
        console.log('done')
      })
    console.log(blobs);
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
