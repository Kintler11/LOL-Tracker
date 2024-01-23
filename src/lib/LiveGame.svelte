<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from "svelte";
    import PlayerInfo from "./PlayerInfo.svelte";
    import Popup from "./Popup.svelte";
    import { fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

    export let players = []
    export let events = []
    export let event_buffer = []

</script>
<div class="popup-container">
    {#each event_buffer as event }
        <Popup bind:event={event} />
    {/each}
</div>
<div class="live-game-tab" 
    out:fly={{ delay: 0, duration: 300, x: 0, y: 800, opacity: 0.2, easing: quintOut }}
>

    <div class="team-side" side="blue" on:mousedown={()=>events.pop()}
        in:fly={{ delay: 300, duration: 300, x: -1200, y: 0, opacity: 0, easing: quintOut }}
    >
        {#each players.filter(pl => pl.team == "Blue") as player }
            <PlayerInfo
                player_data={player}
            />
        {/each}
    </div>
    <div class="team-side" side="red"
        in:fly={{ delay: 300, duration: 300, x: 800, y: 0, opacity: 0, easing: quintOut }}
    >
        {#each players.filter(pl => pl.team == "Red") as player }
            <PlayerInfo 
                player_data={player}
            />
        {/each}
    </div>
    {#if players.length == 0}
        <div class="live-game-placeholder"
        >
            No Game Found
        </div>
    {/if}
</div>