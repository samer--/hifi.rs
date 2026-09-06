<script>
	export let onClick = null;
	export let label = '';
</script>

<button
	aria-label={label}
	title={label}
	class="bg-amber-900 hover:bg-blue-800 text-gray-100 leading-none p-2 lg:p-3 border-r-4 border-b-4 border-blue-800"
	on:click|stopPropagation={onClick}
>
	<slot />
</button>
