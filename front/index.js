const myWorker = new Worker('worker.js', {type: 'module'});

const gliderText = `# comment1
# comment2
x = 10, y = 10
# comment3
bo$2bo$3o!

# is the same as

# bob
# bbo
# ooo
# (1,0), (2,1), (0,2), (1,2), (2,2)`;


console.log("hello");
myWorker.postMessage({req: "init"});

myWorker.onmessage = function (event) {
  console.log(event.data);
  switch (event.data.res) {
    case "cellOk": {
      console.log("cell ok!!");
      const bar = new BigInt64Array(event.data.buf);
      console.log(bar);
      break;
    }
    default: {
      break;
    }
  }
};
