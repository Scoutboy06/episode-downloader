<script lang="ts">
	import { FilePathPicker } from '@/components/FolderPicker';
	import { Search } from '@/components/Search';
	import { Button } from '@/components/ui/button';
	import Home from 'lucide-svelte/icons/house';
</script>

<div class="dark">
	<header class="flex flex-row gap-2 p-2 align-middle">
		<Button href="/" variant="ghost">
			<Home />
		</Button>

		<Search />
	</header>

	<main class="mx-2 mt-6">
		<!-- <FilePathPicker folderStore={seriesFolderStore} /> -->
	</main>
</div>
