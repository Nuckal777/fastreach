<script lang="ts">
    export let onClose: () => void = () => {};
</script>

<div
    class="info-back fill"
    on:click={onClose}
    on:keydown={onClose}
    role="region"
>
    <div class="info-overlay" on:click|stopPropagation={() => {}} role="dialog">
        <input
            type="button"
            class="small-btn border"
            on:click={onClose}
            aria-label="close"
            value="x"
        />
        <h2>Welcome to Fastreach</h2>
    </div>
</div>

<style>
    .info-back {
        position: relative;
        z-index: 2;
        pointer-events: initial;
        background-color: rgba(0, 0, 0, 0.4);
    }

    .info-overlay {
        position: absolute;
        left: 50%;
        top: 50%;
        pointer-events: initial;

        transform: translate(-50%, -50%);
        background-color: rgba(255, 255, 255, 0.85);
        border-radius: 5px;
        margin: 10px;
        padding: 5px;
        width: fit-content;
        height: fit-content;
        border: 2px solid rgba(0, 0, 0, 0.2);
        max-height: calc(80vh - 105px);
        overflow: auto;
    }
</style>
