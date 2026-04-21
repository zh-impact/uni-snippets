import fs from 'node:fs/promises'
import { Readability } from '@mozilla/readability'
import { JSDOM } from 'jsdom'
import ky from 'ky'

const url = 'https://dev.to/devteam/top-7-featured-dev-posts-of-the-week-555a'

const text = await ky.get(url).text()

fs.writeFile('output.html', text)

const doc = new JSDOM(text, { url })

const reader = new Readability(doc.window.document)
const article = reader.parse()

console.log(Object.keys(article ?? {}))

fs.writeFile('output.txt', article?.content || '')
