import { transform } from 'kotone'
import { test, expect } from 'vitest'
import path from 'path'
import fs from 'fs'

test('transform', async () => {
  const excepected = 'こんにちは,世界,いらっしゃいませ,に,日本'
  const file = path.join(__dirname, './test.csv')
  const buffer = fs.readFileSync(file)
  const result = (await transform(buffer)).trim()
  expect(result).toEqual(excepected)
})
