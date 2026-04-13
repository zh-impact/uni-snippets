import assert from 'node:assert/strict'
import { describe, test } from 'node:test'
import { mergeRegion } from './merge-region.js'

describe('mergeRegion', () => {
  test('merges overlapping regions', () => {
    const regions = [
      { start: 1, end: 3 },
      { start: 2, end: 4 },
      { start: 5, end: 6 },
    ]
    const result = mergeRegion(regions)
    assert.deepStrictEqual(result, [
      { start: 1, end: 4 },
      { start: 5, end: 6 },
    ])
  })

  test('merges adjacent regions', () => {
    const regions = [
      { start: 1, end: 3 },
      { start: 3, end: 5 },
      { start: 6, end: 8 },
    ]
    const result = mergeRegion(regions)
    assert.deepStrictEqual(result, [
      { start: 1, end: 5 },
      { start: 6, end: 8 },
    ])
  })

  test('merges regions within gap limit', () => {
    const regions = [
      { start: 1, end: 2 },
      { start: 2.5, end: 3.5 },
      { start: 4, end: 5 },
    ]
    const result = mergeRegion(regions, { gap: 0.6 })
    assert.deepStrictEqual(result, [{ start: 1, end: 5 }])
  })

  test('keeps regions separate when gap is larger than specified', () => {
    const regions = [
      { start: 1, end: 2 },
      { start: 4, end: 5 },
    ]
    const result = mergeRegion(regions, { gap: 1 })
    assert.deepStrictEqual(result, [
      { start: 1, end: 2 },
      { start: 4, end: 5 },
    ])
  })

  test('handles empty input', () => {
    assert.deepStrictEqual(mergeRegion([]), [])
  })

  test('handles single region', () => {
    const regions = [{ start: 1, end: 2 }]
    assert.deepStrictEqual(mergeRegion(regions), [{ start: 1, end: 2 }])
  })

  test('merges region with gap limit (original test case)', () => {
    assert.deepStrictEqual(
      mergeRegion(
        [
          { start: 8.0, end: 10.3 },
          { start: 10.4, end: 10.9 },
          { start: 15.7, end: 17.5 },
          { start: 21.6, end: 23.6 },
          { start: 27.4, end: 29.0 },
        ],
        { gap: 0.5 },
      ),
      [
        { start: 8.0, end: 10.9 },
        { start: 15.7, end: 17.5 },
        { start: 21.6, end: 23.6 },
        { start: 27.4, end: 29.0 },
      ],
    )
  })

  test('merges multiple overlapping regions', () => {
    const regions = [
      { start: 1, end: 3 },
      { start: 2, end: 4 },
      { start: 3, end: 5 },
      { start: 6, end: 8 },
      { start: 7, end: 9 },
    ]
    const result = mergeRegion(regions)
    assert.deepStrictEqual(result, [
      { start: 1, end: 5 },
      { start: 6, end: 9 },
    ])
  })

  test('handles regions in reverse order', () => {
    const regions = [
      { start: 5, end: 6 },
      { start: 3, end: 4 },
      { start: 1, end: 2 },
    ]
    const result = mergeRegion(regions)
    assert.deepStrictEqual(result, [
      { start: 1, end: 2 },
      { start: 3, end: 4 },
      { start: 5, end: 6 },
    ])
  })
})
