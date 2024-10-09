const myWorker = new Worker('worker.js');

myWorker.postMessage('hello to the worker');

myWorker.onmessage = function (event) {
  console.log('Received message from worker:', event.data);
};
