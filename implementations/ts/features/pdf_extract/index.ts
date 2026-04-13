import { readFile } from 'node:fs/promises'
import { PDFParse } from 'pdf-parse'

async function run(input: string) {
  const buffer = await readFile(input)
  const parser = new PDFParse({ data: buffer })

  const result = await parser.getText()
  // or use getRaw() for v1 compatibility
  console.log(result.text)
}

if (import.meta.main) {
  const input = process.argv[2]
  if (input) {
    console.log(`Parsing PDF from: ${input}`)
    await run(input)
  } else {
    console.log('No input URL provided')
  }
}
