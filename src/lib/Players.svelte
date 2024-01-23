<script>
    import { fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

    export let players = [];

    const max_players = 10;

    let input;

    const add_player = () => {
        if(input == "" || input == undefined || players.length >= max_players){return;}

        players.push(input);
        
        // Hack to force svelte to redraw
        players = players;

        input = "";
    }
</script>

<div class="player-control-container">

    <div class="add-players-box"
        in:fly={{ delay: 300, duration: 300, x: 0, y: 400, opacity: 0, easing: quintOut }}
    >
        <input type="text" bind:value={input} placeholder="Add a player">
        <button on:click={add_player}>Add</button>
    </div>
    <div class="player-list">
        {#each players as player, i }
            <div class="list-player"
                in:fly={{ delay: 400 + (25 * i), duration: 300, x: 0, y: 150, opacity: 0, easing: quintOut }}
            >   <div class="list-player-name">
                    {player}
                </div>
                <h3 on:click={() => {players.splice(i, 1); players = players;}}>Remove</h3>
            </div>
        {/each}
    </div>
</div>