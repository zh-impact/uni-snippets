import fs from 'node:fs/promises'
import { validate } from '@hyperjump/json-schema/openapi-3-1'

const openapiSpecPath = new URL('./openapi.json', import.meta.url)

async function main() {
  const specContent = await fs.readFile(openapiSpecPath, 'utf-8')
  const spec = JSON.parse(specContent)

  const validationResult = await validate('https://spec.openapis.org/oas/3.1/schema', spec)
  if (validationResult.valid) {
    console.log('OpenAPI specification is valid.')
  } else {
    console.error('OpenAPI specification is invalid:')
    console.error(validationResult.errors)
  }
}

main().catch((error) => {
  console.error('An error occurred:', error)
})
