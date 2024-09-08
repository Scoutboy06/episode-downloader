<script lang="ts">
	import { FilePathPicker } from '@/components/FolderPicker';
	import seriesFolderPath from '@/stores/seriesFolderPath';
</script>

<main class="mx-2 mt-6">
	<FilePathPicker folderStore={seriesFolderPath} />
</main>
