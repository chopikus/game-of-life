//import {startGame} from "./game.js";

function main() {
    console.log(backend);
    startGame();
}

var backend = {
      onRuntimeInitialized: function() {
        console.log('lerp result: ' + Module.lerp(1, 2, 0.5));
      }
};

