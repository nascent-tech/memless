// undici is pulled in by astro's font downloader (unifont). GHSA-xjj6-f33q-929c:
// a response with too many content-encodings must reject the fetch, not crash
// the build process with an unhandled error.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { once } from "node:events";
import { createServer } from "node:net";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const siteRoot = fileURLToPath(new URL("..", import.meta.url));

const fetchFromChildProcess = `
  import { fetch } from "undici";
  try {
    await fetch(process.argv[1]);
    console.log("resolved");
  } catch (error) {
    console.log("rejected: " + (error.cause?.message ?? error.message));
  }
  setTimeout(() => {}, 200);
`;

function startServerSendingTooManyEncodingsThenDropping() {
  const server = createServer((socket) => {
    socket.once("data", () => {
      socket.write(
        "HTTP/1.1 200 OK\r\n" +
          "Content-Encoding: gzip, gzip, gzip, gzip, gzip, gzip\r\n" +
          "Content-Length: 100\r\n\r\n",
      );
      setTimeout(() => socket.destroy(), 50);
    });
  });
  server.listen(0, "127.0.0.1");
  return server;
}

function runNode(args) {
  return new Promise((resolve) => {
    execFile(process.execPath, args, { cwd: siteRoot }, (error, stdout, stderr) =>
      resolve({ exitCode: error ? error.code : 0, stdout, stderr }),
    );
  });
}

test("a response with too many content-encodings rejects the fetch without crashing", async () => {
  const server = startServerSendingTooManyEncodingsThenDropping();
  await once(server, "listening");
  const { port } = server.address();

  const result = await runNode([
    "--input-type=module",
    "--eval",
    fetchFromChildProcess,
    `http://127.0.0.1:${port}/font.woff2`,
  ]);
  server.close();

  assert.equal(result.exitCode, 0, result.stderr);
  assert.match(result.stdout, /^rejected: too many content-encodings/m);
});
