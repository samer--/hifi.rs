<script>
	import { afterUpdate } from 'svelte';
	import IconButton from './IconButton.svelte';
	import { currentStatus, volume } from '$lib/websocket';

	export let showList, toggleList, controls, navHeight;

	let height;

	afterUpdate(() => {
		navHeight.set(height);
	});
</script>

<div
	bind:offsetHeight={height}
	class="flex flex-row gap-x-4 md:mt-0 px-4 md:px-12 items-end justify-between"
>
	<span class:fixed-button={$showList}>
		<IconButton onClick={toggleList} label={$showList ? 'Close' : 'Menu'}>
			{#if $showList}
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<line x1="18" x2="6" y1="6" y2="18" />
					<line x1="6" x2="18" y1="6" y2="18" />
				</svg>
			{:else}
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<line x1="4" x2="20" y1="6" y2="6" />
					<line x1="4" x2="20" y1="12" y2="12" />
					<line x1="4" x2="20" y1="18" y2="18" />
				</svg>
			{/if}
		</IconButton>
	</span>
	<div class="flex flex-row justify-end gap-x-4 flex-grow">
		<IconButton onClick={() => controls?.previous()} label="Previous">
			<svg
				class="w-6 h-6 lg:w-7 lg:h-7"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<polygon points="19 20 9 12 19 4 19 20" />
				<line x1="5" x2="5" y1="19" y2="5" />
			</svg>
		</IconButton>
		<IconButton
			onClick={() => controls?.playPause()}
			label={$currentStatus === 'Playing' ? 'Pause' : 'Play'}
		>
			{#if $currentStatus === 'Playing'}
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<rect width="4" height="16" x="6" y="4" />
					<rect width="4" height="16" x="14" y="4" />
				</svg>
			{:else}
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polygon points="6 3 20 12 6 21 6 3" />
				</svg>
			{/if}
		</IconButton>
		<IconButton onClick={() => controls?.next()} label="Next">
			<svg
				class="w-6 h-6 lg:w-7 lg:h-7"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<polygon points="5 4 15 12 5 20 5 4" />
				<line x1="19" x2="19" y1="5" y2="19" />
			</svg>
		</IconButton>
		<div class="flex flex-row items-center gap-x-2">
			<IconButton onClick={() => controls?.volumeDown()} label="Decrease volume">
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
					<path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
				</svg>
			</IconButton>
			<span class="text-2xl xl:text-3xl">{Math.round($volume * 100)}%</span>
			<IconButton onClick={() => controls?.volumeUp()} label="Increase volume">
				<svg
					class="w-6 h-6 lg:w-7 lg:h-7"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
					<path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
					<path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
				</svg>
			</IconButton>
		</div>
	</div>
</div>

<style lang="postcss">
	.fixed-button {
		@apply fixed z-20 bottom-4 left-4 md:z-auto md:relative md:bottom-auto md:left-auto;
	}
</style>
