import { describe, expect, it } from 'vitest';
import { createWorkerBridge } from '../src/worker/bridge';
import type { CoreEvent, SimulationCore } from '../src/worker/protocol';

function createCore(): SimulationCore {
  const snapshot: CoreEvent = {
    type: 'StateSnapshot',
    snapshot: {
      map: { width: 30, height: 30, tiles: [] },
      units: [],
      buildings: [],
      construction_jobs: [],
    },
  };

  return {
    snapshot: () => snapshot,
    tick: () => snapshot,
    build_at: (kind: string, x: number, y: number) => {
      if (x === 0 || y === 0) {
        return { type: 'CommandRejected', reason: 'NotBuildable' };
      }
      return { type: 'ConstructionStarted', job_id: kind === 'food_bush' ? 1 : 2 };
    },
  };
}

describe('worker bridge', () => {
  it('returns an initial snapshot when started', async () => {
    const bridge = createWorkerBridge(async () => createCore());

    const event = await bridge.handle({ type: 'Start' });

    expect(event.type).toBe('StateSnapshot');
  });

  it('advances ticks through the core', async () => {
    const bridge = createWorkerBridge(async () => createCore());
    await bridge.handle({ type: 'Start' });

    const event = await bridge.handle({ type: 'Tick' });

    expect(event.type).toBe('StateSnapshot');
  });

  it('forwards accepted and rejected build commands', async () => {
    const bridge = createWorkerBridge(async () => createCore());
    await bridge.handle({ type: 'Start' });

    await expect(
      bridge.handle({ type: 'BuildAt', kind: 'food_bush', x: 4, y: 4 }),
    ).resolves.toEqual({ type: 'ConstructionStarted', job_id: 1 });
    await expect(bridge.handle({ type: 'BuildAt', kind: 'bed', x: 0, y: 0 })).resolves.toEqual({
      type: 'CommandRejected',
      reason: 'NotBuildable',
    });
  });
});
