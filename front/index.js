import { Universe } from "game-of-life";

const pre = document.getElementById("canvas");
const universe = Universe.new();

pre.textContent = universe.render();
