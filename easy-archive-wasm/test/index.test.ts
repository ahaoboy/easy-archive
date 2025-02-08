import { readdirSync, readFileSync } from 'fs'
import { join } from 'path'
import { expect, test } from 'vitest'
import { decode } from '../src-ts'

const assetsDir = "../assets"
const distKey = 'mujs-build-0.0.11/dist-manifest.json'
test("decode", () => {
  for (const name of readdirSync(assetsDir)) {
    const p = join(assetsDir, name)
    const buffer = readFileSync(p)
    const files = decode(name, buffer)!
    expect(files.keys().includes(distKey))
    const jsonBuf = files.get(distKey)?.get_buffer()!
    const str = Buffer.from(jsonBuf).toString()
    const json = JSON.parse(str)
    expect(json['artifacts']['mujs-x86_64-unknown-linux-gnu.tar.xz']['name']).toEqual("mujs-x86_64-unknown-linux-gnu.tar.xz")
  }
})