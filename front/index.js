import { Universe } from "game-of-life";

const pre = document.getElementById("canvas");
const universe = Universe.new();

var fps = 2;
var now;
var then = Date.now();
var interval = 1000/fps;
var delta;
  
function draw() {
     
    requestAnimationFrame(draw);
     
    now = Date.now();
    delta = now - then;
     
    if (delta > interval) {
        // update time stuffs
         
        // Just `then = now` is not enough.
        // Lets say we set fps at 10 which means
        // each frame must take 100ms
        // Now frame executes in 16ms (60fps) so
        // the loop iterates 7 times (16*7 = 112ms) until
        // delta > interval === true
        // Eventually this lowers down the FPS as
        // 112*10 = 1120ms (NOT 1000ms).
        // So we have to get rid of that extra 12ms
        // by subtracting delta (112) % interval (100).
        // Hope that makes sense.
        
        then = now - (delta % interval);
         
	pre.textContent = universe.render();
	universe.tick();	
    }
}
 
draw();
