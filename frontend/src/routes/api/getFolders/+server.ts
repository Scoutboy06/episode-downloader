import { error, json, type RequestHandler } from '@sveltejs/kit';
import fs from 'node:fs';
import { exec } from 'node:child_process';

export const GET: RequestHandler = async ({ url }) => {
	const path = url.searchParams.get('path');

	if (!path) {
		const drives = await listDrives();
		return json(
			drives.map((drive) => ({
				name: drive,
				parentPath: null,
				path: ''
			}))
		);
	}

	try {
		const files = fs
			.readdirSync(path, { withFileTypes: true })
			.filter((entry) => entry.isDirectory());
		return json(files);
	} catch {
		error(404, 'Folder not found');
	}
};

const driveRegex = /[A-Za-z]:/;
function listDrives(): Promise<string[]> {
	return new Promise((resolve, reject) => {
		exec('wmic logicaldisk get name', (error, stdout) => {
			if (error) reject(error);
			else
				resolve(
					stdout
						.split('\r\r\n')
						.filter((val) => driveRegex.test(val))
						.map((val) => val.trim())
				);
		});
	});
}
