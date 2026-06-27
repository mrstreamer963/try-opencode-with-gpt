import type { CoreEvent, SimulationCore, WorkerCommand } from './protocol';

export interface WorkerBridge {
  handle(command: WorkerCommand): Promise<CoreEvent>;
}

export function createWorkerBridge(loadCore: () => Promise<SimulationCore>): WorkerBridge {
  let corePromise: Promise<SimulationCore> | undefined;

  async function getCore() {
    corePromise ??= loadCore();
    return corePromise;
  }

  return {
    async handle(command) {
      const core = await getCore();

      switch (command.type) {
        case 'Start':
          return core.snapshot();
        case 'Tick':
          return core.tick();
        case 'BuildAt':
          return core.build_at(command.kind, command.x, command.y);
      }
    },
  };
}
