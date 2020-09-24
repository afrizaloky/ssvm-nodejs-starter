const { say, is_prime } = require('../pkg/ssvm_nodejs_starter_lib.js');

const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  if (!queryObject['number']) {
    res.end(`Please use command curl http://${hostname}:${port}/?number=number \n`);
  } else {
    res.end(is_prime(parseInt(queryObject['number'])) + '\n');
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
