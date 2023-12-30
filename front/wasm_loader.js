var backend = {
      onRuntimeInitialized: function() {
        globalThis.backend = backend;
        var index = document.createElement("script");
        index.type = "module";
        index.src = "index.js";
        document.body.appendChild(index);

        document.getElementById("loading").hidden = true;
        document.getElementById("menu").hidden = false;
        document.getElementById("canvas").hidden = false; 
      }
};

