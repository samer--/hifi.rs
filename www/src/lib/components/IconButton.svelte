<script>
	export let onClick = null;
	export let label = '';
</script>

<button
	aria-label={label}
	title={label}
	class="bg-neutral-900 hover:bg-blue-800 text-gray-100 leading-none p-3 lg:p-4 border-3 border-blue-800"
	on:click|stopPropagation={onClick}
>
	<slot />
</button>
