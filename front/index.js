const myWorker = new Worker('worker.js', {type: 'module'});

console.log('passing message to the worker..');
myWorker.postMessage('hello to the worker');

let i = 0;
console.time("cells2");
myWorker.onmessage = function (event) {
  const bar = new Uint8Array(event.data);
  console.log(bar.length);
  i++;
  if (i==1000) {
    console.timeEnd("cells2");
  }
};
