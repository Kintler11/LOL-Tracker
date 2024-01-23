<script>
    import { fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
    import { invoke } from "@tauri-apps/api/tauri"

    import fill from "./images/fill.png";
    import top from "./images/top.png";
    import jg from "./images/jungle.png";
    import mid from "./images/mid.png";
    import bot from "./images/bot.png";
    import sup from "./images/support.png";

    const lanes = ["Any", "Top", "Jungle", "Middle", "Bottom", "Support"];

    export let current_champion = {};
    export let current_lane = 0;

    let show_image = true;

    const get_champion = async () =>{
        show_image = false;
        let laneIndex = current_lane;
        current_champion = JSON.parse(await invoke("get_random_champion",{laneIndex}))
    }

    function set_champion (){
        get_champion().then(() => show_image = true);
    }
    
</script>

<div class="page-container" >
    <div class="lane-container" 
        in:fly={{ delay: 300, duration: 300, x: -400, y: 0, opacity: 0.2, easing: quintOut }}
        out:fly={{ delay: 0, duration: 300, x: 0, y: 800, opacity: 0.2, easing: quintOut }}
    >
        {#each [fill, top, jg, mid, bot, sup] as lane, i }
            <div class="lane" selected="{current_lane == i}" on:click={() => {current_lane = i}}>
                <img src={lane} alt="">
                <h2>{lanes[i]}</h2>
            </div>
        {/each}
    </div>
    <div class="randomizer"
        in:fly={{ delay: 300, duration: 300, x: 400, y: 0, opacity: 0, easing: quintOut }}
        out:fly={{ delay: 0, duration: 300, x: 0, y: 800, opacity: 0.2, easing: quintOut }}
    >
        <div class="random-champion-image">
            {#if current_champion.name == undefined}
                <div class="champion-placeholder">
                    Randomize
                </div>
            {:else if show_image }
                <img src="https://ddragon.leagueoflegends.com/cdn/13.24.1/img/{current_champion.image}" alt="{current_champion.name}"
                    in:scale={{ duration: 300, delay: 0, opacity: 0.5, start: 0.7, easing: quintOut }}
                >
                <h2>{current_champion.name}</h2>
            {/if}

        </div>
        <div class="random-champion-button" on:click={set_champion}> Randomize </div>
    </div>
</div>