import { persistent } from '@furudean/svelte-persistent-store';

const seriesFolderPath = persistent({
	start_value: '',
	key: 'seriesFolderPath',
	storage_type: 'localStorage',
	serialize: (val) => val,
	deserialize: (val) => val
});

export default seriesFolderPath;
