<script lang="ts">
	import { Input } from '../ui/input';
	import axios from 'axios';
	import * as Popover from '../ui/popover';
	import { Button } from '../ui/button';
	import debounce from '$lib/debounce';

	let searchText = '';
	let results: SearchResponse | null = null;
	let inputEl: Input;

	const handleUpdate = debounce((e: any) => {
		const q = e.target.value as string;

		if (!q) {
			results = [];
			return;
		}

		axios
			.get('/api/search?q=' + q)
			.then((res) => (results = res.data))
			.catch(console.error);
	}, 300);
</script>

<Popover.Root disableFocusTrap openFocus={inputEl}>
	<Popover.Trigger>
		<Input
			class="w-96 justify-start"
			placeholder="Search..."
			bind:value={searchText}
			on:input={handleUpdate}
			bind:this={inputEl}
		/>
	</Popover.Trigger>

	<Popover.Content class="flex flex-col">
		{#each results ?? [] as series, i}
			<Button
				variant="ghost"
				href={'/' + series.link}
				title={series.name}
				class={'h-12 w-full justify-start gap-2 px-2 py-1'}
			>
				<img src={series.img} alt={series.name} class="h-10" />

				<p class="w-full overflow-hidden text-ellipsis">{series.name}</p>
			</Button>
		{/each}
	</Popover.Content>
</Popover.Root>
