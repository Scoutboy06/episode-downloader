<script lang="ts">
	import { Input } from '@/components/ui/input';
	import Plus from 'lucide-svelte/icons/plus';
	import Search from 'lucide-svelte/icons/search';
	import type { Writable } from 'svelte/store';
	import * as Dialog from '../ui/dialog';
	import Button from '../ui/button/button.svelte';
	import axios from 'axios';
	import { cn } from '@/utils';
	import { ScrollArea } from '../ui/scroll-area';

	let folderPath = '';
	let selectedFolderPath = '';
	let folders: Dirent[] | null = null;

	$: dialogButtonsDisabled = folders === null || !folderPath;

	export let folderStore: Writable<any>;
	folderStore.subscribe((newVal) => {
		folderPath = newVal;
		selectedFolderPath = newVal;
	});

	function updateFolders(path: string) {
		axios
			.get(`/api/getFolders?path=${path}`)
			.then((res) => {
				folders = res.data;
			})
			.catch((err) => {
				console.error(err);
				folders = [];
			});
	}

	function handleFolderClick(newPath: string) {
		updateFolders(newPath);
		folderPath = newPath;
	}

	function handleInputChange(e: any) {
		updateFolders(e.target.value);
		folderPath = e.target.value;
	}

	function handleSubmit() {
		folderStore.set(folderPath);
	}

	function getFolderLink(folder: Dirent): string {
		let link = '';

		if (folder.parentPath) {
			link += folder.parentPath;
			if (folder.parentPath.at(-1) !== '\\') link += '\\';
		}

		link += folder.name;

		link += '\\';

		return link;
	}

	function getBackFolderLink(): string {
		const i = folderPath.lastIndexOf('\\', folderPath.length - 2);
		if (i === -1) return '';
		return folderPath.slice(0, i + 1);
	}
</script>

<Dialog.Root
	onOpenChange={(isOpen) => {
		isOpen && updateFolders(folderPath);
	}}
>
	<Dialog.Trigger class="flex cursor-pointer items-center gap-2">
		<Input
			on:input={handleInputChange}
			value={selectedFolderPath}
			placeholder="Select a folder"
			readonly
			class="w-64 cursor-pointer"
		/>

		<Search />
	</Dialog.Trigger>

	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Select a folder</Dialog.Title>
			<Dialog.Description>
				Browse or enter the path to select. The folder must be writeable.
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-4 py-4">
			<Input bind:value={folderPath} on:input={handleInputChange} placeholder="Select a folder" />
		</div>

		<ScrollArea class="mb-4 max-h-64">
			<div class="flex flex-col">
				{#if folders !== null}
					{#if folderPath !== ''}
						<Button
							variant="outline"
							on:click={() => handleFolderClick(getBackFolderLink())}
							class={'justify-start rounded-b-none'}>...</Button
						>
					{/if}

					{#each folders as folder, i (folder.parentPath + folder.name)}
						<Button
							variant="outline"
							on:click={() => handleFolderClick(getFolderLink(folder))}
							class={cn(
								'justify-start',
								(folderPath !== '' || i > 0) && 'rounded-t-none border-t-0',
								i < folders.length - 1 && 'rounded-b-none'
							)}>{folder.name}</Button
						>
					{:else}
						<Button variant="outline" class="mt-2 border-0 rounded-t-none" disabled
							>Empty folder</Button
						>
					{/each}
				{/if}
			</div>
		</ScrollArea>

		<div class="flex justify-between">
			<Button variant="outline" class="gap-1" disabled={dialogButtonsDisabled}>
				<Plus />
				<span>Create new folder</span>
			</Button>
			<Dialog.Close disabled={dialogButtonsDisabled}>
				<Button variant="default" on:click={handleSubmit} disabled={dialogButtonsDisabled}
					>Select folder</Button
				>
			</Dialog.Close>
		</div>
	</Dialog.Content>
</Dialog.Root>
