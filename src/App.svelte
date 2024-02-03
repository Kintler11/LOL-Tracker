<script lang="ts">
  import LiveGame from './lib/LiveGame.svelte'
  import Champion from './lib/Champion.svelte'
  import Teams from './lib/Teams.svelte'
  import Players from './lib/Players.svelte'
  import { invoke } from "@tauri-apps/api/tauri"

  const updateInterval = 1000;

  
  const set_snapshot = async (snapshotType: String) => {await invoke("set_snapshot", {snapshotType})}
  const clear_snapshot = async () => {await invoke("clear_snapshot")}
  const update_game_data = async () => {await invoke("update");}
  const get_player_data = async () => {return JSON.parse(await invoke("get_all_players"));}
  const get_all_events = async () => {return JSON.parse(await invoke("get_all_events"));}
  
  let randomizer = {
    current_champion: {},
    current_lane: 0
  };

  let current_tab = 2;

  let players = []
  let team_1 = [];
  let team_2 = [];
  let player_list = ["Player 1", "Player 2", "Player 3", "Player 4"]

  let events = []
  let event_buffer = []

  const update_state = () => {
      update_game_data();
      get_player_data().then((data)=>{players = data;})
      get_all_events().then((data)=>{
          if (events.length > 0 && data.length != events.length){
              event_buffer = event_buffer.concat(data.slice(events.length, data.length));
          }
          events = data;
      })
  }

  setInterval(() => update_state(), updateInterval);

  // SNAPSHOT CONTROLS ( FOR TESTING )
  // Available snapshots: Normal, Aram, Arena, TFT
  // set_snapshot("classic")
  // clear_snapshot()
  update_state()


</script>

<main class="container">
  <div class="nav-bar">
    {#each ["Players", "Champions", "Teams", "Live"] as tab, i }
      <div class="nav-link" selected="{current_tab == i}" on:click={() => current_tab = i}>{tab}</div>
    {/each}
  </div>
  {#if current_tab == 3}
    <LiveGame bind:players={players} bind:events={events} bind:event_buffer={event_buffer} />
  {:else if current_tab ==  2}
    <Teams bind:players={player_list} bind:team_1={team_1} bind:team_2={team_2} />
  {:else if current_tab ==  1}
    <Champion bind:current_champion={randomizer.current_champion} bind:current_lane={randomizer.current_lane} />
  {:else if current_tab ==  0}
    <Players bind:players={player_list} />
  {/if}

</main>
