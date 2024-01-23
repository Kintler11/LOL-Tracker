<script>

    import { onMount } from "svelte";
    import { fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
    export let event = {}

    let point_count = 0;
    let popup_header = "";
    let popup_subject = "";
    let subject_team = "";
    let popup_description = "";

    let show_popup = false;
    onMount(async () => {
        show_popup = true;
        console.log(event)

        if(event.type == null){
            show_popup = false
            event = {}
        }

        switch (event.type){
            case "NexusTurret":
                point_count = 1;
                popup_header = "Nexus Turret Taken";
                subject_team = event.team;
                popup_subject = `${event.team}-side`;
                popup_description = `has lost a Nexus turret.`;
                break;
            case "Turret":
                point_count = 1;
                popup_header = "Turret Taken";
                subject_team = event.team;
                popup_subject = `${event.team}-side`;
                popup_description = `has lost a ${event.lane} turret.`;
                break;
            case "Inhibitor":
                point_count = 1;
                popup_header = "Inhibitor Taken";
                subject_team = event.team;
                popup_subject = `${event.team}-side`;
                popup_description = `has lost a ${event.lane} Inhibitor.`;
                break;
            case "Dragon":
                point_count = 1;
                popup_header = "Dragon Taken";
                subject_team = event.team;
                popup_subject = `${event.team}-side`;
                popup_description = `has taken a dragon.`;
                break;
            case "Elder":
                point_count = 2;
                popup_header = "Elder Taken";
                subject_team = event.team;
                popup_subject = `${event.team}-side`;
                if(event.stolen){
                    popup_description = `has stolen the Elder Dragon.`;
                }else{
                    popup_description = `has taken the Elder Dragon.`;
                }
                break;
            case "Herald":
                point_count = 1;
                popup_header = `Herald`;
                subject_team = event.team;
                popup_subject = `${event.team}`;
                popup_description = `has taken Herald.`;
                break;
            case "Baron":
                point_count = 1;
                popup_header = `Baron`;
                subject_team = event.team;
                popup_subject = `${event.team}`;
                popup_description = `has taken Baron.`;
                break;
            case "Penta":
                point_count = 5;
                popup_header = `PENTA!`;
                subject_team = event.team;
                popup_subject = `${event.player}`;
                popup_description = `has gotten a PENTA!.`;
                break;
            case "FirstBlood":
                point_count = 2;
                popup_header = `First Blood`;
                subject_team = (event.team == "Red") ? "Blue" : "Red";
                popup_subject = `${event.player}`;
                popup_description = `is First Blood.`;
                break;
        }


        setTimeout(() =>{
            show_popup = false
            event = {}
        }, 5000);
        
    });
</script>
{#if show_popup}
    <div class="popup" team="{subject_team}" transition:fly={{ delay: 0, duration: 500, x: 600, y: 0, opacity: 0.2, easing: quintOut }}>
        <div class="popup-icon">+{point_count}</div>
        <div class="popup-info">
            <div class="popup-header">{popup_header}</div>
            <div class="popup-description"><div class="popup-subject">{popup_subject}</div>{popup_description}</div>
        </div>
    </div>
{/if}