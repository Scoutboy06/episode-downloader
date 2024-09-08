import getEpisodeDownloadUrl from '@/server/getEpisodeDownloadLink';
import { error, json, type RequestHandler } from '@sveltejs/kit';
import { createWriteStream } from 'node:fs';
import path from 'node:path';
import http from 'node:http';

/**
 * /download?id=...&fileName=...&dir=D:\JellyfinMedia\Series
 */
export const GET: RequestHandler = async ({ url }) => {
	const id = url.searchParams.get('id');
	const fileName = url.searchParams.get('fileName');
	const dir = url.searchParams.get('dir');

	if (!id) error(400, "Missing 'id' query parameter");
	if (!fileName) error(400, "Missing 'fileName' query parameter");
	if (!dir) error(400, "Missing 'dir' query parameter");

	const downloadLink = await getEpisodeDownloadUrl(id);
	console.log(downloadLink);

	if (!downloadLink) error(404, 'Could not find download link');

	const dest = path.join(dir, fileName);
	const file = createWriteStream(dest);

	return new Promise((resolve, reject) => {
		http
			.get(downloadLink, (res) => {
				console.log('Started download...');
				res.pipe(file);

				file.on('finish', () => {
					console.log('Download finished!');
					resolve(json({ ok: true, id, fileName, dir }));
				});
			})
			.on('error', () => reject(json({ ok: false })));
	});
};
