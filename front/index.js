const myWorker = new Worker('worker.js', {type: 'module'});

console.log('passing message to the worker..');
myWorker.postMessage('hello to the worker');

myWorker.onmessage = function (event) {
  const bar = new Uint8Array(event.data);
  console.log(bar.length);
};
