import { createWorkerBridge } from './bridge';
import initWasm, { WasmSimulation } from '../wasm/idle_core/idle_core.js';
import type { SimulationCore, WorkerCommand } from './protocol';

async function loadWasmCore(): Promise<SimulationCore> {
  await initWasm();
  return new WasmSimulation() as SimulationCore;
}

const bridge = createWorkerBridge(loadWasmCore);

self.onmessage = async (message: MessageEvent<WorkerCommand>) => {
  try {
    self.postMessage(await bridge.handle(message.data));
  } catch (error) {
    self.postMessage({
      type: 'CommandRejected',
      reason: error instanceof Error ? error.message : 'WorkerError',
    });
  }
};
