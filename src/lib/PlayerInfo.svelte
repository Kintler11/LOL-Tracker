<script>

    export let player_data;
    
    const spell_lookup = (spell) => {
        // Map spells to api compatible names
        let spell_map = {
            "Flash": "SummonerFlash",
            "Hexflash": "SummonerFlash",
            "Ghost": "SummonerHaste",
            "Teleport": "SummonerTeleport",
            "Unleashed Teleport": "SummonerTeleport",
            "Heal": "SummonerHeal",
            "Ignite": "SummonerDot",
            "Smite": "SummonerSmite",
            "Primal Smite": "SummonerSmite",
            "Unleashed Smite": "SummonerSmite",
            "Flee": "SummonerCherryHold",
            "Mark": "SummonerSnowball",
            "Barrier": "SummonerBarrier",
            "Exhaust": "SummonerExhaust",
            "Clarity": "SummonerMana",
            "Cleanse": "SummonerBoost"
        };

        if(!spell_map[spell]){console.log("Spell not found:", spell);}

        return spell_map[spell] || "Summoner_UltBookPlaceholder"
    }

    
</script>
<div class="player-info-container">
    <div class="player-container">
        {#if (player_data.is_bot)}
            <div class="bot-tag">BOT</div>
        {/if}
        <div class="champion-thumbnail">
            <div class="champion-image">
                <img src="https://ddragon.leagueoflegends.com/cdn/13.24.1/img/{player_data.champion.image}" alt="">
            </div>
        </div>
        <div class="player-spells">
            <img class="player-spell" src="https://ddragon.leagueoflegends.com/cdn/13.24.1/img/spell/{spell_lookup(player_data.summoner_one)}.png" alt="">
            <img class="player-spell" src="https://ddragon.leagueoflegends.com/cdn/13.24.1/img/spell/{spell_lookup(player_data.summoner_two)}.png" alt="">
    
        </div>
        <div class="player-name">
            {(!player_data.is_bot) ? player_data.player_name : player_data.champion.name}
        </div>
        <div class="player-points">
            {player_data.score.deaths + player_data.stats.points}
        </div>
    </div>
    <div class="player-more-info">
        <div class="info-container">
            <div class="info-type">Structures</div>
            <div class="info">{player_data.stats.structures}</div>
        </div>
        <div class="info-container">
            <div class="info-type">Objectives</div>
            <div class="info">{player_data.stats.objectives}</div>
        </div>
        <div class="info-container">
            <div class="info-type">Pentas</div>
            <div class="info">{player_data.stats.pentas}</div>
        </div>
        <div class="info-container">
            <div class="info-type">First bloods</div>
            <div class="info">{player_data.stats.first_blood}</div>
        </div>
        <div class="info-container">
            <div class="info-type">K/D/A</div>
            <div class="info">{player_data.score.kills}/{player_data.score.deaths}/{player_data.score.assists}</div>
        </div>
    </div>
</div>
