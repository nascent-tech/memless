// Guards GHSA-2m5v-pch6-74hx: undici's Agent used to JSON-clone its `tls`
// options, which dropped `checkServerIdentity` and turned a `ca` Buffer into a
// plain object. undici reaches the site through unifont (remote fonts at build).
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { createServer } from 'node:https';
import { createRequire } from 'node:module';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { after, before, describe, it } from 'node:test';

const astroRequire = createRequire(createRequire(import.meta.url).resolve('astro/package.json'));
const unifontRequire = createRequire(astroRequire.resolve('unifont'));
const { Agent, request } = unifontRequire('undici');

function createSelfSignedCertificate(directory) {
	const certificatePath = join(directory, 'certificate.pem');
	const privateKeyPath = join(directory, 'private-key.pem');

	execFileSync('openssl', [
		'req', '-x509', '-nodes', '-days', '1',
		'-newkey', 'ec', '-pkeyopt', 'ec_paramgen_curve:prime256v1',
		'-subj', '/CN=localhost', '-addext', 'subjectAltName=IP:127.0.0.1',
		'-keyout', privateKeyPath, '-out', certificatePath,
	], { stdio: 'ignore' });

	return { cert: readFileSync(certificatePath), key: readFileSync(privateKeyPath) };
}

describe('the undici copy used by unifont', () => {
	let directory;
	let certificate;
	let server;
	let origin;

	before(async () => {
		directory = mkdtempSync(join(tmpdir(), 'undici-tls-'));
		certificate = createSelfSignedCertificate(directory);
		server = createServer(certificate, (_request, response) => response.end('ok'));
		await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
		origin = `https://127.0.0.1:${server.address().port}`;
	});

	after(() => {
		server.close();
		rmSync(directory, { recursive: true, force: true });
	});

	it('trusts a server whose certificate authority is given as a Buffer', async () => {
		const dispatcher = new Agent({ tls: { ca: certificate.cert } });

		const response = await request(origin, { dispatcher });

		assert.equal(response.statusCode, 200);
		await response.body.text();
		await dispatcher.close();
	});

	it('rejects a server that the custom identity check refuses', async () => {
		const refuseEveryServer = () => new Error('server identity refused');
		const dispatcher = new Agent({
			tls: { ca: certificate.cert.toString(), checkServerIdentity: refuseEveryServer },
		});

		await assert.rejects(request(origin, { dispatcher }), /server identity refused/);
		await dispatcher.close();
	});
});
