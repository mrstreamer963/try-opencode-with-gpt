import { describe, expect, it } from 'vitest';
import {
  canReplaceHeldFeedback,
  createBuildCommand,
  formatNeed,
  rejectionMessage,
} from '../src/game/viewModel';

describe('game view model', () => {
  it('creates build commands from selected build mode and tile coordinates', () => {
    expect(createBuildCommand('food_bush', { x: 4, y: 5 })).toEqual({
      type: 'BuildAt',
      kind: 'food_bush',
      x: 4,
      y: 5,
    });
  });

  it('formats need values as whole-number percentages', () => {
    expect(formatNeed(83.8)).toBe('84%');
    expect(formatNeed(-2)).toBe('0%');
    expect(formatNeed(130)).toBe('100%');
  });

  it('maps command rejections to visible feedback', () => {
    expect(rejectionMessage('OutOfBounds')).toBe('Tile is outside the map.');
    expect(rejectionMessage('NotBuildable')).toBe('This terrain cannot be built on.');
    expect(rejectionMessage('Occupied')).toBe('This tile is already occupied.');
  });

  it('keeps rejection feedback visible until the hold expires', () => {
    expect(canReplaceHeldFeedback(1_000, 1_500)).toBe(false);
    expect(canReplaceHeldFeedback(1_500, 1_500)).toBe(true);
  });
});
