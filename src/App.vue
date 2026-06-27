<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, shallowRef } from 'vue';
import { createPixiMapRenderer, type PixiMapRenderer } from './game/pixiMap';
import {
  canReplaceHeldFeedback,
  createBuildCommand,
  formatNeed,
  rejectionMessage,
} from './game/viewModel';
import type { BuildCommandKind, CoreEvent, StateSnapshot, WorkerCommand } from './worker/protocol';

const mapHost = ref<HTMLElement | null>(null);
const renderer = shallowRef<PixiMapRenderer | null>(null);
const snapshot = ref<StateSnapshot | null>(null);
const selectedBuild = ref<BuildCommandKind>('food_bush');
const feedback = ref('Starting simulation worker...');

let worker: Worker | undefined;
let tickTimer: number | undefined;
let feedbackHoldUntil = 0;

const unitCount = computed(() => snapshot.value?.units.length ?? 0);

function send(command: WorkerCommand) {
  worker?.postMessage(command);
}

function handleTileSelected(x: number, y: number) {
  send(createBuildCommand(selectedBuild.value, { x, y }));
}

function handleCoreEvent(event: CoreEvent) {
  if (event.type === 'StateSnapshot') {
    snapshot.value = event.snapshot;
    renderer.value?.render(event.snapshot);
    if (canReplaceHeldFeedback(Date.now(), feedbackHoldUntil)) {
      feedback.value = `Simulation running: ${event.snapshot.construction_jobs.length} active builds.`;
    }
    return;
  }

  if (event.type === 'ConstructionStarted') {
    feedback.value = `Construction job #${event.job_id} started.`;
    return;
  }

  feedbackHoldUntil = Date.now() + 1_500;
  feedback.value = rejectionMessage(event.reason);
}

onMounted(async () => {
  if (!mapHost.value) {
    return;
  }

  renderer.value = await createPixiMapRenderer(mapHost.value, handleTileSelected);
  worker = new Worker(new URL('./worker/simulation.worker.ts', import.meta.url), { type: 'module' });
  worker.onmessage = (message: MessageEvent<CoreEvent>) => handleCoreEvent(message.data);
  worker.onerror = () => {
    feedback.value = 'Simulation worker failed. Build the Wasm core with npm run wasm:build.';
  };

  send({ type: 'Start' });
  tickTimer = window.setInterval(() => send({ type: 'Tick' }), 500);
});

onUnmounted(() => {
  if (tickTimer !== undefined) {
    window.clearInterval(tickTimer);
  }
  worker?.terminate();
  renderer.value?.destroy();
});
</script>

<template>
  <main class="shell">
    <section class="hero">
      <p class="eyebrow">Idle Colony MVP</p>
      <h1>Three stranded units, one tiny grid, no mercy from sleep debt.</h1>
      <p class="copy">
        Pick a build mode, then click or tap a tile. Units complete food bushes and beds from the
        Rust ECS simulation running through the worker boundary.
      </p>
    </section>

    <section class="game-layout" aria-label="Game area">
      <div class="map-panel">
        <div ref="mapHost" class="map-host" aria-label="Tile map"></div>
      </div>

      <aside class="side-panel">
        <section class="card">
          <h2>Build</h2>
          <div class="build-buttons" aria-label="Build mode">
            <button
              :class="{ active: selectedBuild === 'food_bush' }"
              type="button"
              @click="selectedBuild = 'food_bush'"
            >
              Food bush
            </button>
            <button
              :class="{ active: selectedBuild === 'bed' }"
              type="button"
              @click="selectedBuild = 'bed'"
            >
              Bed
            </button>
          </div>
          <p class="hint">Selected: {{ selectedBuild === 'food_bush' ? 'Food bush' : 'Bed' }}</p>
        </section>

        <section class="card">
          <h2>Units ({{ unitCount }})</h2>
          <div v-if="snapshot" class="units">
            <article v-for="unit in snapshot.units" :key="unit.id" class="unit-card">
              <strong>Unit {{ unit.id }}</strong>
              <span>Food {{ formatNeed(unit.food) }}</span>
              <span>Sleep {{ formatNeed(unit.sleep) }}</span>
            </article>
          </div>
          <p v-else class="hint">Waiting for first snapshot...</p>
        </section>

        <section class="card status" aria-live="polite">
          <h2>Status</h2>
          <p>{{ feedback }}</p>
        </section>
      </aside>
    </section>
  </main>
</template>
