
<script lang="ts">

	let ingredients: Ingredient[] =
		[
			{name: "carrot"},
			{name: "orange"},
			{name: "paprika"},
		];

	function click() {
		ingredients.push({name: "steak"})
	}


</script>

<h1>
	{ingredients} welcome
</h1>
<button id="see-ingr" onclick={click}> See ingredients </button>