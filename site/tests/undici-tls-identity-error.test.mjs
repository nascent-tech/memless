// GHSA-qrxm-fqhp-jwvm: a TLS identity error raised on an open connection (as a
// certificate-changing renegotiation does) hit an internal assert in undici and
// crashed the whole process. The undici checked here is the one astro's font
// provider (unifont) loads during the build.
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { createRequire } from 'node:module';
import { createServer } from 'node:net';
import { describe, it } from 'node:test';

const requireFromUnifont = createRequire(new URL('../node_modules/unifont/', import.meta.url));
const undiciPath = requireFromUnifont.resolve('undici');

// Runs in a child process because the vulnerable versions throw an uncaught
// assertion that would take the test runner down with it.
const requestThroughFailingSocket = `
const net = require('node:net');
const { Client } = require(process.argv[1]);
const port = Number(process.argv[2]);

const client = new Client('http://127.0.0.1:' + port, {
	connect(options, callback) {
		const socket = net.connect(port, '127.0.0.1', () => {
			callback(null, socket);
			setTimeout(() => {
				const error = new Error('Hostname/IP does not match certificate altnames');
				error.code = 'ERR_TLS_CERT_ALTNAME_INVALID';
				socket.destroy(error);
			}, 50);
		});
	},
});

client
	.request({ path: '/', method: 'GET' })
	.then(() => console.log('unexpected-response'), (error) => console.log('rejected:' + error.code))
	.finally(() => client.close());
`;

describe('the undici used by the site build', () => {
	it('rejects the request instead of crashing when a connection fails its tls identity check', async () => {
		const silentServer = createServer(() => {});
		silentServer.listen(0, '127.0.0.1');
		await once(silentServer, 'listening');

		const child = await runInChild(undiciPath, silentServer.address().port);
		silentServer.close();

		assert.equal(child.status, 0, child.stderr);
		assert.equal(child.stdout.trim(), 'rejected:ERR_TLS_CERT_ALTNAME_INVALID');
	});
});

async function runInChild(modulePath, port) {
	const child = spawn(process.execPath, ['-e', requestThroughFailingSocket, modulePath, String(port)], {
		timeout: 10_000,
	});
	let stdout = '';
	let stderr = '';
	child.stdout.on('data', (chunk) => (stdout += chunk));
	child.stderr.on('data', (chunk) => (stderr += chunk));
	const [status] = await once(child, 'close');

	return { status, stdout, stderr };
}
