<script lang="ts">
	import { onMount } from 'svelte';

	let data: {} | null = null;

	onMount(async () => {
		const data = await fetch('https://dummyjson.com/test').then((res) => res.json());
		console.log(data);
	});
</script>

{#if !data}
	<!-- content here -->
{/if}
