<script>
    import { fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
    import { invoke } from "@tauri-apps/api/tauri"

    import fill from "./images/fill.png";
    import top from "./images/top.png";
    import jg from "./images/jungle.png";
    import mid from "./images/mid.png";
    import bot from "./images/bot.png";
    import sup from "./images/support.png";
    
    export let players = [];

    const lane_img = [top,jg,mid,bot,sup];

    const lanes = ["Top", "Jungle", "Middle", "Bottom", "Support"];

    let teams = 0;
    let team_size = 5;

    export let team_1 = []
    export let team_2 = []

    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }

    function fill_players(array, count) {
        let filled_array = [...array];

        for(let i = 0; i < count; i++){
            filled_array.push("A player")
        }

        return filled_array;
    }

    const update_players = () => {
        
        let temp_players = [...players].sort( () => .5 - Math.random() );
        
        // Incase players dont split evenly, change "randomly" which team has an extra player
        let half = (getRandomInt(2) == 1) ? Math.ceil(players.length / 2) : Math.floor(players.length / 2);   
        
        team_1 = temp_players
        if(teams == 1){
            team_1 = temp_players.slice(0, half)
            team_2 = temp_players.slice(half)
        }

        // [p1, p2, p3] -> [p1, p2, p3, p?, p?] -> [p3, p1, p?, p2, p?]
        team_1 = fill_players(team_1, team_size - team_1.length).sort( () => .5 - Math.random() );
        team_2 = fill_players(team_2, team_size - team_2.length).sort( () => .5 - Math.random() );
    }

</script>
<div class="game-teams-container"
    out:fly={{ delay: 0, duration: 300, x: 0, y: 800, opacity: 0.2, easing: quintOut }}
>
    <div class="team-container"
        in:fly={{ delay: 300, duration: 300, x: -1200, y: 0, opacity: 0, easing: quintOut }}
    >
        {#each lanes as lane, i}
            <div class="player-wrapper" is-active="{i < team_size}">
                <div class="role-wrapper">
                    <img src={(team_size < 5) ? fill : lane_img[i]} alt="">
                </div>
                <div class="player-info-wrapper" on:load={() => console.log("reload")}>
                    {#if i < team_size && team_1[i] != undefined && team_1[i] != "A player"}
                        <h2 class="color-red">{team_1[i]}</h2>
                    {:else}
                        <h2>A player</h2>
                    {/if}
                    <h3>{ (team_size < 5) ? "Any" : lane}</h3>
                </div>
            </div>
        {/each}
    </div>
    <div class="team-settings"
        in:fly={{ delay: 300, duration: 300, x: 0, y: 1200, opacity: 0, easing: quintOut }}
    >
        <div class="team-setting">
            <div class="setting-name"> Team Size </div>
            <div class="team-slider">
                1
                <input type="range" bind:value={team_size} min="1" max="5" on:input={update_players} id="slider">
                5
            </div>
        </div>
        <div class="team-setting sideways-switch">
            <div class="setting-name"> 2 Teams</div>
            <label class="switch">
                <input type="checkbox" bind:checked={teams} on:change={update_players}>
                <span class="slider round"></span>
              </label>
        </div>
        <div class="random-champion-button" on:click={update_players}> Randomize </div>
    </div>
    <div class="team-container side-b"
        in:fly={{ delay: 300, duration: 300, x: 1200, y: 0, opacity: 0, easing: quintOut }}
    >
        {#each lanes as lane, i}
            <div class="player-wrapper" is-active="{i < team_size && teams == 1}">
                <div class="role-wrapper">
                    <img src={(team_size < 5) ? fill : lane_img[i]} alt="">
                </div>
                <div class="player-info-wrapper">
                    {#if i < team_size && teams == 1 && team_2[i] != undefined && team_2[i] != "A player"}
                        <h2 class="color-red">{team_2[i]}</h2>
                    {:else}
                        <h2>A player</h2>
                    {/if}
                    <h3>{ (team_size < 5) ? "Any" : lane}</h3>
                </div>
            </div>
        {/each}
    </div>
</div>