const http = require("http");

let count = 0;

const makeRequest = () => {
  count++;
  return http.request(
    {
      hostname: "localhost",
      port: 8080,
      path: "/spotify/currently-playing",
      method: "GET",
    },
    (res) => {
      console.log(`statusCode: ${res.url}`);

      res.on("data", (d) => {
        process.stdout.write(d);
      });
    }
  );
};

const test = () => {
  console.log(count);
  makeRequest();
  setTimeout(test, 1000);
};

test();
